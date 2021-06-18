use crate::nf::def::*;
use crate::nf::capi::*;
use std::net::*;

//TODO try ret ref, with lifecycle support

pub unsafe fn nf_process_name(process_id: u32) -> String {
    const BUF_SIZE: usize = 1024;
    let buf = [0u16; BUF_SIZE];
    let len = BUF_SIZE as u32;
    nf_getProcessNameFromKernel(process_id, &buf[0], len); //TODO check nf call ret
    let mut i = 0;
    while i < BUF_SIZE {
        let empty: u16 = 0;
        if buf[i] == empty {
            break;
        }
        i += 1;
    }
    String::from_utf16(&buf[..i]).unwrap()
}

pub fn nf_socket_address(nf_sock_addr: &NFSockAddr) -> Option<SocketAddrV4> {
    match nf_sock_addr.sa_family {
        NFIpFamily::V4 => {
            let data = nf_sock_addr.sa_data;
            let port = u16::from_be_bytes([data[0], data[1]]);
            let ip = Ipv4Addr::new(data[2], data[3], data[4], data[5]);
            Some(SocketAddrV4::new(ip, port))
        }
        _ => { None }
    }
}