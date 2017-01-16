extern crate getopts;
extern crate lib;

use std::env;
use std::str::FromStr;
use std::fs::File;
use std::io::Seek;
use std::io::SeekFrom;
use std::io::Read;

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

    if id == 0 { // superblock
        let mut buf = [0;512];
        let mut f = File::open(&devname).unwrap();
        f.read(&mut buf).unwrap();
        let sup_header = lib::SuperBlockHeader::from_buf(&buf);

        let s = if sup_header.magic == 0x57427374 {
            "formatted"
        } else {
            "unformatted"
        };

        println!("superblock header:");
        println!("magic = {} ({})", sup_header.magic, s);

        f.seek(SeekFrom::Start((1u64 << 20) - 512)).unwrap();
        f.read(&mut buf).unwrap();
        let sup_record = lib::SuperBlockRecord::from_buf(&buf);

        println!("superblock record:");
        println!("last writeback id = {}", sup_record.last_writeback_segment_id);
    } else { // header
        let mut buf = [0;4096];
        let mut f = File::open(&devname).unwrap();
        let start_byte: u64 = (dev.calc_segment_start(id) as u64) << 9;
        f.seek(SeekFrom::Start(start_byte)).unwrap();
        f.read(&mut buf).unwrap();
        let (header, metablocks) = lib::Segment::from_buf(&buf);

        println!("segment header:");
        println!("id        = {}", header.id);
        println!("checksumx = {}", header.checksum);
        println!("length    = {}", header.length);

        for (i, metablock) in metablocks.iter().enumerate() {
            println!("[{}] sector={}, dirty_bits={}", i, metablock.sector, metablock.dirty_bits);
        }
    }
}
