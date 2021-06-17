mod nf;
use crate::nf::def::*;
use crate::nf::capi::*;
use crate::nf::handler::*;

use std::os::raw::*;

use std::io::stdin;
use std::ffi::CString;

#[macro_use]
extern crate log;
extern crate simplelog;

use simplelog::*;

fn main() {
    CombinedLogger::init(
        vec![
            TermLogger::new(LevelFilter::Trace, Config::default(), TerminalMode::Mixed, ColorChoice::Auto),
            // WriteLogger::new(LevelFilter::Info, Config::default(), File::create("my_rust_binary.log").unwrap()),
        ]
    ).unwrap();
    unsafe {
        let nf_driver_name = CString::new(b"netfilter2" as &[u8]).unwrap();
        let nf_driver_name_ptr: *const u8 = nf_driver_name.as_ptr() as *const u8;
        let nf_handler = NFEventHandler {
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
        };
        let mut nf_rule:NFRule = Default::default();
        nf_rule.filtering_flag = NFFilteringFlag::NfFilter;
        nf_adjustProcessPriviledges();
        // let init_status =
        let init_status = nf_init(nf_driver_name_ptr, &nf_handler);
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
    println!("Hello, world!");
}
