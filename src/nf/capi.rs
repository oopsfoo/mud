use crate::nf::def::*;
use winapi::shared::ws2def::SOCKADDR;

#[link(name = "nfapi")]
extern "C" {
    // nf driver
    pub fn nf_init(driver: *const u8, handler: &NFEventHandler) -> NFStatus;
    pub fn nf_free();
    pub fn nf_adjustProcessPriviledges();

    // rule
    pub fn nf_addRule(rule: *const NFRule, to_head: i32) -> NFStatus;

    // nf network
    pub fn nf_tcpPostSend(id: u64, buf: *const u8, len: i32);
    pub fn nf_tcpPostReceive(id: u64, buf: *const u8, len: i32);
    pub fn nf_udpPostSend(id: u64, remote_address: *const SOCKADDR, buf: *const u8, len: i32, options: &NFUdpOptions);
    pub fn nf_udpPostReceive(id: u64, remote_address: *const SOCKADDR, buf: *const u8, len: i32, options: &NFUdpOptions);

    // nf helper
    // BOOL nf_getProcessNameFromKernel(DWORD process_id, wchar_t * buf, DWORD len)
    pub fn nf_getProcessNameFromKernel(process_id: u32, buf: *mut u16, len: u32) -> i32;
}