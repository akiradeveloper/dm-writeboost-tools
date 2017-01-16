extern crate getopts;
extern crate lib;

use std::env;
use std::str::FromStr;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut opts = getopts::Options::new();
    opts.optflag("", "baseid", "todo");
    opts.optflag("h", "help", "todo");
    let matches = opts.parse(&args[1..]).expect("couldn't parse args");
    if matches.free.len() != 2 {
        panic!("too much or less essential parameters (should be two)");
    }
    let devname: String = matches.free[0].clone();
    let mb_idx: i32 = i32::from_str(&matches.free[1].clone()).expect("idx should be int");
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
