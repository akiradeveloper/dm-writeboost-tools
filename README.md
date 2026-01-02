# dm-writeboost-tools

[![Join the chat at https://gitter.im/akiradeveloper/dm-writeboost-tools](https://badges.gitter.im/akiradeveloper/dm-writeboost-tools.svg)](https://gitter.im/akiradeveloper/dm-writeboost-tools?utm_source=badge&utm_medium=badge&utm_campaign=pr-badge&utm_content=badge)

Commandset to help users of [dm-writeboost](https://github.com/akiradeveloper/dm-writeboost)
to analyze and report bugs. Written in Rust. 

## Install

Rustup is the best way to install Rust toolset.

https://github.com/rust-lang-nursery/rustup.rs

Then `cargo install --path writeboost-cli` to install **writeboost-cli**.
(For older version, `cargo install --path wbtools`)

## Video

https://www.youtube.com/watch?v=QAXbmr2Rf30

## Commands (Lexicographical order)

### check

Check if the log was written successfully.

* `writeboost-cli check /dev/cache --segid=1` checks the segment ID 1. Returns 0 on success.
* `writeboost-cli check /dev/cache -a` checks all the segments. Returns 0 on success.

### create

Create a WB device (wrapping dmsetup create)

* `writeboost-cli create wbdev /dev/backing /dev/cache --reformat` creates a WB device with reformatting
* `writeboost-cli create wbdev /dev/backing /dev/cache --reformat --read_cache_threshold=4 --writeback_threshold=70`
  creates a WB device with some settings

### dump

Dump the data blocks corresponding to the specified metablock ID.

* `writeboost-cli dump /dev/cache 126` dumps the data block of metablock index 126

### meta

Look into the metadata in the cache device.

* `writeboost-cli meta /dev/cache 0` dumps the superblock metadata (ID 0 is special)
* `writeboost-cli meta /dev/cache 1` dumps the metadata of segment ID 1

### wbremove

Remove a WB device

* `writeboost-cli remove wbdev` removes a WB device after flushing data in RAM buffer and then writing back
  all cache blocks. This is the way Dmirty Smirnov's writeboost script suggests. (Recommended)  
* `writeboost-cli remove wbdev --nowriteback` remove a WB device without writing back all cache blocks.

### wbstatus

Pretty-print the status line.

* `dmsetup status wbdev | writeboost-cli status`

## Author

Akira Hayakawa (ruby.wktk@gmail.com)
