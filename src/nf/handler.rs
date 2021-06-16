use crate::nf::def::*;
use crate::nf::capi::*;

use std::os::raw::*;


pub extern fn nf_thread_start() {
    println!("threadStart();");
}

pub extern fn nf_thread_end() {
    println!("threadEnd();");
}

pub extern fn nf_tcp_connect_request(id: c_longlong, conn_info: NFTcpConnInfo) {
    println!("tcp->ConnectRequest();");
}

pub extern fn nf_tcp_connected(id: c_longlong, conn_info: NFTcpConnInfo) {
    println!("tcp->Connected();");
}

pub extern fn nf_tcp_closed(id: c_longlong, conn_info: NFTcpConnInfo) {
    println!("tcp->Closed();");
}

pub unsafe extern fn nf_tcp_receive(id: EndpointId, buf: *const c_char, len: c_int) {
    println!("tcp->receive();");
    nf_tcpPostReceive(id, buf, len);
}

pub unsafe extern fn nf_tcp_send(id: EndpointId, buf: *const c_char, len: c_int) {
    println!("tcp->send();");
    nf_tcpPostSend(id, buf, len);
}

pub extern fn nf_tcp_can_receive(id: c_longlong) {
    println!("tcp->can_receive();");
}

pub extern fn nf_tcp_can_send(id: c_longlong) {
    println!("tcp->can_send();");
}

pub unsafe extern fn nf_udp_created(id: c_longlong, conn_info: *const NFUdpConnInfo) {
    // let local_addr: SOCKADDR = (*conn_info).local_address;
    // if i32::from(local_addr.sa_family) == AF_INET {
    //     println!("-------------------");
    //     let data = local_addr.sa_data.as_ptr() as *const u8;
    //     let data_arr = (*slice_from_raw_parts(data, 14)).to_vec();
    //     let port_bytes = [*data.offset(0), *data.offset(1)];
    //     let port = u16::from_le_bytes(port_bytes);
    //     let ip = SocketAddrV4::new(
    //         Ipv4Addr::new(
    //             *data.offset(1), *data.offset(3), *data.offset(4), *data.offset(5))
    //         , port);
    //
    //     println!("udp->created() id= {}, process_id={} local_ip= {} local_address={}",
    //              id,
    //              (*conn_info).process_id,
    //              ip.to_string(),
    //              port);
    // }
}

pub extern fn nf_udp_connect_request(id: c_longlong, conn_info: NFUdpConnInfo) {
    println!("udp->connect_request();");
}

pub extern fn nf_udp_closed(id: c_longlong, conn_info: NFUdpConnInfo) {
    println!("udp->close();");
}

pub unsafe extern fn nf_udp_receive(id: EndpointId, remote_address: *const c_uchar, buf: *const c_char, len: c_int, options: &NFUdpOptions) {
    println!("udp->receive();");
    nf_udpPostReceive(id, remote_address, buf, len, options);
}

pub unsafe extern fn nf_udp_send(id: EndpointId, remote_address: *const c_uchar, buf: *const c_char, len: c_int, options: &NFUdpOptions) {
    println!("udp->send: id={}", id);
    nf_udpPostSend(id, remote_address, buf, len, options);
}

pub extern fn nf_udp_can_receive(id: c_longlong) {
    println!("udp->can_receive();");
}

pub extern fn nf_udp_can_send(id: c_longlong) {
    println!("udp->can_send();");
}
