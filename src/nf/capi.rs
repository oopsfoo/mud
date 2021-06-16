use crate::nf::def::*;
use std::os::raw::*;

#[link(name = "nfapi")]
extern "C" {
    pub fn nf_init(driver: *const c_char, handler: &NFEventHandler) -> NFStatus;
    pub fn nf_free();
    pub fn nf_adjustProcessPriviledges();
    pub fn nf_addRule(rule: *const NFRule, to_head: c_int) -> NFStatus;
    pub fn nf_tcpPostSend(id: EndpointId, buf: *const c_char, len: c_int);
    pub fn nf_tcpPostReceive(id: EndpointId, buf: *const c_char, len: c_int);
    pub fn nf_udpPostSend(id: EndpointId, remote_address: *const c_uchar, buf: *const c_char, len: c_int, options: &NFUdpOptions);
    pub fn nf_udpPostReceive(id: EndpointId, remote_address: *const c_uchar, buf: *const c_char, len: c_int, options: &NFUdpOptions);
}