extern crate clap;

use clap::App;
use std::io::{self, Read};

fn print_status(s: &str) {
    let toks: Vec<&str> = {
        s.split_whitespace().collect()
    };

    println!("cursor pos              = {}", toks[3]);
    println!("# of cache blocks       = {}", toks[4]);
    println!("# of segments           = {}", toks[5]);
    println!("current id              = {}", toks[6]);
    println!("last flushed id         = {}", toks[7]);
    println!("last writeback id       = {}", toks[8]);
    println!("# of dirty cache blocks = {}", toks[9]);
    println!("# of partial flushes    = {}", toks[26]);

    println!("write? hit? on_buffer? fullsize?");
    for i in 0..16 {
        let b = |bit: usize| {
            if (i & (1 << bit)) > 0 {
                1
            } else {
                0
            }
        };
        let v = toks[10 + i];
        println!("{}      {}    {}          {}         {}", b(3), b(2), b(1), b(0), v);
    }
}

#[test]
fn test_print_status() {
    let mut buf = String::new();
    let mut f = std::fs::File::open("data/sample.status.226").unwrap();
    f.read_to_string(&mut buf).unwrap();
    print_status(&buf);
}

fn main() {
    App::new("wbstatus")
        .version("1.0.0")
        .author("Akira Hayakawa <ruby.wktk@gmail.com>")
        .about("Pretty-print the dmsetup status output")
        .usage("dmsetup status wbdev | wbstatus")
        .get_matches();

    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf).unwrap();

    print_status(&buf);
}
