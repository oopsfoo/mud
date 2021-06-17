use winapi::shared::ws2def::SOCKADDR;

pub const NF_MAX_ADDRESS_LENGTH: usize = 28;
pub const NF_MAX_IP_ADDRESS_LENGTH: usize = 16;

#[repr(C)]
#[derive(Debug)]
pub enum NFStatus {
    NfStatusSuccess = 0,
    NfStatusFail = -1,
    NfStatusInvalidEndpointId = -2,
    NfStatusNotInitialized = -3,
    NfStatusIoError = -4,
    NfStatusRebootRequired = -5,
}

#[repr(C)]
pub enum NFFilteringFlag {
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
pub struct NFTcpConnInfo {
    pub filtering_flag: u32,
    pub process_id: u32,
    pub direction: u8,
    pub ip_family: u16,
    pub local_address: [u8; NF_MAX_ADDRESS_LENGTH],
    pub remote_address: [u8; NF_MAX_ADDRESS_LENGTH],
}

#[repr(packed)]
pub struct NFUdpConnInfo {
    pub process_id: u32,
    pub ip_family: u16,
    pub local_address: [u8; NF_MAX_ADDRESS_LENGTH],
}

#[repr(C)]
pub struct NFUdpOptions {
    pub flags: u32,
    pub options_length: u32,
    pub options: *const u8,
}

#[repr(packed)]
pub struct NFRule {
    pub protocol: i32,
    pub process_id: u32,
    pub direction: u8,
    pub local_port: u16,
    pub remote_port: u16,
    pub ip_family: u16,
    pub local_ip_address: [u8; NF_MAX_IP_ADDRESS_LENGTH],
    pub local_ip_address_mask: [u8; NF_MAX_IP_ADDRESS_LENGTH],
    pub remote_ip_address: [u8; NF_MAX_IP_ADDRESS_LENGTH],
    pub remote_ip_address_mask: [u8; NF_MAX_IP_ADDRESS_LENGTH],
    pub filtering_flag: u32,
}

#[repr(C)]
pub struct NFEventHandler {
    pub thread_start: extern fn(),
    pub thread_end: extern fn(),
    pub tcp_connect_request: unsafe extern fn(id: u64, conn_info: &NFTcpConnInfo),
    pub tcp_connected: extern fn(id: u64, conn_info: &NFTcpConnInfo),
    pub tcp_closed: extern fn(id: u64, conn_info: &NFTcpConnInfo),
    pub tcp_receive: unsafe extern fn(id: u64, buf: *const u8, len: i32),
    pub tcp_send: unsafe extern fn(id: u64, buf: *const u8, len: i32),
    pub tcp_can_receive: extern fn(id: u64),
    pub tcp_can_send: extern fn(id: u64),
    pub udp_created: unsafe extern fn(id: u64, conn_info: &NFUdpConnInfo),
    pub udp_connect_request: unsafe extern fn(id: u64, conn_info: &NFUdpConnInfo),
    pub udp_closed: unsafe extern fn(id: u64, conn_info: &NFUdpConnInfo),
    pub udp_receive: unsafe extern fn(id: u64, remote: *const SOCKADDR, buf: *const u8, len: i32, options: &NFUdpOptions),
    pub udp_send: unsafe extern fn(id: u64, remote: *const SOCKADDR, buf: *const u8, len: i32, options: &NFUdpOptions),
    pub udp_can_receive: extern fn(id: u64),
    pub udp_can_send: extern fn(id: u64),
}