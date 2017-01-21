extern crate clap;
extern crate lib;

use std::process::Command;
use clap::{Arg, App};

fn main() {
    let matches = App::new("wbcreate")
        .version("1.0.0")
        .author("Akira Hayakawa <ruby.wktk@gmail.com>")
        .about("Create a writeboost device")
        .arg(Arg::with_name("LVNAME")
             .help("Name of the writeboost device")
             .required(true)
             .index(1))
        .arg(Arg::with_name("BACKINGDEV")
             .help("Path to the backing device")
             .required(true)
             .index(2))
        .arg(Arg::with_name("CACHEDEV")
             .help("Path to the cache device")
             .required(true)
             .index(3))
        .arg(Arg::with_name("reformat")
             .help("Reformat the cache device. This cleans up all existing cache blocks")
             .long("reformat"))
        .arg(Arg::with_name("write_around_mode")
             .long("write_around_mode"))
        .arg(Arg::with_name("writeback_threshold")
             .long("writeback_threshold")
             .value_name("INT")
             .takes_value(true))
        .arg(Arg::with_name("nr_max_batched_writeback")
             .long("nr_max_batched_writeback")
             .value_name("INT")
             .takes_value(true))
        .arg(Arg::with_name("update_sb_record_interval")
             .long("update_sb_record_interval")
             .value_name("INT")
             .takes_value(true))
        .arg(Arg::with_name("sync_data_interval")
             .long("sync_data_interval")
             .value_name("INT")
             .takes_value(true))
        .arg(Arg::with_name("read_cache_threshold")
             .long("read_cache_threshold")
             .value_name("INT")
             .takes_value(true))
        .get_matches();

    let wbname = matches.value_of("LVNAME").unwrap().to_string();
    let backing_dev = {
         let name = matches.value_of("BACKINGDEV").unwrap().to_string();
         lib::BlockDevice::new(name)
    };
    let cache_dev_name = matches.value_of("CACHEDEV").unwrap().to_string();

    if matches.is_present("reformat") {
        let status = Command::new("dd")
            .arg("if=/dev/zero")
            .arg(format!("of={}", cache_dev_name))
            .arg("bs=512")
            .arg("count=1")
            .status()
            .expect("Failed to zero out the cache device");
        assert!(status.success());
    }

    let mut optionals: Vec<String> = Vec::new();
    if matches.is_present("write_around_mode") {
        optionals.push("write_around_mode".to_string());
        optionals.push("1".to_string());
    }
    let tunables = [
        "writeback_threshold",
        "nr_max_batched_writeback",
        "update_sb_record_interval",
        "sync_data_interval",
        "read_cache_threshold"];
    for name in &tunables {
        if let Some(value) = matches.value_of(name) {
            optionals.push(name.to_string());
            optionals.push(value.to_string());
        }
    }

    let n = optionals.len();

    let optionals_table = if n == 0 {
        "".to_string()
    } else {
        format!(" {} {}", n, optionals.join(" "))
    };

    let table = format!("0 {} writeboost {} {}{}",
                        backing_dev.size(),
                        backing_dev.name(),
                        cache_dev_name,
                        optionals_table);

    let status = Command::new("dmsetup")
        .arg("create")
        .arg(wbname)
        .arg("--table")
        .arg(table)
        .status()
        .expect("Failed to execute dmsetup create");
    assert!(status.success());
}
