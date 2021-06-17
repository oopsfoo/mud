use crate::nf::def::*;
use crate::nf::capi::*;
use crate::nf::util::*;

use winapi::shared::ws2def::SOCKADDR;

pub extern fn nf_thread_start() {
    info!("threadStart();");
}

pub extern fn nf_thread_end() {
    info!("threadEnd();");
}

pub unsafe extern fn nf_tcp_connect_request(id: u64, conn_info: &NFTcpConnInfo) {
    trace!("nf_tcp_connect_request called [Endpoint={}]", id);

    const BUF_SIZE: usize = 1024;

    let process_id = conn_info.process_id;
    let mut buf = [0u16; BUF_SIZE];
    let len = BUF_SIZE as u32;

    nf_getProcessNameFromKernel(process_id, buf.as_mut_ptr(), len);
    // println!("tcp->ConnectRequest();");
}

pub extern fn nf_tcp_connected(_id: u64, _conn_info: &NFTcpConnInfo) {
    // println!("tcp->Connected();");
}

pub extern fn nf_tcp_closed(_id: u64, _conn_info: &NFTcpConnInfo) {
    // println!("tcp->Closed();");
}

pub unsafe extern fn nf_tcp_receive(id: u64, buf: *const u8, len: i32) {
    // println!("tcp->receive();");
    nf_tcpPostReceive(id, buf, len);
}

pub unsafe extern fn nf_tcp_send(id: u64, buf: *const u8, len: i32) {
    // println!("tcp->send();");
    nf_tcpPostSend(id, buf, len);
}

pub extern fn nf_tcp_can_receive(_id: u64) {
    // println!("tcp->can_receive();");
}

pub extern fn nf_tcp_can_send(_id: u64) {
    // println!("tcp->can_send();");
}

pub unsafe extern fn nf_udp_created(_id: u64, conn_info: &NFUdpConnInfo) {
    let process_name = get_udp_proc_name(&*conn_info);
    trace!("udp->created() [{}: {}]", conn_info.process_id, process_name)
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
    //     println!("udp->created() _id= {}, process_id={} local_ip= {} local_address={}",
    //              _id,
    //              (*conn_info).process_id,
    //              ip.to_string(),
    //              port);
    // }
}

pub unsafe extern fn nf_udp_connect_request(_id: u64, conn_info: &NFUdpConnInfo) {
    let process_name = get_udp_proc_name(&*conn_info);
    trace!("udp->connect_request() [{}: {}]", conn_info.process_id, process_name)
    // println!("udp->connect_request();");
}

pub unsafe extern fn nf_udp_closed(_id: u64, conn_info: &NFUdpConnInfo) {
    let process_name = get_udp_proc_name(&*conn_info);
    trace!("udp->connect_closed() [{}: {}]", conn_info.process_id, process_name)
}

pub unsafe extern fn nf_udp_receive(id: u64, remote_address: *const SOCKADDR, buf: *const u8, len: i32, options: &NFUdpOptions) {
    // println!("udp->receive();");
    nf_udpPostReceive(id, remote_address, buf, len, options);
}

pub unsafe extern fn nf_udp_send(id: u64, remote_address: *const SOCKADDR, buf: *const u8, len: i32, options: &NFUdpOptions) {
    // println!("udp->send: id={}", id);
    nf_udpPostSend(id, remote_address, buf, len, options);
}

pub extern fn nf_udp_can_receive(_id: u64) {
    // println!("udp->can_receive();");
}

pub extern fn nf_udp_can_send(_id: u64) {
    // println!("udp->can_send();");
}
