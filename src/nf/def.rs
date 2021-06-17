use crate::nf::handler::*;

use winapi::shared::ws2def::SOCKADDR;

pub const NF_MAX_ADDRESS_LENGTH: usize = 28;
pub const NF_MAX_IP_ADDRESS_LENGTH: usize = 16;

#[repr(C)]
#[derive(Debug)]
#[allow(dead_code)]
pub enum NFStatus {
    NfStatusSuccess = 0,
    NfStatusFail = -1,
    NfStatusInvalidEndpointId = -2,
    NfStatusNotInitialized = -3,
    NfStatusIoError = -4,
    NfStatusRebootRequired = -5,
}

#[repr(u32)]
#[allow(dead_code)]
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

impl Default for NFFilteringFlag {
    fn default() -> Self { NFFilteringFlag::NfAllow }
}

#[repr(u16)]
#[allow(dead_code)]
pub enum NFIpFamily {
    UnSpec = 0,
    V4 = 2,
    V6 = 32,
}

impl Default for NFIpFamily {
    fn default() -> Self { NFIpFamily::UnSpec }
}

#[repr(i32)]
#[allow(dead_code)]
pub enum NFIpProto {
    UnSpec = 0,
    TCP = 6,
    UDP = 17,
}

impl Default for NFIpProto {
    fn default() -> Self { NFIpProto::UnSpec }
}

#[repr(u8)]
#[allow(dead_code)]
pub enum NFDirection {
    UnSpec = 0,
    IN = 1,
    OUT = 2,
    BOTH = 3,
}

impl Default for NFDirection {
    fn default() -> Self { NFDirection::UnSpec }
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
    pub options: *const u8,
    pub options_length: u32,
}

#[repr(packed)]
#[derive(Default)]
pub struct NFRule {
    pub protocol: NFIpProto,
    pub process_id: u32,
    pub direction: NFDirection,
    pub local_port: u16,
    pub remote_port: u16,
    pub ip_family: NFIpFamily,
    pub local_ip_address: [u8; NF_MAX_IP_ADDRESS_LENGTH],
    pub local_ip_address_mask: [u8; NF_MAX_IP_ADDRESS_LENGTH],
    pub remote_ip_address: [u8; NF_MAX_IP_ADDRESS_LENGTH],
    pub remote_ip_address_mask: [u8; NF_MAX_IP_ADDRESS_LENGTH],
    pub filtering_flag: NFFilteringFlag,
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

impl Default for NFEventHandler {
    fn default() -> Self {
        Self {
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
        }
    }
}