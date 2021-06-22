mod nf;
mod net;

use crate::nf::def::*;
use crate::nf::capi::*;
use crate::nf::util::*;

use crate::net::ss;

use std::io::stdin;
use std::ffi::CString;

#[macro_use]
extern crate log;
extern crate simplelog;

use simplelog::*;
use std::net::{UdpSocket, SocketAddr};
use std::ptr::slice_from_raw_parts;
use std::error::Error;
use tokio::runtime::Builder;

fn main() {
    CombinedLogger::init(
        vec![
            TermLogger::new(LevelFilter::Info, Config::default(), TerminalMode::Mixed, ColorChoice::Auto),
            // WriteLogger::new(LevelFilter::Info, Config::default(), File::create("my_rust_binary.log").unwrap()),
        ]
    ).unwrap();
    unsafe {
        let nf_driver_name = CString::new(b"netfilter2" as &[u8]).unwrap(); //TODO need better way for string -> &u8
        let mut nf_handler: NFEventHandler = Default::default();
        nf_handler.udp_send = my_udp_send;
        let mut nf_rule: NFRule = Default::default();
        nf_rule.protocol = NFIpProto::UDP;
        nf_rule.ip_family = NFIpFamily::V4;
        nf_rule.filtering_flag = NFFilteringFlag::NfFilter;

        nf_adjustProcessPriviledges();
        let init_status = nf_init(&nf_driver_name.to_bytes()[0], &nf_handler);
        match init_status {
            NFStatus::NfStatusSuccess => {
                info!("nf driver inited success! starting add rule.");
                let add_rule_status = nf_addRule(&nf_rule, 0);
                match add_rule_status {
                    NFStatus::NfStatusSuccess => {
                        info!("nf rule added! starting do filtering.");
                    }
                    _ => {
                        println!("add nf rule failed! err={:?} ", add_rule_status);
                    }
                }
            }
            _ => {
                error!("nf driver inited failed! err={:?}", init_status);
            }
        }
        let mut guess = String::new();
        stdin().read_line(&mut guess).expect("Failed to read line");
        println!("You guessed: {}", guess);
        nf_free()
    };
}


unsafe extern fn my_udp_send(id: u64, remote_address: &NFSockAddr, buf: &u8, len: i32, options: &NFUdpOptions) {
    let remote = nf_socket_address(remote_address);
    match remote {
        None => { return; }
        Some(r) => {
            if r.port() != 53 {
                nf_udpPostSend(id, remote_address, buf, len, options);
            } else {
                my_real_udp_send(id, remote_address, buf, len, options);
            }
        }
    }
}

unsafe extern fn my_real_udp_send(id: u64, remote_address: &NFSockAddr, buf: &u8, len: i32, options: &NFUdpOptions) {
    info!("myudp->send() [{}: packet to: {}, size: {}]", id, nf_socket_address(remote_address).unwrap(), len);
    // let data_size: usize = len;
    let data = slice_from_raw_parts(buf, len as usize);
    let server = String::from("114.114.114.114:53");
    let socket = UdpSocket::bind("0.0.0.0:0").unwrap();
    info!("my_conn_r() [server_addr:{}]", server);
    let conn_result = socket.connect(server);
    conn_result.unwrap();
    let send_result = socket.send(data.as_ref().unwrap());
    send_result.unwrap();
    let mut buff = [0u8; 4096];
    let (ret_len, _) = socket.recv_from(&mut buff).unwrap();
    info!("my_conn_r() [ret_len: {}]", ret_len);
    nf_udpPostReceive(id, remote_address, &buff[0], ret_len as i32, options);

    info!("send to ss");
    // USE ss udp
    let dst = nf_socket_address(remote_address).unwrap();

    let mut builder = Builder::new_current_thread();

    let runtime = builder.enable_all().build().expect("create tokio Runtime");

    runtime.block_on(
        ss::udp::send(SocketAddr::V4(dst).clone(), data.as_ref().unwrap())
    );
}