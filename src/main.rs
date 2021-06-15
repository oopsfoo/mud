use std::os::raw::{c_char, c_long, c_longlong, c_short, c_int, c_ulong, c_ushort};
use std::ffi::CString;
use std::io;
use winapi::shared::ws2def::{SOCKADDR, ADDRESS_FAMILY, AF_INET};
use winapi::shared::ws2ipdef;
use std::ptr::slice_from_raw_parts;
use std::net::{SocketAddrV4, Ipv4Addr};
use winapi::um::winsock2::{u_char, u_long, u_short, u_int64};
use winapi::ctypes::c_uchar;

pub type ENDPOINT_ID = u_int64;

#[repr(C)]
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
    NF_ALLOW = 0,
    // Allow the activity without filtering transmitted packets
    NF_BLOCK = 1,
    // Block the activity
    NF_FILTER = 2,
    // Filter the transmitted packets
    NF_SUSPENDED = 4,
    // Suspend receives from server and sends from client
    NF_OFFLINE = 8,
    // Emulate establishing a TCP connection with remote server
    NF_INDICATE_CONNECT_REQUESTS = 16,
    // Indicate outgoing connect requests to API
    NF_DISABLE_REDIRECT_PROTECTION = 32,
    // Disable blocking indicating connect requests for outgoing connections of local proxies
    NF_PEND_CONNECT_REQUEST = 64,
    // Pend outgoing connect request to complete it later using nf_complete(TCP|UDP)ConnectRequest
    NF_FILTER_AS_IP_PACKETS = 128,
    // Indicate the traffic as IP packets via ipSend/ipReceive
    NF_READONLY = 256,
    // Don't block the IP packets and indicate them to ipSend/ipReceive only for monitoring
    NF_CONTROL_FLOW = 512,
    // Use the flow limit rules even without NF_FILTER flag
    NF_REDIRECT = 1024,            // Redirect the outgoing TCP connections to address specified in redirectTo
}

#[repr(C)]
struct TcpConnInfo {
    // unsigned long	filteringFlag;	// See NF_FILTERING_FLAG
    // unsigned long	processId;		// Process identifier
    // unsigned char	direction;		// See NF_DIRECTION
    // unsigned short	ip_family;		// AF_INET for IPv4 and AF_INET6 for IPv6
    // unsigned char	localAddress[NF_MAX_ADDRESS_LENGTH];
    // unsigned char	remoteAddress[NF_MAX_ADDRESS_LENGTH];
}

#[repr(C)]
struct NFUdpConnInfo {
    process_id: c_ulong,
    ip_family: c_ushort,
    local_address: SOCKADDR,
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
    process_id: u_long,
    direction: u_char,
    local_port: u_short,
    remote_port: u_short,
    ip_family: u_short,
    local_ip_address: [u_char; 16],
    local_ip_address_mask: [u_char; 16],
    remote_ip_address: [u_char; 16],
    remote_ip_address_mask: [u_char; 16],
    filtering_flag: NFFilteringFlag,
}

#[repr(C)]
struct NFEventHandler {
    thread_start: extern fn(),
    thread_end: extern fn(),
    tcp_connect_request: extern fn(id: c_longlong, conn_info: TcpConnInfo),
    tcp_connected: extern fn(id: c_longlong, conn_info: TcpConnInfo),
    tcp_closed: extern fn(id: c_longlong, conn_info: TcpConnInfo),
    tcp_receive: extern fn(id: c_longlong, buf: *const c_char, len: c_int),
    tcp_send: extern fn(id: c_longlong, buf: *const c_char, len: c_int),
    tcp_can_receive: extern fn(id: c_longlong),
    tcp_can_send: extern fn(id: c_longlong),
    udp_created: unsafe extern fn(id: c_longlong, conn_info: *const NFUdpConnInfo),
    udp_connect_request: extern fn(id: c_longlong, conn_info: NFUdpConnInfo),
    udp_closed: extern fn(id: c_longlong, conn_info: NFUdpConnInfo),
    udp_receive: unsafe extern fn(id: c_longlong, remote: *const c_char, buf: *const c_char, len: c_int, options: NFUdpOptions),
    udp_send: unsafe extern fn(id: ENDPOINT_ID, remote: *const u_char, buf: *const c_char, len: c_int, options: &NFUdpOptions),
    udp_can_receive: extern fn(id: c_longlong),
    udp_can_send: extern fn(id: c_longlong),
}

