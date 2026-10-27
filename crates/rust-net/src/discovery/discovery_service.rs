#[cfg(feature = "log")]
use crate::log::{debug, trace};
use crate::discovery::error::{e_fmt, Error, Result};
use crate::discovery::utils::{local_if_addrs};
use flume::{bounded, Receiver, Sender, TrySendError};
use mio::{event::Source, net::UdpSocket as MioUdpSocket, Interest, Poll, Registry, Token, Waker};
use socket2::Domain;
use socket_pktinfo::PktInfoUdpSocket;
use std::{
    collections::{HashMap},
    io,
    net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6, UdpSocket},
    sync::Arc,
    thread,
};
use crate::discovery::types::{LocalInterface, DnsRegistry};

pub const MDNS_PORT: u16 = 5353;

const GROUP_ADDR_V4: Ipv4Addr = Ipv4Addr::new(224, 0, 0, 251);
const GROUP_ADDR_V6: Ipv6Addr = Ipv6Addr::new(0xff02, 0, 0, 0, 0, 0, 0, 0xfb);
const LOOPBACK_V4: Ipv4Addr = Ipv4Addr::new(127, 0, 0, 1);
const EVENT_CHANNEL_CAPACITY: usize = 1000;

const WAKER_EVENT_KEY: usize = usize::MAX - 1;

pub struct DiscoveryService {
    sender: Sender<Command>,
    waker: Arc<Waker>,
}

impl DiscoveryService {
    pub fn new() -> Result<Self> {
        Self::new_with_port(MDNS_PORT)
    }

    pub fn new_with_port(port: u16) -> Result<Self> {
        let poll = Poll::new().map_err(|e| e_fmt!("failed to create mio Poll: {e}"))?;
        let waker = Arc::new(
            Waker::new(poll.registry(), Token(WAKER_EVENT_KEY))
                .map_err(|e| e_fmt!("failed to create mio Waker: {e}"))?,
        );
        let (sender, receiver) = bounded(100);
        
        let thread_waker = waker.clone();
        let cmd_sender = sender.clone();

        thread::Builder::new()
            .name("discovery_service".to_string())
            .spawn(move || {
                Self::service_thread(thread_waker, poll, receiver, port, cmd_sender)
            })
            .map_err(|e| e_fmt!("thread builder failed to spawn: {}", e))?;
        
        Ok(Self{
            sender,
            waker,
       })
    }

    pub fn browse(&self, service_name: &str) -> Result<Receiver<DiscoveryEvent>> {
        let (sender, receiver) = bounded(EVENT_CHANNEL_CAPACITY);
        Ok(receiver)
    }

    fn send_cmd(&self, cmd: Command) -> Result<()> {
        self.sender.try_send(cmd).map_err(|e| match e {
            TrySendError::Full(_) => Error::Again,
            TrySendError::Disconnected(_) => Error::ServiceShutdown,
        })?;

        self.waker.wake().map_err(|e| e_fmt!("failed to wake daemon: {e}"))?;
        
        Ok(())
    }

    fn service_thread(
        waker: Arc<Waker>,
        poll: Poll,
        receiver: Receiver<Command>,
        port: u16, 
        cmd_sender: Sender<Command>,
    ) {
        let mut zc = Zeroconf::new(waker, poll, port, cmd_sender);
        
        if let Some(cmd) = zc.run(receiver) {
            match cmd {
                Command::Exit => {
                }
                _ => {
                    debug!("unexpected command: {:?}", cmd);
                }
            }
        }
    }
}

struct CustomUdpSocket {
    pktinfo: PktInfoUdpSocket,
    mio: MioUdpSocket,
}

impl CustomUdpSocket {
    pub fn new(pktinfo: PktInfoUdpSocket) -> io::Result<Self> {
        let std_sock = pktinfo.try_clone_std()?;
        let mio = MioUdpSocket::from_std(std_sock);
        Ok(Self { pktinfo, mio })
    }
}

struct Zeroconf {
    dns_registry_map: HashMap<u32, DnsRegistry>,
    intf_map: HashMap<u32, LocalInterface>,
    poll: Poll,
    port: u16,
    waker: Arc<Waker>,
}

impl Zeroconf {
    fn new(
        waker: Arc<Waker>,
        poll: Poll,
        port: u16, 
        cmd_sender: Sender<Command>,
    ) -> Self {
        let addrs = local_if_addrs();

        let mut intf_map: HashMap<u32, LocalInterface> = HashMap::new();
        let mut dns_registry_map: HashMap<u32, DnsRegistry> = HashMap::new();

        let addr = SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), port);
        let ipv4_sock = new_multicast_socket(addr.into());

        println!("{addr}");

        Self{
            dns_registry_map,
            intf_map,
            poll,
            port,
            waker,
        }
    }

    fn run(&mut self, receiver: Receiver<Command>) -> Option<Command> {
        return None;
    }
}

#[derive(Debug)]
enum Command {
    Browse(String, u32, bool, Sender<DiscoveryEvent>),
    Exit,
}

#[derive(Debug)]
pub enum DiscoveryEvent {
    SearchStarted(String),
}

pub enum ServiceStatus {
    Running, 
    Shutdown,
}

fn new_multicast_socket(addr: SocketAddr) -> Result<CustomUdpSocket> {
    let sock = new_socket(addr, true)?;

    let ttl_result = match addr {
        SocketAddr::V4(_) => sock.set_multicast_ttl_v4(255),
        SocketAddr::V6(_) => sock.set_multicast_hops_v6(255),
    };

    if let Err(e) = ttl_result {
        debug!("failed to set multicast TTL on {addr}: {e}");
    }

    CustomUdpSocket::new(sock)
        .map_err(|e| e_fmt!("failed to create CustomUdpSocket for {}: {}", addr, e)) 
}

fn new_socket(addr: SocketAddr, non_block: bool) -> Result<PktInfoUdpSocket> {
    let domain = match addr {
        SocketAddr::V4(_) => socket2::Domain::IPV4,
        SocketAddr::V6(_) => socket2::Domain::IPV6,
    };

    let fd = PktInfoUdpSocket::new(domain).map_err(|e| e_fmt!("create socket failed: {}", e))?;

    fd.set_reuse_address(true).map_err(|e| e_fmt!("set ReuseAddr failed: {}", e))?;

    if non_block {
        fd.set_nonblocking(true)
            .map_err(|e| e_fmt!("set O_NONBLOCK: {}", e))?;
    }

    fd.bind(&addr.into())
        .map_err(|e| e_fmt!("socket bind to {} failed: {}", &addr, e))?;

    trace!("new socket bind to {}", &addr);
    Ok(fd)
}

