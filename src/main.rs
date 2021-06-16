use std::os::raw::{c_char, c_long, c_longlong, c_short, c_int, c_ulong, c_ushort};
use std::net::{SocketAddrV4, Ipv4Addr};
use winapi::ctypes::c_uchar;
use winapi::um::winsock2::{u_char, u_long, u_short, u_int64};
use winapi::shared::ws2def::{SOCKADDR, ADDRESS_FAMILY, AF_INET};
use std::io::stdin;
use std::ffi::CString;

#[macro_use] extern crate log;
extern crate simplelog;
use simplelog::*;

#[link(name = "nfapi")]
extern "C" {
    fn nf_init(driver: *const c_char, handler: &NFEventHandler) -> NFStatus;
    fn nf_free();
    fn nf_adjustProcessPriviledges();
    fn nf_addRule(rule: *const NFRule, to_head: c_int) -> NFStatus;
    fn nf_tcpPostSend(id: EndpointId, buf: *const c_char, len: c_int);
    fn nf_tcpPostReceive(id: EndpointId, buf: *const c_char, len: c_int);
    fn nf_udpPostSend(id: EndpointId, remote_address: *const u_char, buf: *const c_char, len: c_int, options: &NFUdpOptions);
    fn nf_udpPostReceive(id: EndpointId, remote_address: *const u_char, buf: *const c_char, len: c_int, options: &NFUdpOptions);
}

type EndpointId = u_int64;

const NF_MAX_ADDRESS_LENGTH: usize = 28;
const NF_MAX_IP_ADDRESS_LENGTH: usize = 16;

#[repr(C)]
#[derive(Debug)]
enum NFStatus {
    NfStatusSuccess = 0,
    NfStatusFail = -1,
    NfStatusInvalidEndpointId = -2,
    NfStatusNotInitialized = -3,
    NfStatusIoError = -4,
    NfStatusRebootRequired = -5,
}

#[repr(C)]
enum NFFilteringFlag {
    NfAllow = 0,
    NfFilter = 2,
    NfSuspended = 4,
    NfOffline = 8,
    NfIndicateConnectRequests = 16,
    NfDisableRedirectProtection = 32,
    NfPendConnectRequest = 64,
    NfFilterAsIpPackets = 128,
    NfReadonly = 256,
    NfControlFlow = 512,
    NfRedirect = 1024,
}

#[repr(packed)]
struct NFTcpConnInfo {
    filtering_flag: c_ulong,
    process_id: c_ulong,
    direction: c_uchar,
    ip_family: c_ushort,
    local_address: [c_uchar; NF_MAX_ADDRESS_LENGTH],
    remote_address: [c_uchar; NF_MAX_ADDRESS_LENGTH],
}

#[repr(packed)]
struct NFUdpConnInfo {
    process_id: c_ulong,
    ip_family: c_ushort,
    local_address: [c_uchar; NF_MAX_ADDRESS_LENGTH],
}

#[repr(C)]
struct NFUdpOptions {
    flags: c_ulong,
    options_length: c_long,
    options: *const c_uchar,
}

#[repr(packed)]
struct NFRule {
    protocol: c_int,
    process_id: c_ulong,
    direction: c_uchar,
    local_port: c_ushort,
    remote_port: c_ushort,
    ip_family: c_ushort,
    local_ip_address: [c_uchar; NF_MAX_IP_ADDRESS_LENGTH],
    local_ip_address_mask: [c_uchar; NF_MAX_IP_ADDRESS_LENGTH],
    remote_ip_address: [c_uchar; NF_MAX_IP_ADDRESS_LENGTH],
    remote_ip_address_mask: [c_uchar; NF_MAX_IP_ADDRESS_LENGTH],
    filtering_flag: NFFilteringFlag,
}

#[repr(C)]
struct NFEventHandler {
    thread_start: extern fn(),
    thread_end: extern fn(),
    tcp_connect_request: extern fn(id: c_longlong, conn_info: NFTcpConnInfo),
    tcp_connected: extern fn(id: c_longlong, conn_info: NFTcpConnInfo),
    tcp_closed: extern fn(id: c_longlong, conn_info: NFTcpConnInfo),
    tcp_receive: unsafe extern fn(id: EndpointId, buf: *const c_char, len: c_int),
    tcp_send: unsafe extern fn(id: EndpointId, buf: *const c_char, len: c_int),
    tcp_can_receive: extern fn(id: c_longlong),
    tcp_can_send: extern fn(id: c_longlong),
    udp_created: unsafe extern fn(id: c_longlong, conn_info: *const NFUdpConnInfo),
    udp_connect_request: extern fn(id: c_longlong, conn_info: NFUdpConnInfo),
    udp_closed: extern fn(id: c_longlong, conn_info: NFUdpConnInfo),
    udp_receive: unsafe extern fn(id: EndpointId, remote: *const u_char, buf: *const c_char, len: c_int, options: &NFUdpOptions),
    udp_send: unsafe extern fn(id: EndpointId, remote: *const u_char, buf: *const c_char, len: c_int, options: &NFUdpOptions),
    udp_can_receive: extern fn(id: c_longlong),
    udp_can_send: extern fn(id: c_longlong),
}


