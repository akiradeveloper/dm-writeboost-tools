// This file is part of dm-writeboost-tools
// Copyright (C) 2015 Akira Hayakawa <ruby.wktk@gmail.com>
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation; either version 2 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License along
// with this program; if not, write to the Free Software Foundation, Inc.,
// 51 Franklin Street, Fifth Floor, Boston, MA 02110-1301 USA.

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
