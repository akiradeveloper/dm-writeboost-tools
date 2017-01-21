# dm-writeboost-tools

Commandset to help users of [dm-writeboost](https://github.com/akiradeveloper/dm-writeboost)
to analyze and report bugs. Written in Rust. 

## Install

Rustup is the best way to install Rust toolset.

https://github.com/rust-lang-nursery/rustup.rs

Then `cargo install` to install the commands.

## Video

[[ http://rawgit.com/akiradeveloper/dm-writeboost-tools/master/usage.gif | height = 600px ]]

## Commands (Lexicographical order)

### wbcheck

Check if the log was written successfully.

* `wbcheck /dev/cache 1` checks the segment ID 1. Returns 0 on success.

### wbcreate

Create a WB device (wrapping dmsetup create)

* `wbcreate wbdev /dev/backing /dev/cache --reformat` creates a WB device with reformatting
* `wbcreate wbdev /dev/backing /dev/cache --reformat --read_cache_threshold=4 --writeback_threshold=70`
  creates a WB device with some settings

### wbdump

Dump the data blocks corresponding to the specified metablock ID.

* `wbdump /dev/cache 126` dumps the data block of metablock index 126

### wbmeta

Look into the metadata in the cache device.

* `wbmeta /dev/cache 0` dumps the superblock metadata (ID 0 is special)
* `wbmeta /dev/cache 1` dumps the metadata of segment ID 1

### wbremove

Remove a WB device

* `wbremove wbdev` removes a WB device after flushing data in RAM buffer and then writing back
  all cache blocks. This is the way Dmirty Smirnov's writeboost script suggests. (Recommended)  
* `wbremove wbdev --nowriteback` remove a WB device without writing back all cache blocks.

### wbstatus

Pretty-print the status line.

* `dmsetup status wbdev | wb_status`

## Author

Akira Hayakawa (ruby.wktk@gmail.com)
