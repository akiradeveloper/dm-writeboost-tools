use super::*;

use std::process::Command;

#[derive(Args)]
#[command(about = "Remove a writeboost device")]
#[command(author, version)]
pub struct CommandArgs {
    #[arg(help = "Name of the writeboost device")]
    lvname: String,
    #[arg(long, help = "Don't flush RAM buffer to cache device before removing")]
    noflush: bool,
    #[arg(
        long,
        help = "Don't write back dirty caches to the backing device before removing"
    )]
    nowriteback: bool,
}

pub fn run(args: CommandArgs) {
    let wbname = args.lvname;

    let do_flush = !args.noflush;
    if do_flush {
        let status = Command::new("dmsetup")
            .arg("suspend")
            .arg(&wbname)
            .status()
            .expect("Failed to flush transient data");
        assert!(status.success());

        let status = Command::new("dmsetup")
            .arg("resume")
            .arg(&wbname)
            .status()
            .expect("Failed to flush transient data");
        assert!(status.success());
    }

    let do_writeback = !args.nowriteback;
    if do_writeback {
        let status = Command::new("dmsetup")
            .arg("message")
            .arg(&wbname)
            .arg("0")
            .arg("drop_caches")
            .status()
            .expect("Failed to drop caches");
        assert!(status.success());
    }

    let cache_dev_name = WBDev::new(wbname.to_string())
        .table()
        .cache_dev
        .sys_dev_table()
        .get("DEVNAME");

    let status = Command::new("dmsetup")
        .arg("remove")
        .arg(&wbname)
        .status()
        .expect("Failed to execute dmsetup remove");
    assert!(status.success());

    if do_writeback {
        let status = Command::new("dd")
            .arg("if=/dev/zero")
            .arg(format!("of=/dev/{}", cache_dev_name))
            .arg("bs=512")
            .arg("count=1")
            .status()
            .expect("Failed to zero out the cache device");
        assert!(status.success());
    }
}
