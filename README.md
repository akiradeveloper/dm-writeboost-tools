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

## Lisence

```
Copyright (C) 2017 Akira Hayakawa <ruby.wktk@gmail.com>

This program is free software; you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation; either version 2 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License along
with this program; if not, write to the Free Software Foundation, Inc.,
51 Franklin Street, Fifth Floor, Boston, MA 02110-1301 USA.
```

## Author

Akira Hayakawa (ruby.wktk@gmail.com)
