use crate::nf::def::*;

#[link(name = "nfapi")]
#[allow(improper_ctypes)]
extern "C" {
    // nf driver
    pub fn nf_init(driver: &u8, handler: &NFEventHandler) -> NFStatus;
    pub fn nf_free();
    pub fn nf_adjustProcessPriviledges();

    // rule
    pub fn nf_addRule(rule: &NFRule, to_head: i32) -> NFStatus;

    // nf network
    pub fn nf_tcpPostSend(id: u64, buf: &u8, len: i32);
    pub fn nf_tcpPostReceive(id: u64, buf: &u8, len: i32);
    pub fn nf_udpPostSend(id: u64, remote_address: &NFSockAddr, buf: &u8, len: i32, options: &NFUdpOptions);
    pub fn nf_udpPostReceive(id: u64, remote_address: &NFSockAddr, buf: &u8, len: i32, options: &NFUdpOptions);

    // nf helper
    // BOOL nf_getProcessNameFromKernel(DWORD process_id, wchar_t * buf, DWORD len)
    pub fn nf_getProcessNameFromKernel(process_id: u32, buf: &u16, len: u32) -> i32;
}