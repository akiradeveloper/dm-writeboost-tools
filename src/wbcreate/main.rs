extern crate getopts;
extern crate lib;

use std::process::Command;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut opts = getopts::Options::new();
    opts.optopt("", "writeback_threshold", "todo", "INT");
    opts.optopt("", "nr_max_batched_writeback", "todo", "INT");
    opts.optopt("", "update_sb_record_interval", "todo", "INT");
    opts.optopt("", "sync_data_interval", "todo", "INT");
    opts.optopt("", "read_cache_threshold", "todo", "INT");
    opts.optflag("", "write_around_mode", "todo");
    opts.optflag("", "clean_caches", "todo");
    opts.optflag("h", "help", "todo");
    let matches = opts.parse(&args[1..]).expect("couldn't parse args");

    if matches.free.len() != 3 {
        panic!("too much or less essential parameters (should be two)");
    }

    let wbname = matches.free[0].clone();
    let backing_dev = {
         let name = matches.free[1].clone();
         lib::BlockDevice::new(name)
    };
    let caching_dev_name = matches.free[2].clone();

    let mut optionals: Vec<String> = Vec::new();
    if matches.opt_present("write_around_mode") {
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
        match matches.opt_str(name) {
            Some(value) => {
                optionals.push(name.to_string());
                optionals.push(value);
            }
            _ => {}
        }
    }

    let n = optionals.len();

    let optionals_table = if n == 0 {
        "".to_string()
    } else {
        format!(" {} {}", n, optionals.join(" "))
    };

    let table = format!("\"0 {} writeboost {} {}{}\"",
                        backing_dev.size(),
                        backing_dev.name(),
                        caching_dev_name,
                        optionals_table);

    let _ = Command::new("dmsetup")
        .arg("create")
        .arg(wbname)
        .arg("--table")
        .arg(table)
        .spawn()
        .expect("failed to execute dmsetup create");
}
