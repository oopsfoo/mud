use std::os::raw::*;

pub type EndpointId = winapi::um::winsock2::u_int64;

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
    pub filtering_flag: c_ulong,
    pub process_id: c_ulong,
    pub direction: c_uchar,
    pub ip_family: c_ushort,
    pub local_address: [c_uchar; NF_MAX_ADDRESS_LENGTH],
    pub remote_address: [c_uchar; NF_MAX_ADDRESS_LENGTH],
}

#[repr(packed)]
pub struct NFUdpConnInfo {
    pub process_id: c_ulong,
    pub ip_family: c_ushort,
    pub local_address: [c_uchar; NF_MAX_ADDRESS_LENGTH],
}

#[repr(C)]
pub struct NFUdpOptions {
    pub flags: c_ulong,
    pub options_length: c_long,
    pub options: *const c_uchar,
}

#[repr(packed)]
pub struct NFRule {
    pub protocol: c_int,
    pub process_id: c_ulong,
    pub direction: c_uchar,
    pub local_port: c_ushort,
    pub remote_port: c_ushort,
    pub ip_family: c_ushort,
    pub local_ip_address: [c_uchar; NF_MAX_IP_ADDRESS_LENGTH],
    pub local_ip_address_mask: [c_uchar; NF_MAX_IP_ADDRESS_LENGTH],
    pub remote_ip_address: [c_uchar; NF_MAX_IP_ADDRESS_LENGTH],
    pub remote_ip_address_mask: [c_uchar; NF_MAX_IP_ADDRESS_LENGTH],
    pub filtering_flag: c_ulong,
}

#[repr(C)]
pub struct NFEventHandler {
    pub thread_start: extern fn(),
    pub thread_end: extern fn(),
    pub tcp_connect_request: extern fn(id: c_longlong, conn_info: NFTcpConnInfo),
    pub tcp_connected: extern fn(id: c_longlong, conn_info: NFTcpConnInfo),
    pub tcp_closed: extern fn(id: c_longlong, conn_info: NFTcpConnInfo),
    pub tcp_receive: unsafe extern fn(id: EndpointId, buf: *const c_char, len: c_int),
    pub tcp_send: unsafe extern fn(id: EndpointId, buf: *const c_char, len: c_int),
    pub tcp_can_receive: extern fn(id: c_longlong),
    pub tcp_can_send: extern fn(id: c_longlong),
    pub udp_created: unsafe extern fn(id: c_longlong, conn_info: *const NFUdpConnInfo),
    pub udp_connect_request: extern fn(id: c_longlong, conn_info: NFUdpConnInfo),
    pub udp_closed: extern fn(id: c_longlong, conn_info: NFUdpConnInfo),
    pub udp_receive: unsafe extern fn(id: EndpointId, remote: *const c_uchar, buf: *const c_char, len: c_int, options: &NFUdpOptions),
    pub udp_send: unsafe extern fn(id: EndpointId, remote: *const c_uchar, buf: *const c_char, len: c_int, options: &NFUdpOptions),
    pub udp_can_receive: extern fn(id: c_longlong),
    pub udp_can_send: extern fn(id: c_longlong),
}