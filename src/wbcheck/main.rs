extern crate getopts;
extern crate crc;
extern crate lib;

use std::env;
use std::str::FromStr;
use std::fs::File;
use std::io::Seek;
use std::io::SeekFrom;
use std::io::Read;

fn checksum(data: &[u8]) -> u32 {
    crc::crc32::checksum_castagnoli(data)
}

#[test]
fn test_checksum() {
    let buf = vec![0;4096-512];
    assert_eq!(checksum(&buf), 143703573);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut opts = getopts::Options::new();
    opts.optflag("h", "help", "todo");
    let matches = opts.parse(&args[1..]).expect("couldn't parse args");
    if matches.free.len() != 2 {
        panic!("too much or less essential parameters (should be two)");
    }
    let devname: String = matches.free[0].clone();
    let id: i32 = i32::from_str(&matches.free[1].clone()).expect("id should be int");
    let dev = lib::BlockDevice::new(devname.to_owned());

    let mut f = File::open(&devname).expect(&format!("device {} not found", &devname));

    let start_byte: u64 = (dev.calc_segment_start(id) as u64) << 9;
    f.seek(SeekFrom::Start(start_byte)).unwrap();

    let header = { 
        let mut buf = vec![0;512];
        f.read(&mut buf).unwrap();
        lib::SegmentHeader::from_buf(&buf)
    };

    let computed = {
        let size: usize = (4096 - 512) + ((header.length as usize) << 12);
        let mut buf = vec![0;size];
        f.read(&mut buf).unwrap();
        checksum(&buf)
    };

    if computed != header.checksum {
        panic!(format!("checksum is broken. computed={}, expected={}", computed, header.checksum));
    }
}
