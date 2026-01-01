extern crate clap;
extern crate crc;

use std::fs::File;
use std::io::Read;
use std::io::Seek;
use std::io::SeekFrom;

use crc::{Crc, CRC_32_ISCSI};
pub const CASTAGNOLI: Crc<u32> = Crc::<u32>::new(&CRC_32_ISCSI);
fn checksum(data: &[u8]) -> u32 {
    CASTAGNOLI.checksum(data)
}

use clap::Parser;
#[derive(Parser)]
#[command(name = "wbcheck")]
#[command(about = "Check if the segment is broken")]
#[command(author, version)]
struct Args {
    #[arg(help = "Path to the cache device")]
    cachedev: String,
    #[arg(help = "Segment id")]
    segid: i32,
}

#[test]
fn test_checksum() {
    let buf = vec![0; 4096 - 512];
    assert_eq!(checksum(&buf), 143703573);
}

fn main() {
    let args = Args::parse();

    let devname: String = args.cachedev;
    let id = args.segid;
    let cache_dev = lib::CacheDevice::new(devname.to_owned());

    let mut f = File::open(&devname).expect(&format!("Device {} not found", &devname));

    let start_byte: u64 = (cache_dev.calc_segment_start(id) as u64) << 9;
    f.seek(SeekFrom::Start(start_byte)).unwrap();

    let header = {
        let mut buf = vec![0; 512];
        f.read(&mut buf).unwrap();
        lib::SegmentHeader::from_buf(&buf)
    };

    if header.uninitialized() {
        std::process::exit(0);
    }

    let computed = {
        let size: usize = (4096 - 512) + ((header.length as usize) << 12);
        let mut buf = vec![0; size];
        f.read(&mut buf).unwrap();
        checksum(&buf)
    };

    if computed != header.checksum {
        panic!(
            "Checksum is broken. computed={}, expected={}",
            computed, header.checksum
        );
    }
}
