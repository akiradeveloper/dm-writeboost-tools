use super::*;

#[derive(Args)]
#[command(about = "Dump a cache block")]
#[command(author, version)]
pub struct Opts {
    #[arg(help = "Path to the cache device")]
    cachedev: String,
    #[arg(help = "Metablock index")]
    mbidx: i32,
    #[arg(long, help = "MBIDX is relative to this SEGID (default is 1)")]
    #[arg(default_value_t = 1)]
    segid: i32,
}

pub fn run(args: Opts) {
    let mb_idx: i32 = args.mbidx;
    let cache_dev = {
        let devname = args.cachedev;
        CacheDevice::new(devname.to_owned())
    };

    let mut base_id = args.segid;

    base_id += mb_idx / 127;
    let idx_inseg = mb_idx % 127;
    let start_byte = (cache_dev.calc_segment_start(base_id) << 9) + ((1 + idx_inseg) << 12);

    use std::process::Command;
    let output = Command::new("od")
        .arg(format!("-j{}", start_byte))
        .arg("-N4096")
        .arg("-Ax")
        .arg(&cache_dev.dev.name())
        .output()
        .expect("failed to execute od")
        .stdout;
    let output = String::from_utf8(output)
        .expect("invalid utf8 output")
        .to_string();
    let output = output.trim();
    println!("{}", output);
}
