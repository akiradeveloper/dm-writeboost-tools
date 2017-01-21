extern crate clap;
extern crate getopts;
extern crate lib;

use clap::{Arg, App};
use std::process::Command;

fn main() {
    let matches = App::new("wbremove")
        .version("1.0.0")
        .author("Akira Hayakawa <ruby.wktk@gmail.com>")
        .about("Remove a writeboost device")
        .arg(Arg::with_name("LVNAME")
             .help("Name of the writeboost device")
             .required(true)
             .index(1))
        .arg(Arg::with_name("noflush")
             .help("Don't flush RAM buffer to cache device before removing")
             .long("noflush"))
        .arg(Arg::with_name("nowriteback")
             .help("Don't write back dirty caches to the backing device before removing")
             .long("nowriteback"))
        .get_matches();

    let wbname = matches.value_of("LVNAME").unwrap().to_string();

    if !matches.is_present("noflush") {
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

    let will_writeback = !matches.is_present("nowriteback");

    if will_writeback {
        Command::new("dmsetup")
            .arg("message")
            .arg(&wbname)
            .arg("0")
            .arg("drop_caches")
            .spawn()
            .expect("Failed to drop caches");
    }

    Command::new("dmsetup")
        .arg("remove")
        .arg(&wbname)
        .spawn()
        .expect("Failed to execute dmsetup create");

    if will_writeback {
        let cache_dev_name = lib::WBDev::new(wbname.to_string())
            .table()
            .cache_dev
            .sys_dev_table()
            .get("DEVNAME");

        Command::new("dd")
            .arg("if=/dev/zero")
            .arg(format!("of={}", cache_dev_name))
            .arg("bs=512")
            .arg("count=1")
            .spawn()
            .expect("Failed to zero out the caching device");
    }
}
