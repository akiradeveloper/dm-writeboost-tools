extern crate clap;

use std::fs::File;
use std::io::Read;
use std::io::Seek;
use std::io::SeekFrom;

use clap::Parser;
#[derive(Parser)]
#[command(name = "wbmeta")]
#[command(about = "Dump a segment header")]
#[command(author, version)]
struct Args {
    #[arg(help = "Path to the cache device")]
    cachedev: String,
    #[arg(help = "Segment id. 0 means the superblock")]
    segid: i32,
}

fn main() {
    let args = Args::parse();

    let devname: String = args.cachedev;
    let id: i32 = args.segid;

    let cache_dev = lib::CacheDevice::new(devname.to_owned());

    if id == 0 {
        // superblock
        let mut buf = [0; 512];
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
        println!(
            "last writeback id = {}",
            sup_record.last_writeback_segment_id
        );
    } else {
        // header
        let mut buf = [0; 4096];
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
            println!(
                "[{}] sector={}, dirty_bits={}",
                i, metablock.sector, metablock.dirty_bits
            );
        }
    }
}
