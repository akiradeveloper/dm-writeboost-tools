extern crate byteorder;

use byteorder::{ReadBytesExt, LittleEndian};
use std::io::Cursor;
use std::io::Read;
use std::io::Seek;
use std::io::SeekFrom;
use std::fs::File;
use std::path::Path;
use std::collections::HashMap;

pub struct BlockDevice {
    name: String
}

impl BlockDevice {
    pub fn new(name_: String) -> Self {
        BlockDevice {
            name: name_
        }
    }
    pub fn name(&self) -> String {
        self.name.to_owned()
    }
    pub fn size(&self) -> i32 {
        use std::process::Command;
        use std::str::FromStr;
        let output: Vec<u8> =
            Command::new("blockdev")
            .arg("--getsize")
            .arg(&self.name())
            .output()
            .expect(&format!("failed to get the size of {}", self.name()))
            .stdout;
        let output = String::from_utf8(output).expect("invalid utf8 output").to_string();
        let output = output.trim_right();
        i32::from_str(output).expect("couldn't parse as i32")
    }
    fn nr_segments(&self) -> i32 {
        (self.size() - (1 << 11)) / (1 << 10)
    }
    pub fn calc_segment_start(&self, id: i32) -> i32 {
        let idx = (id - 1) % self.nr_segments();
        (1 << 11) + (idx * (1 << 10))
    }
}

pub struct SegmentHeader {
    pub id: u64,
    pub checksum: u32,
    pub length: u8
}

impl SegmentHeader {
    pub fn from_buf(data: &[u8]) -> SegmentHeader {
        let mut rdr = Cursor::new(data);
        let id_ = rdr.read_u64::<LittleEndian>().unwrap();
        let checksum_ = rdr.read_u32::<LittleEndian>().unwrap();
        let length_ = rdr.read_u8().unwrap();
        SegmentHeader {
            id: id_,
            checksum: checksum_,
            length: length_
        }
    }
}

pub struct Metablock {
    pub sector: u64,
    pub dirty_bits: u8,
}

pub struct Segment {}

impl Segment {
    pub fn from_buf(buf: &[u8]) -> (SegmentHeader, Vec<Metablock>) {
        let seg = SegmentHeader::from_buf(buf);

        let mut metablocks = Vec::new();
        let mut rdr = Cursor::new(buf);
        rdr.seek(SeekFrom::Start(512)).unwrap();
        for _ in 0..seg.length {
            let sector_ = rdr.read_u64::<LittleEndian>().unwrap();
            let dirty_bits_ = rdr.read_u8().unwrap();
            let metablock = Metablock {
                sector: sector_,
                dirty_bits: dirty_bits_
            };
            metablocks.push(metablock);
            let padding = 16 - (8 + 1);
            rdr.seek(SeekFrom::Current(padding)).unwrap();
        }
        (seg, metablocks)
    }
}

pub struct SuperBlockHeader {
    pub magic: u32
}

impl SuperBlockHeader {
    pub fn from_buf(data: &[u8]) -> SuperBlockHeader {
        let mut rdr = Cursor::new(data);
        let magic_ = rdr.read_u32::<LittleEndian>().unwrap();
        SuperBlockHeader {
            magic: magic_
        }
    }
}

pub struct SuperBlockRecord {
    pub last_writeback_segment_id: u64
}

impl SuperBlockRecord {
    pub fn from_buf(data: &[u8]) -> SuperBlockRecord {
        let mut rdr = Cursor::new(data);
        let last_writeback_segment_id_ = rdr.read_u64::<LittleEndian>().unwrap();
        SuperBlockRecord {
            last_writeback_segment_id: last_writeback_segment_id_
        }
    }
}

#[derive(Debug)]
pub struct SysDevTable {
    map: HashMap<String, String>
}

impl SysDevTable {
    pub fn from_file(path: &str) -> SysDevTable {
        let mut f = File::open(path).unwrap();
        let mut s = String::new();
        f.read_to_string(&mut s);
        let it = s.lines().map (|line| {
             let v: Vec<&str> = line.split("=").collect();
             (v[0].to_string(), v[1].to_string())
        });
        let mut m = HashMap::new();
        for (k, v) in it {
            m.insert(k, v);
        }
        SysDevTable {
            map: m
        }
    }
    pub fn get(&self, name: &str) -> String {
        self.map[name].to_string()
    }
}

#[test]
fn test_read_sysdev_file() {
    let t = SysDevTable::from_file("data/sysdev.0");
    assert_eq!(t.get("DEVNAME"), "vda1");
}

pub struct DMTable {
    pub backing_dev: String,
    pub caching_dev: String
}

impl DMTable {
    pub fn parse(line: String) -> DMTable {
        let line: Vec<String> = line.split(" ").filter(|x| x != &"").map(|x| x.to_string()).collect();
        println!("[LOG] {:?}", line);
        DMTable {
            backing_dev: line[3].clone(),
            caching_dev: line[4].clone()
        }
    }
}

#[test]
fn test_dmtable_parse() {
    let mut s = String::new();
    let mut f = File::open("data/sample.table.226").unwrap();
    f.read_to_string(&mut s);
    let t = DMTable::parse(s.trim().to_string());
    println!("{}", s.clone());
    assert_eq!(t.backing_dev, "251:0");
    assert_eq!(t.caching_dev, "251:3");
}
