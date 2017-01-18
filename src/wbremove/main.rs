extern crate getopts;
extern crate lib;

use std::process::Command;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut opts = getopts::Options::new();
    opts.optflag("", "noflush", "Don't flush RAM buffer to caching device before removing");
    opts.optflag("", "nowriteback", "Don't write back dirty caches to the backing device before removing");
    opts.optflag("h", "help", "todo");
    let matches = opts.parse(&args[1..]).expect("Couldn't parse args");

    if matches.free.len() != 1 {
        panic!("Too much or less essential parameters (should be two)");
    }

    let wbname = matches.free[0].clone();

    if !matches.opt_present("noflush") {
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

    if !matches.opt_present("nowriteback") {
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
        println!("[LOG] dev to zero out={}", caching_dev_name);
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
