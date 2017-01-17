# dm-writeboost-tools

Commandset to help users of [dm-writeboost](https://github.com/akiradeveloper/dm-writeboost)
to analyze and report bugs. Written in Rust. 

## Install

Rustup is the best way to install Rust toolset.

https://github.com/rust-lang-nursery/rustup.rs

Then `cargo install` to install the commands.

## Commands (Lexicographical order)

### wbcheck

Check if the log was written successfully.

* `wbcheck /dev/cache 1` to check the segment ID 1

### wbdump

Dump the data blocks corresponding to the specified metablock ID.

* `wbdump /dev/cache 126` to dump the data block of metablock IDX 126

### wbmeta

Look into the metadata in the cache device.

* `wbmeta /dev/cache 0` to see the superblock metadata (ID 0 is special)
* `wbmeta /dev/cache 1` to see the metadata of segment ID 1

### wbstatus

Pretty-print the status line.

* `dmsetup status wbdev | wb_status`

## Author

Akira Hayakawa (ruby.wktk@gmail.com)
