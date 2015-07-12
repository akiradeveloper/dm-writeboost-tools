# dm-writeboost-tools

Commandset to help users of [dm-writeboost](https://github.com/akiradeveloper/dm-writeboost)
to analyze and report bugs.  

## Commands

### wb\_status

Pretty-print the status line.

* Example: `dmsetup status wb | wb_status`
* Status: Experimental

### wb\_meta

Investigate the metadata in the cache device.

* Example:
  * `wb_meta /dev/cache 0` to see the superblock metadata (ID 0 is special)
  * `wb_meta /dev/cache 1` to see the metadata of segment ID 1
* Status: Experimental

### wb\_dump

Dump the data block corresponding to the specified metablock ID.

* Example: `wb_dump /dev/cache 126` to dump the data block of
  metablock ID 126.
* Status: Experimental

### wb\_check

Check if the written log is successful.

* Example: `wb_check /dev/cache 1` to check the segment ID 1
* Status: Experimental

## TODO

* Print usage when -h flag is passed
* Error checks
* Fix (wb\_check) crc32c computation
* Project hierarchy should follow standard
* Tests and Travis
* Distribution packages

## Lisence

```
Copyright (C) 2015 Akira Hayakawa <ruby.wktk@gmail.com>

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
