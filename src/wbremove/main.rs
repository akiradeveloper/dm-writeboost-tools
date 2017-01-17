extern crate getopts;
extern crate lib;

use std::process::Command;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut opts = getopts::Options::new();
    opts.optflag("", "no-flush", "Don't flush RAM buffer to caching device before removing");
    opts.optflag("", "no-writeback", "Don't write back dirty caches to the backing device before removing");
    opts.optflag("h", "help", "todo");
    let matches = opts.parse(&args[1..]).expect("Couldn't parse args");

    if matches.free.len() != 1 {
        panic!("Too much or less essential parameters (should be two)");
    }

    let wbname = matches.free[0].clone();

    if !matches.opt_present("no-flush") {
        Command::new("dmsetup")
            .arg("suspend")
            .arg(&wbname)
            .spawn()
            .expect("Failed to flush transient data");

        Command::new("dmsetup")
            .arg("resume")
            .arg(&wbname)
            .spawn()
            .expect("Failed to flush transient data");
    }

    if !matches.opt_present("no-writeback") {
        Command::new("dmsetup")
            .arg("message")
            .arg(&wbname)
            .arg("0")
            .arg("drop_caches")
            .spawn()
            .expect("Failed to drop caches");
        
        // kill the cache blocks
    }

    Command::new("dmsetup")
        .arg("remove")
        .arg(&wbname)
        .spawn()
        .expect("Failed to execute dmsetup create");
}
