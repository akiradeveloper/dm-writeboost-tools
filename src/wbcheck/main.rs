extern crate clap;
extern crate crc;
extern crate lib;

use clap::{Arg, App};
use std::fs::File;
use std::io::Seek;
use std::io::SeekFrom;
use std::io::Read;
use std::str::FromStr;

fn checksum(data: &[u8]) -> u32 {
    crc::crc32::checksum_castagnoli(data)
}

#[test]
fn test_checksum() {
    let buf = vec![0;4096-512];
    assert_eq!(checksum(&buf), 143703573);
}

fn main() {
    let matches = App::new("wbcheck")
        .version(lib::VERSION)
        .author("Akira Hayakawa <ruby.wktk@gmail.com>")
        .about("Check if the segment is broken")
        .arg(Arg::with_name("CACHEDEV")
             .help("Name of the cache device")
             .required(true)
             .index(1))
        .arg(Arg::with_name("SEGID")
             .help("Segment id")
             .required(true)
             .index(2))
        .get_matches();

    let devname: String = matches.value_of("CACHEDEV").unwrap().to_string();
    let id: i32 = i32::from_str(matches.value_of("SEGID").unwrap()).expect("Segment id should be int");
    let cache_dev = lib::CacheDevice::new(devname.to_owned());

    let mut f = File::open(&devname).expect(&format!("Device {} not found", &devname));

    let start_byte: u64 = (cache_dev.calc_segment_start(id) as u64) << 9;
    f.seek(SeekFrom::Start(start_byte)).unwrap();

    let header = { 
        let mut buf = vec![0;512];
        f.read(&mut buf).unwrap();
        lib::SegmentHeader::from_buf(&buf)
    };

    if header.uninitialized() {
        std::process::exit(0);
    }

    let computed = {
        let size: usize = (4096 - 512) + ((header.length as usize) << 12);
        let mut buf = vec![0;size];
        f.read(&mut buf).unwrap();
        checksum(&buf)
    };

    if computed != header.checksum {
        panic!(format!("Checksum is broken. computed={}, expected={}", computed, header.checksum));
    }
}