#[link(name = "nfapi")]
extern "C" {
    fn nf_init(driver: *const c_char, handler: &NFEventHandler) -> NFStatus;
    fn nf_free();
    fn nf_adjustProcessPriviledges();
    fn nf_addRule(rule: *const NFRule, to_head: c_int) -> NFStatus;
}

extern fn nf_thread_start() {
    println!("threadStart();");
}

extern fn nf_thread_end() {
    println!("threadEnd();");
}

extern fn nf_tcp_connect_request(id: c_longlong, conn_info: TcpConnInfo) {
    println!("tcp->ConnectRequest();");
}

extern fn nf_tcp_connected(id: c_longlong, conn_info: TcpConnInfo) {
    println!("tcp->Connected();");
}

extern fn nf_tcp_closed(id: c_longlong, conn_info: TcpConnInfo) {
    println!("tcp->Closed();");
}

extern fn nf_tcp_receive(id: c_longlong, buf: *const c_char, len: c_int) {
    println!("tcp->receive();");
}

extern fn nf_tcp_send(id: c_longlong, buf: *const c_char, len: c_int) {
    println!("tcp->send();");
}

extern fn nf_tcp_can_receive(id: c_longlong) {
    println!("tcp->can_receive();");
}

extern fn nf_tcp_can_send(id: c_longlong) {
    println!("tcp->can_send();");
}

unsafe extern fn nf_udp_created(id: c_longlong, conn_info: *const NFUdpConnInfo) {
    let local_addr: SOCKADDR = (*conn_info).local_address;
    if i32::from(local_addr.sa_family) == AF_INET {
        println!("-------------------");
        let data = local_addr.sa_data.as_ptr() as *const u8;
        let data_arr = (*slice_from_raw_parts(data, 14)).to_vec();
        let port_bytes = [*data.offset(0), *data.offset(1)];
        let port = u16::from_le_bytes(port_bytes);
        let ip = SocketAddrV4::new(
            Ipv4Addr::new(
                *data.offset(1), *data.offset(3), *data.offset(4), *data.offset(5))
            , port);

        println!("udp->created() id= {}, process_id={} local_ip= {} local_address={}",
                 id,
                 (*conn_info).process_id,
                 ip.to_string(),
                 port);
    }
}

extern fn nf_udp_connect_request(id: c_longlong, conn_info: NFUdpConnInfo) {
    println!("udp->connect_request();");
}

extern fn nf_udp_closed(id: c_longlong, conn_info: NFUdpConnInfo) {
    println!("udp->close();");
}

unsafe extern fn nf_udp_receive(id: c_longlong, remote: *const c_char, buf: *const c_char, len: c_int, options: NFUdpOptions) {
    println!("udp->receive();");
}

unsafe extern fn nf_udp_send(id: ENDPOINT_ID, remoteAddress: *const u_char, buf: *const c_char, len: c_int, options: &NFUdpOptions) {
    println!("udp->send: id={}", id);
}

extern fn nf_udp_can_receive(id: c_longlong) {
    println!("udp->can_receive();");
}

extern fn nf_udp_can_send(id: c_longlong) {
    println!("udp->can_send();");
}

fn main() {
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
            filtering_flag: NFFilteringFlag::NF_FILTER,
        };
        nf_adjustProcessPriviledges();
        // let init_status =
        nf_init(nf_driver_name_ptr, &nf_handler);
        let rule_status = nf_addRule(&nf_rule, 0);
        println!("started");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line");
        println!("You guessed: {}", guess);
        nf_free()
    };
    println!("Hello, world!");
}
