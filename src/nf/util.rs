use crate::nf::def::*;
use crate::nf::capi::*;

pub unsafe fn get_udp_proc_name(conn_info: &NFUdpConnInfo) -> String {
    const BUF_SIZE: usize = 1024;
    let process_id = conn_info.process_id;
    let mut buf = [0u16; BUF_SIZE];
    let len = BUF_SIZE as u32;
    nf_getProcessNameFromKernel(process_id, buf.as_mut_ptr(), len);
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