use crate::nf::def::*;
use crate::nf::capi::*;
use std::net::SocketAddr;

pub unsafe fn nf_process_name(process_id: u32) -> String {
    const BUF_SIZE: usize = 1024;
    let mut buf = [0u16; BUF_SIZE];
    let len = BUF_SIZE as u32;
    nf_getProcessNameFromKernel(process_id, &buf[0], len); //TODO check nf call ret
    let mut i = 0;
    while i < BUF_SIZE {
        let empty:u16 = 0;
        if buf[i] == empty {
            break;
        }
        i += 1;
    }
    String::from_utf16(&buf[..i]).unwrap()
}

// pub fn nf_socket_address() -> &SocketAddr{
//
// }