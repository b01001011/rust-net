#[cfg(feature = "log")]
use crate::log::{info};

use crate::discovery::{
    error::{e_fmt, Error, Result},
};
use mio::{event::Source, net::UdpSocket as MioUdpSocket, Interest, Poll, Registry, Token};
use std::{
    net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6, UdpSocket},
};

pub const MDNS_PORT: u16 = 5353;

const GROUP_ADDR_V4: Ipv4Addr = Ipv4Addr::new(224, 0, 0, 251);
const GROUP_ADDR_V6: Ipv6Addr = Ipv6Addr::new(0xff02, 0, 0, 0, 0, 0, 0, 0xfb);
const LOOPBACK_V4: Ipv4Addr = Ipv4Addr::new(127, 0, 0, 1);

pub struct DiscoveryService {
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

        let poller = Poll::new().map_err(|e| e_fmt!("failed to create mio Poll: {e}"))?;

        Ok(Self{})
    }
}

