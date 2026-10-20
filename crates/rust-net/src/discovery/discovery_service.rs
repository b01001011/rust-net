#[cfg(feature = "log")]
use crate::log::{debug};

use crate::discovery::{
    error::{e_fmt, Error, Result},
};
use flume::{bounded, Receiver, Sender};
use mio::{event::Source, net::UdpSocket as MioUdpSocket, Interest, Poll, Registry, Token};
use std::{
    net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6, UdpSocket},
    thread,
};

pub const MDNS_PORT: u16 = 5353;

const GROUP_ADDR_V4: Ipv4Addr = Ipv4Addr::new(224, 0, 0, 251);
const GROUP_ADDR_V6: Ipv6Addr = Ipv6Addr::new(0xff02, 0, 0, 0, 0, 0, 0, 0xfb);
const LOOPBACK_V4: Ipv4Addr = Ipv4Addr::new(127, 0, 0, 1);

pub struct DiscoveryService {
    sender: Sender<Command>,
    signal_addr: SocketAddr,
}

impl DiscoveryService {
    pub fn new() -> Result<Self> {
        Self::new_with_port(MDNS_PORT)
    }

    pub fn new_with_port(port: u16) -> Result<Self> {
        let signal_addr = SocketAddrV4::new(LOOPBACK_V4, 0);
        
        let signal_sock = UdpSocket::bind(signal_addr)
            .map_err(|e| e_fmt!("failed to create signal_sock for service: {}", e))?;

        let signal_addr = signal_sock
            .local_addr()
            .map_err(|e| e_fmt!("failed to get singal sock addr: {}", e))?;

        signal_sock.set_nonblocking(true)
            .map_err(|e| e_fmt!("failed to set nonblocking for singal socket: {}", e))?;

        let poll = Poll::new().map_err(|e| e_fmt!("failed to create mio Poll: {e}"))?;
        let (sender, receiver) = bounded(100);
        let mio_sock = MioUdpSocket::from_std(signal_sock);
        let cmd_sender = sender.clone();
        thread::Builder::new()
            .name("discovery_service".to_string())
            .spawn(move || {
                Self::service_thread(mio_sock, poll, receiver, port, cmd_sender, signal_addr)
            })
            .map_err(|e| e_fmt!("thread builder failed to spawn: {}", e))?;
        
        Ok(Self{
            sender,
            signal_addr,
        })
    }

    fn service_thread(
        signal_sock: MioUdpSocket,
        poll: Poll,
        receiver: Receiver<Command>,
        port: u16, 
        cmd_sender: Sender<Command>,
        signal_addr: SocketAddr,
    ) {
        let mut zc = Zeroconf::new(signal_sock, poll, port, cmd_sender, signal_addr);

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

struct Zeroconf {
    poll: Poll,
    port: u16,
}

impl Zeroconf {
    fn new(
        signal_sock: MioUdpSocket,
        poll: Poll,
        port: u16, 
        cmd_sender: Sender<Command>,
        signal_addr: SocketAddr,
    ) -> Self {

        Self{
            poll,
            port,
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

pub enum DiscoveryEvent {
    SearchStarted(String),
}

pub enum ServiceStatus {
    Running, 
    Shutdown,
}

