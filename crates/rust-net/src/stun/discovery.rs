use std::io;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr};
use std::time::Duration;

use tokio::net::UdpSocket;
use tokio::time::timeout;

const DEFAULT_STUN_SERVER: &str = "stun.l.google.com:19302";

const STUN_MAGIC_COOKIE: u32 = 0x2112_A442;
const STUN_BINDING_REQUEST: u16 = 0x0001;
const STUN_BINDING_SUCCESS: u16 = 0x0101;

const ATTR_MAPPED_ADDRESS: u16 = 0x0001;
const ATTR_XOR_MAPPED_ADDRESS: u16 = 0x0020;

pub async fn discover_public_address() -> io::Result<SocketAddr> {
    let server_addr = tokio::net::lookup_host(DEFAULT_STUN_SERVER)
        .await?
        .next()
        .ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::NotFound, 
                format!("Could not resolve STUN server: {DEFAULT_STUN_SERVER}"),
            )
        })?;

    let bind_addr = match server_addr {
        SocketAddr::V4(_) => SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), 0),
        SocketAddr::V6(_) => SocketAddr::new(IpAddr::V6(Ipv6Addr::UNSPECIFIED), 0),
    };

    let socket = UdpSocket::bind(bind_addr).await?;
    socket.connect(server_addr).await?;

    let tid: [u8; 12] = rand::random();

    let mut request = [0u8; 20];
    request[0..2].copy_from_slice(&STUN_BINDING_REQUEST.to_be_bytes());
    request[2..4].copy_from_slice(&0u16.to_be_bytes());
    request[4..8].copy_from_slice(&STUN_MAGIC_COOKIE.to_be_bytes());
    request[8..20].copy_from_slice(&tid);

    socket.send(&request).await?;

    let mut response = [0u8; 1500];

    let received: Result<usize, io::Error> = timeout(
        Duration::from_secs(5), 
        socket.recv(&mut response),
    )
    .await
    .map_err(|_| {
        io::Error::new(
            io::ErrorKind::TimedOut,
            "STUN request timed out",
        )
    })?;

    let received = received?;

    parse(&response[..received], &tid)
}

fn parse(data: &[u8], tid: &[u8; 12]) -> io::Result<SocketAddr> {
    if data.len() < 20 {
        return Err(io::Error::new(io::ErrorKind::InvalidData, "STUN response is too short."));
    }

    let message_type = u16::from_be_bytes([data[0], data[1]]);
    if message_type != STUN_BINDING_SUCCESS {
        return Err(io::Error::new(io::ErrorKind::InvalidData, "Unexpected STUN message type. 0x{message_type:04x}"));
    }

    let message_length = u16::from_be_bytes([data[2], data[3]]) as usize;
    if data.len() < 20 + message_length {
        return Err(io::Error::new(io::ErrorKind::InvalidData, "Truncated Stun response."));
    }

    let magic_cookie = u32::from_be_bytes([data[4], data[5], data[6], data[7]]);
    if magic_cookie != STUN_MAGIC_COOKIE {
        return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid STUN magic cookie."));
    }

    if data[8..20] != tid[..] {
        return Err(io::Error::new(io::ErrorKind::InvalidData, "STUN transaction ID mismatch."));
    }

    let attributes = &data[20..20 + message_length];
    let mut offset = 0;

    while offset + 4 <= attributes.len() {
        let attr_type = u16::from_be_bytes([attributes[offset], attributes[offset + 1]]);
        let attr_length = u16::from_be_bytes([attributes[offset + 2], attributes[offset + 3]]) as usize;

        let value_start = offset + 4;
        let value_end = value_start + attr_length;
        if value_end > attributes.len() {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "Truncated STUN attribute."));
        }

        let value = &attributes[value_start..value_end];

        match attr_type {
            ATTR_MAPPED_ADDRESS => {
                if let Some(addr) = parse_mapped_address(value) {
                    return Ok(addr);
                }
            }
            ATTR_XOR_MAPPED_ADDRESS => {
                if let Some(addr) = parse_xor_mapped_address(value) {
                    return Ok(addr);
                }
            }
            _ => {}
        }
        offset = value_end.next_multiple_of(4);
    }

    return Err(io::Error::new(
        io::ErrorKind::InvalidData, 
        "Failed to figure out public address."));
}

fn parse_mapped_address(value: &[u8]) -> Option<SocketAddr> {
    if value.len() < 8 {
        return None;
    }

    let family = value[1];
    let port = u16::from_be_bytes([value[2], value[3]]);
    
    match family {
        0x01 => {
            let ip = Ipv4Addr::new(
                value[4],
                value[5],
                value[6],
                value[7],
            );

            Some(SocketAddr::new(IpAddr::V4(ip), port))
        }

        0x02 => {
            if value.len() < 20 {
                return None;
            }

            let mut ip = [0u8; 16];
            ip.copy_from_slice(&value[4..20]);

            Some(SocketAddr::new(
                IpAddr::V6(Ipv6Addr::from(ip)),
                port,
            ))
        }

        _ => None,    
    }
}

fn parse_xor_mapped_address(value: &[u8]) -> Option<SocketAddr> {
    if value.len() < 8 {
        return None;
    }

    let family = value[1];
    let port = u16::from_be_bytes([value[2], value[3]]) ^ (STUN_MAGIC_COOKIE >> 16) as u16;
    
    match family {
        0x01 => {
            let encoded = u32::from_be_bytes([
                value[4],
                value[5],
                value[6],
                value[7],
            ]);

            let ip = encoded ^ STUN_MAGIC_COOKIE;

            Some(SocketAddr::new(
                IpAddr::V4(Ipv4Addr::from(ip)),
                port,
            ))
        }
        0x02 => {
            if value.len() < 20 {
                return None;
            }

            let mut ip = [0u8; 16];

            let cookie = STUN_MAGIC_COOKIE.to_be_bytes();
            for i in 0..16 {
                let mask = if i < 4 {
                    cookie[i]
                } else {
                    0
                };

                ip[i] = value[4 + i] ^ mask;
            }

            Some(SocketAddr::new(
                IpAddr::V6(Ipv6Addr::from(ip)),
                port,
            ))
        }

        _ => None
    }
}