extern fn nf_thread_start() {
    println!("threadStart();");
}

extern fn nf_thread_end() {
    println!("threadEnd();");
}

extern fn nf_tcp_connect_request(id: c_longlong, conn_info: NFTcpConnInfo) {
    println!("tcp->ConnectRequest();");
}

extern fn nf_tcp_connected(id: c_longlong, conn_info: NFTcpConnInfo) {
    println!("tcp->Connected();");
}

extern fn nf_tcp_closed(id: c_longlong, conn_info: NFTcpConnInfo) {
    println!("tcp->Closed();");
}

unsafe extern fn nf_tcp_receive(id: EndpointId, buf: *const c_char, len: c_int) {
    println!("tcp->receive();");
    nf_tcpPostReceive(id, buf, len);
}

unsafe extern fn nf_tcp_send(id: EndpointId, buf: *const c_char, len: c_int) {
    println!("tcp->send();");
    nf_tcpPostSend(id, buf, len);
}

extern fn nf_tcp_can_receive(id: c_longlong) {
    println!("tcp->can_receive();");
}

extern fn nf_tcp_can_send(id: c_longlong) {
    println!("tcp->can_send();");
}

unsafe extern fn nf_udp_created(id: c_longlong, conn_info: *const NFUdpConnInfo) {
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

extern fn nf_udp_connect_request(id: c_longlong, conn_info: NFUdpConnInfo) {
    println!("udp->connect_request();");
}

extern fn nf_udp_closed(id: c_longlong, conn_info: NFUdpConnInfo) {
    println!("udp->close();");
}

unsafe extern fn nf_udp_receive(id: EndpointId, remoteAddress: *const u_char, buf: *const c_char, len: c_int, options: &NFUdpOptions) {
    println!("udp->receive();");
    nf_udpPostReceive(id, remoteAddress, buf, len, options);
}

unsafe extern fn nf_udp_send(id: EndpointId, remoteAddress: *const u_char, buf: *const c_char, len: c_int, options: &NFUdpOptions) {
    println!("udp->send: id={}", id);
    nf_udpPostSend(id, remoteAddress, buf, len, options);
}

extern fn nf_udp_can_receive(id: c_longlong) {
    println!("udp->can_receive();");
}

extern fn nf_udp_can_send(id: c_longlong) {
    println!("udp->can_send();");
}

fn main() {
    CombinedLogger::init(
        vec![
            TermLogger::new(LevelFilter::Debug, Config::default(), TerminalMode::Mixed, ColorChoice::Auto),
            // WriteLogger::new(LevelFilter::Info, Config::default(), File::create("my_rust_binary.log").unwrap()),
        ]
    ).unwrap();
    unsafe {
        let nf_driver_name = CString::new(b"netfilter2" as &[u8]).unwrap();
        let nf_driver_name_ptr: *const c_char = nf_driver_name.as_ptr();
        let nf_handler = NFEventHandler {
            thread_start: nf_thread_start,
            thread_end: nf_thread_end,
            tcp_connect_request: nf_tcp_connect_request,
            tcp_connected: nf_tcp_connected,
            tcp_closed: nf_tcp_closed,
            tcp_receive: nf_tcp_receive,
            tcp_send: nf_tcp_send,
            tcp_can_receive: nf_tcp_can_receive,
            tcp_can_send: nf_tcp_can_send,
            udp_created: nf_udp_created,
            udp_connect_request: nf_udp_connect_request,
            udp_closed: nf_udp_closed,
            udp_receive: nf_udp_receive,
            udp_send: nf_udp_send,
            udp_can_receive: nf_udp_can_receive,
            udp_can_send: nf_udp_can_send,
        };
        let nf_rule = NFRule {
            protocol: 0,
            process_id: 0,
            direction: 0,
            local_port: 0,
            remote_port: 0,
            ip_family: 0,
            local_ip_address: [0; 16],
            local_ip_address_mask: [0; 16],
            remote_ip_address: [0; 16],
            remote_ip_address_mask: [0; 16],
            filtering_flag: NFFilteringFlag::NfFilter,
        };
        nf_adjustProcessPriviledges();
        // let init_status =
        let init_status = nf_init(nf_driver_name_ptr, &nf_handler);
        match init_status {
            NFStatus::NfStatusSuccess => {
                info!("nf driver inited success! starting add rule.");
                let add_rule_status = nf_addRule(&nf_rule, 0);
                match add_rule_status {
                    NFStatus::NfStatusSuccess => {
                        info!("nf rule added! starting do filtering.");
                    }
                    _ => {
                        println!("add nf rule failed! err={:?} ", add_rule_status);
                    }
                }
            }
            _ => {
                error!("nf driver inited failed! err={:?}", init_status);
            }
        }
        let mut guess = String::new();
        stdin().read_line(&mut guess).expect("Failed to read line");
        println!("You guessed: {}", guess);
        nf_free()
    };
    println!("Hello, world!");
}
