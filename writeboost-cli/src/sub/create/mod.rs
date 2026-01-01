use super::*;

use std::process::Command;

#[derive(Args)]
#[command(about = "Create a writeboost device")]
#[command(author, version)]
#[clap(rename_all = "snake_case")]
pub struct CommandArgs {
    #[arg(help = "Name of the writeboost device")]
    lvname: String,
    #[arg(help = "Path to the backing device")]
    backingdev: String,
    #[arg(help = "Path to the cache device")]
    cachedev: String,
    #[arg(
        help = "Reformat the cache device. This cleans up all existing cache blocks",
        long
    )]
    reformat: bool,
    #[arg(long)]
    write_around_mode: bool,
    #[arg(long, value_name = "INT")]
    writeback_threshold: Option<u32>,
    #[arg(long, value_name = "INT")]
    nr_max_batched_writeback: Option<u32>,
    #[arg(long, value_name = "INT")]
    update_sb_record_interval: Option<u32>,
    #[arg(long, value_name = "INT")]
    sync_data_interval: Option<u32>,
    #[arg(long, value_name = "INT")]
    read_cache_threshold: Option<u32>,
}

pub fn run(args: CommandArgs) {
    let wbname = args.lvname;
    let backing_dev = {
        let name = args.backingdev;
        BlockDevice::new(name)
    };
    let cache_dev_name = args.cachedev;

    if args.reformat {
        let status = Command::new("dd")
            .arg("if=/dev/zero")
            .arg(format!("of={}", cache_dev_name))
            .arg("bs=512")
            .arg("count=1")
            .status()
            .expect("Failed to zero out the cache device");
        assert!(status.success());
    }

    let mut optionals: Vec<String> = Vec::new();
    if args.write_around_mode {
        optionals.push("write_around_mode".to_string());
        optionals.push("1".to_string());
    }
    if let Some(v) = args.writeback_threshold {
        optionals.push("writeback_threshold".to_string());
        optionals.push(v.to_string());
    }
    if let Some(v) = args.nr_max_batched_writeback {
        optionals.push("nr_max_batched_writeback".to_string());
        optionals.push(v.to_string());
    }
    if let Some(v) = args.update_sb_record_interval {
        optionals.push("update_sb_record_interval".to_string());
        optionals.push(v.to_string());
    }
    if let Some(v) = args.sync_data_interval {
        optionals.push("sync_data_interval".to_string());
        optionals.push(v.to_string());
    }
    if let Some(v) = args.read_cache_threshold {
        optionals.push("read_cache_threshold".to_string());
        optionals.push(v.to_string());
    }

    let n = optionals.len();

    let optionals_table = if n == 0 {
        "".to_string()
    } else {
        format!(" {} {}", n, optionals.join(" "))
    };

    let table = format!(
        "0 {} writeboost {} {}{}",
        backing_dev.size(),
        backing_dev.name(),
        cache_dev_name,
        optionals_table
    );

    let status = Command::new("dmsetup")
        .arg("create")
        .arg(wbname)
        .arg("--table")
        .arg(table)
        .status()
        .expect("Failed to execute dmsetup create");
    assert!(status.success());
}
