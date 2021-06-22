use shadowsocks::context::Context;
use shadowsocks::config::ServerType;
use shadowsocks::{ProxySocket, ServerConfig};

use std::sync::Arc;

use std::net::{SocketAddr, UdpSocket, IpAddr, Ipv4Addr};
use shadowsocks::relay::Address;
use shadowsocks::crypto::v1::CipherKind;
use shadowsocks::relay::socks5::Address::DomainNameAddress;
use simple_dns::Packet;

pub  async fn send(dst: SocketAddr, pkt: &[u8]) {
    println!("Staring DNS Query over SS");
    let ss_server = Address::SocketAddress(SocketAddr::new(IpAddr::V4(Ipv4Addr::new(1, 1, 1,1)), 3345));
    let password = "";
    let method = CipherKind::AES_128_GCM;
    let cfg = ServerConfig::new(ss_server, password, method);

    let ctx = Context::new_shared(ServerType::Local);

    let ss_socket = ProxySocket::connect(ctx, &cfg).await.unwrap();

    let dst = Address::SocketAddress(SocketAddr::new(IpAddr::V4(Ipv4Addr::new(8, 8, 8, 8)), 53));
    ss_socket.send(&dst, &pkt).await.unwrap();

    let mut buf = [0u8; 1024];
    //let dst = Address::SocketAddress(SocketAddr::new(IpAddr::V4(Ipv4Addr::new(114, 114, 114, 114)), 53));

    println!("data send!!");
    let (ret_n, addr, n) = ss_socket.recv(&mut buf).await.unwrap();
    // println!("ret {}， {}, {}", ret_n, addr, n);
    // println!("ret_n: {:?}", buf[..ret_n].iter().map(|b| format!(" {:02X}", b)).collect::<String>());
    println!("n: {:?}", buf[..n].iter().map(|b| format!(" {:02X}", b)).collect::<String>());
    let packet = Packet::parse(&buf[..n]).unwrap();
    println!("{:?}", packet);

    println!("DNS Query Done");
}