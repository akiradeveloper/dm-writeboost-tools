extern crate clap;
extern crate getopts;
extern crate lib;

use std::env;
use std::str::FromStr;
use clap::{Arg, App};

fn main() {
    let matches = App::new("wbdump")
        .version("0.1")
        .author("Akira Hayakawa <ruby.wkkt@gmail.com>")
        .about("Dump a cache block")
        .arg(Arg::with_name("CACHEDEV")
             .required(true)
             .index(1))
        .arg(Arg::with_name("MBIDX")
             .help("metablock index")
             .required(true)
             .index(2))
        .arg(Arg::with_name("baseid")
             .help("MBIDX is relative to this SEGID (default is 1)")
             .long("baseid")
             .takes_value(true))
        .get_matches();

    let devname: String = matches.value_of("CACHEDEV").unwrap().to_string();
    let mb_idx: i32 = i32::from_str(matches.value_of("MBIDX").unwrap()).expect("metablock index should be int");
    let dev = lib::BlockDevice::new(devname.to_owned());

    let mut base_id = 1;
    base_id += mb_idx / 127;
    let idx_inseg = mb_idx % 127;
    let start_byte = (dev.calc_segment_start(base_id) << 9) + ((1 + idx_inseg) << 12);

    use std::process::Command;
    let output = Command::new("od")
        .arg(format!("-j{}", start_byte))
        .arg("-N4096")
        .arg("-Ax")
        .arg(&dev.name())
        .output()
        .expect("failed to execute od")
        .stdout;
    let output = String::from_utf8(output).expect("invalid utf8 output").to_string();
    let output = output.trim();
    println!("{}", output);
}
