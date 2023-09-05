use std::net::{Ipv4Addr, SocketAddr};

pub fn of(port:u16) -> SocketAddr{
    return SocketAddr::new(
        std::net::IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), port
    );

}