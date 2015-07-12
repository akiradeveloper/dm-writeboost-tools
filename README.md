# Tools

## wb\_status

Pretty-print the status line.

* Example: `dmsetup status wb | wb_status`
* Status: Experimental

## wb\_meta

Investigate the metadata in the cache device.

* Example:
  * `wb_meta /dev/cache 0` to see the superblock metadata
  * `wb_meta /dev/cache 1` to see the metadata of segment ID=1
* Status: Experimental

## wb\_dump

Dump the data block corresponding to the specified metablock ID.

* Example: `wb_dump /dev/cache 126`
* Status: Experimental

## wb\_check

Check if the written log is successful.

* Example: `wb_check /dev/cache 1`
* Status: Buggy
