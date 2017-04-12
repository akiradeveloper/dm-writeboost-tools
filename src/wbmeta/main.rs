extern crate clap;
extern crate lib;

use clap::{Arg, App};
use std::fs::File;
use std::io::Seek;
use std::io::SeekFrom;
use std::io::Read;
use std::str::FromStr;

fn main() {
    let matches = App::new("wbmeta")
        .version("1.0.0")
        .author("Akira Hayakawa <ruby.wktk@gmail.com>")
        .about("Dump a segment header")
        .arg(Arg::with_name("CACHEDEV")
             .help("Path to the cache device")
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

        println!("[superblock header]");
        println!("magic = {} ({})", sup_header.magic, s);

        f.seek(SeekFrom::Start((1u64 << 20) - 512)).unwrap();
        f.read(&mut buf).unwrap();
        let sup_record = lib::SuperBlockRecord::from_buf(&buf);

        println!("[superblock record]");
        println!("last writeback id = {}", sup_record.last_writeback_segment_id);
    } else { // header
        let mut buf = [0;4096];
        let mut f = File::open(&devname).unwrap();
        let start_byte: u64 = (cache_dev.calc_segment_start(id) as u64) << 9;
        f.seek(SeekFrom::Start(start_byte)).unwrap();
        f.read(&mut buf).unwrap();
        let (header, metablocks) = lib::Segment::from_buf(&buf);

        println!("[segment header]");
        println!("id        = {}", header.id);
        println!("checksumx = {}", header.checksum);
        println!("length    = {}", header.length);

        for (i, metablock) in metablocks.iter().enumerate() {
            println!("[{}] sector={}, dirty_bits={}", i, metablock.sector, metablock.dirty_bits);
        }
    }
}
