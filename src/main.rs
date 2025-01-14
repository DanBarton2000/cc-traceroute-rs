use std::net::{SocketAddr, ToSocketAddrs};
use std::io;
use clap::Parser;

fn main() -> io::Result<()> {
    let args = Args::parse();

    let max_hops = 64;
    let packet_size = 32;
    let addr = (args.hostname.clone(), 0).to_socket_addrs()?
        .next()
        .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "No address found"))?;

    let ipv4 = (args.hostname.clone(), 0).to_socket_addrs()?
        .find(|addr| matches!(addr, SocketAddr::V4(_)));

    let ip_str: String = if let Some(ip) = ipv4 {
        ip.ip().to_string()
    } else {
        addr.ip().to_string()
    };

    println!("traceroute to {} ({}), {} hops max, {} byte packets", args.hostname, ip_str, max_hops, packet_size);

    Ok(())
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    hostname: String
}