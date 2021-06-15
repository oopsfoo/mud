use std::os::raw::{c_char, c_long, c_longlong, c_short, c_int, c_ulong, c_ushort};
use std::ffi::CString;

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
struct TcpConnInfo {
    // unsigned long	filteringFlag;	// See NF_FILTERING_FLAG
    // unsigned long	processId;		// Process identifier
    // unsigned char	direction;		// See NF_DIRECTION
    // unsigned short	ip_family;		// AF_INET for IPv4 and AF_INET6 for IPv6
    // unsigned char	localAddress[NF_MAX_ADDRESS_LENGTH];
    // unsigned char	remoteAddress[NF_MAX_ADDRESS_LENGTH];
}

#[repr(C)]
struct UdpConnInfo {
    process_id: c_long,
    ip_family: c_short,
    local_address: c_char,
}

#[repr(C)]
struct UdpOptions {
    flags: c_ulong,
    optionsLength: c_long,
    options: c_char,
}

#[repr(C)]
struct NFRule {
    protocol: c_int,
    process_id: c_ulong,
    direction: c_char,
    // unsigned char	direction;	// See NF_DIRECTION
    local_port: c_ushort,
    remote_port: c_ushort,
    ip_family: c_ushort,
    local_ip_address: c_char,
    // unsigned char	localIpAddress[NF_MAX_IP_ADDRESS_LENGTH];
    local_ip_address_mask: c_char,
    // unsigned char	localIpAddressMask[NF_MAX_IP_ADDRESS_LENGTH];
    remote_ip_address: c_char,
    // unsigned char	remoteIpAddress[NF_MAX_IP_ADDRESS_LENGTH];
    remote_ip_address_mask: c_char,
    // unsigned char	remoteIpAddressMask[NF_MAX_IP_ADDRESS_LENGTH];
    filtering_flag: c_long,
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
    udp_created: extern fn(id: c_longlong, conn_info: UdpConnInfo),
    udp_connect_request: extern fn(id: c_longlong, conn_info: UdpConnInfo),
    udp_closed: extern fn(id: c_longlong, conn_info: UdpConnInfo),
    udp_receive: extern fn(id: c_longlong, remote: *const c_char, buf: *const c_char, len: c_int, options: UdpOptions),
    udp_send: extern fn(id: c_longlong, remote: *const c_char, buf: *const c_char, len: c_int, options: UdpOptions),
    udp_can_receive: extern fn(id: c_longlong),
    udp_can_send: extern fn(id: c_longlong),
}

#[link(name = "nfapi")]
extern "C" {
    fn nf_init(driver: *const c_char, handler: &NFEventHandler) -> NFStatus;
    fn nf_free();
    fn nf_addRule(rule: *const NFRule, to_head: c_int) -> c_int;
}

extern fn nf_thread_start() {
    println!("threadStart();");
}

extern fn nf_thread_end() {
    println!("threadEnd();");
}


extern fn nf_tcp_connect_request(id: c_longlong, conn_info: TcpConnInfo) {
    println!("tcpConnectRequest();");
}

extern fn nf_tcp_connected(id: c_longlong, conn_info: TcpConnInfo) {
    println!("tcpConnectRequest();");
}

extern fn nf_tcp_closed(id: c_longlong, conn_info: TcpConnInfo) {
    println!("tcpConnectRequest();");
}

extern fn nf_tcp_receive(id: c_longlong, buf: *const c_char, len: c_int) {
    println!("tcpConnectRequest();");
}

extern fn nf_tcp_send(id: c_longlong, buf: *const c_char, len: c_int) {
    println!("tcpConnectRequest();");
}

extern fn nf_tcp_can_receive(id: c_longlong) {
    println!("tcpConnectRequest();");
}

extern fn nf_tcp_can_send(id: c_longlong) {
    println!("tcpConnectRequest();");
}

extern fn nf_udp_created(id: c_longlong, conn_info: UdpConnInfo) {
    println!("tcpConnectRequest();");
}

extern fn nf_udp_connect_request(id: c_longlong, conn_info: UdpConnInfo) {
    println!("tcpConnectRequest();");
}

extern fn nf_udp_closed(id: c_longlong, conn_info: UdpConnInfo) {
    println!("tcpConnectRequest();");
}

extern fn nf_udp_receive(id: c_longlong, remote: *const c_char, buf: *const c_char, len: c_int, options: UdpOptions) {
    println!("tcpConnectRequest();");
}

extern fn nf_udp_send(id: c_longlong, remote: *const c_char, buf: *const c_char, len: c_int, options: UdpOptions) {
    println!("tcpConnectRequest();");
}

extern fn nf_udp_can_receive(id: c_longlong) {
    println!("tcpConnectRequest();");
}

extern fn nf_udp_can_send(id: c_longlong) {
    println!("udp_can_send({})", id);
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
            local_ip_address: 0,
            local_ip_address_mask: 0,
            remote_ip_address: 0,
            remote_ip_address_mask: 0,
            filtering_flag: 2,
        };
        let nf_status = nf_init(nf_driver_name_ptr, &nf_handler);
        nf_addRule(&nf_rule, 1);
        nf_free()
    };
    println!("Hello, world!");
}
