// TODO lisence

package main

import (
	"encoding/binary"
	"fmt"
	"io"
	"io/ioutil"
	"os"
	"strconv"

	"./lib"
)

type sb_header struct {
	magic uint32
}

type sb_record struct {
}

type metablock struct {
	sector     uint64
	dirty_bits uint8
}

// TODO help

func read_metablock(f io.Reader) metablock {
	var v metablock
	binary.Read(f, binary.LittleEndian, &v.sector)
	binary.Read(f, binary.LittleEndian, &v.dirty_bits)
	io.CopyN(ioutil.Discard, f, 16-(8+1))
	return v
}

func main() {
	dev := lib.NewDev(os.Args[1])
	// fmt.Println(dev.Size())
	id, _ := strconv.Atoi(os.Args[2])
	if id == 0 { // superblock
		f, _ := os.Open(dev.Name())
		var v32 uint32
		binary.Read(f, binary.LittleEndian, &v32)
		var s string
		if v32 == 0x57427374 {
			s = "formatted"
		} else {
			s = "not formatted"
		}
		fmt.Println("superblock header")
		fmt.Printf("magic number: 0x%x (%s)\n", v32, s)

		fmt.Println()

		var v64 uint64
		f.Seek((1<<20)-512, 0)
		binary.Read(f, binary.LittleEndian, &v64)
		fmt.Println("superblock record")
		fmt.Println("last writeback id:", v64)
	} else { // segment header (id > 0)
		f, _ := os.Open(dev.Name())
		f.Seek(int64(dev.CalcSegmentStart(id))<<9, 0)
		segH := lib.ReadSegmentHeader(f)
		fmt.Println("id:", segH.Id)
		fmt.Println("checksum:", segH.Checksum)
		fmt.Println("length:", segH.Length)
		fmt.Println()
		for i := 0; i < 127; i++ {
			mb := read_metablock(f)
			fmt.Printf("%3d sector=%d dirty_bits=%d\n", i, mb.sector, mb.dirty_bits)
		}
	}
}
