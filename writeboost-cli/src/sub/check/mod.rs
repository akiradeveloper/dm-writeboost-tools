use super::*;

use std::fs::File;
use std::io::Read;
use std::io::Seek;
use std::io::SeekFrom;

use crc::{CRC_32_ISCSI, Crc};
pub const CASTAGNOLI: Crc<u32> = Crc::<u32>::new(&CRC_32_ISCSI);
fn checksum(data: &[u8]) -> u32 {
    CASTAGNOLI.checksum(data)
}

#[derive(thiserror::Error, Debug)]
enum CheckError {
    #[error("segment is not initialized")]
    NotInitialized,
    #[error(
        "segment {seg_id} is broken. checksum: computed={computed_checksum}, expected={expected_checksum}"
    )]
    CacheBlockBroken {
        seg_id: u64,
        computed_checksum: u32,
        expected_checksum: u32,
    },
}

fn do_check(devname: &str, seg_id: u64) -> Result<(), CheckError> {
    let cache_dev = CacheDevice::new(devname.to_owned());

    let mut f = File::open(&devname).expect(&format!("Device {} not found", &devname));

    let start_byte: u64 = (cache_dev.calc_segment_start(seg_id) as u64) << 9;
    f.seek(SeekFrom::Start(start_byte)).unwrap();

    let header = {
        let mut buf = vec![0; 512];
        f.read(&mut buf).unwrap();
        SegmentHeader::from_buf(&buf)
    };

    if header.uninitialized() {
        return Err(CheckError::NotInitialized);
    }

    let computed = {
        let size: usize = (4096 - 512) + ((header.length as usize) << 12);
        let mut buf = vec![0; size];
        f.read(&mut buf).unwrap();
        checksum(&buf)
    };

    if computed != header.checksum {
        return Err(CheckError::CacheBlockBroken {
            seg_id,
            computed_checksum: computed,
            expected_checksum: header.checksum,
        });
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_checksum() {
        let buf = vec![0; 4096 - 512];
        assert_eq!(checksum(&buf), 143703573);
    }

    #[test]
    fn test_check() {
        let devname = "data/sample.cache.226";
        let seg_id = 73;
        let res = do_check(&devname, seg_id);
        assert!(res.is_ok());
    }

    #[test]
    fn test_uninitialized() {
        let devname = "data/sample.cache.uninit";
        let seg_id = 3;
        let res = do_check(&devname, seg_id);
        assert!(matches!(res, Err(CheckError::NotInitialized)));
    }
}

#[derive(Args)]
#[command(about = "Check if the segment is broken")]
pub struct CommandArgs {
    #[arg(help = "Path to the cache device")]
    cachedev: String,
    #[arg(long, help = "Segment id to check")]
    segid: Option<u64>,
    #[arg(short, long, help = "Check all segments")]
    all: bool,
}

pub fn run(args: CommandArgs) {
    let devname: String = args.cachedev;

    let range = if let Some(seg_id) = args.segid {
        seg_id..=seg_id
    } else if args.all {
        let cache_dev = CacheDevice::new(devname.to_owned());
        let nr_segs = cache_dev.nr_segments();
        1..=nr_segs as u64
    } else {
        panic!("either --segid or --all must be specified");
    };

    for seg_id in range {
        match do_check(&devname, seg_id) {
            Ok(()) => {}
            Err(CheckError::NotInitialized) => {
                // Since segments are zero-ed out at formatting,
                // if the segment is all zeros, it is considered still unused.
            }
            Err(e) => {
                panic!("{e}")
            }
        }
    }
}
