mod nf;
use crate::nf::def::*;
use crate::nf::capi::*;

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
        let nf_driver_name = CString::new(b"netfilter2" as &[u8]).unwrap(); //TODO need better way for string -> &u8
        let nf_handler = Default::default();
        let mut nf_rule:NFRule = Default::default();
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
    println!("Hello, world!");
}
