extern crate clap;
extern crate getopts;
extern crate lib;

use std::process::Command;
use std::env;
use clap::{Arg, App};

fn main() {
    let matches = App::new("wbremove")
        .version("0.1")
        .author("Akira Hayakawa <ruby.wkkt@gmail.com>")
        .about("Remove a writeboost device")
        .arg(Arg::with_name("LVNAME")
             .required(true)
             .index(1))
        .arg(Arg::with_name("noflush")
             .help("Don't flush RAM buffer to caching device before removing")
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

    if !matches.is_present("nowriteback") {
        Command::new("dmsetup")
            .arg("message")
            .arg(&wbname)
            .arg("0")
            .arg("drop_caches")
            .spawn()
            .expect("Failed to drop caches");

        let output = Command::new("dmsetup")
            .arg("table")
            .arg(&wbname)
            .output()
            .expect("Fail to dmsetup table")
            .stdout;
        let output = String::from_utf8(output).expect("invalid utf8 output").to_string();
        let output = output.trim().to_string();
        let dm_table = lib::DMTable::parse(output);
        let path = format!("/sys/dev/block/{}/uevent", dm_table.caching_dev);
        let sysdev_table = lib::SysDevTable::from_file(&path);
        let caching_dev_name = sysdev_table.get("DEVNAME");
        Command::new("dd")
            .arg("if=/dev/zero")
            .arg(format!("of={}", caching_dev_name))
            .arg("bs=512")
            .arg("count=1")
            .spawn()
            .expect("Failed to zero out the caching device");
    }

    Command::new("dmsetup")
        .arg("remove")
        .arg(&wbname)
        .spawn()
        .expect("Failed to execute dmsetup create");
}
