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
	"fmt"
	"hash/crc32"
	"os"
	"strconv"

	"./lib"
)

// FIXME
// the checksums don't match
// I am not sure either the way of computing the hash value or
// the we are seeing different data causes this bug.
// simply, don't use this program.

// We need the same function in libcrc32c in Linux kernel.
// I don't have a clue how I can make it out using Go's crc32.
// If you find some solution please share it.

func checkSum(data []byte) uint32 {
	// fmt.Println(len(data))
	var table = crc32.MakeTable(crc32.Castagnoli)
	return crc32.Update(0xffffffff, table, data)
}

func main() {
	dev := lib.NewDev(os.Args[1])
	id, _ := strconv.Atoi(os.Args[2])

	f, _ := os.Open(dev.Name())

	startByte := int64(dev.CalcSegmentStart(id)) << 9
	f.Seek(startByte, 0)
	segH := lib.ReadSegmentHeader(f)
	// fmt.Println("read seg")

	// fmt.Println(segH.Id)
	fmt.Println(segH.Checksum)
	// fmt.Println(segH.Length)

	size := (4096 - 512) + int(segH.Length)<<12
	// fmt.Println(size)
	data := make([]byte, size)
	// fmt.Println(data)
	f.Seek(startByte+512, 0)
	// fmt.Println(data)
	// fmt.Println("seek dat")
	f.Read(data)
	// fmt.Println(data)
	f.Close()

	// fmt.Println("read dat")
	// fmt.Println(data)

	computed := checkSum(data)
	fmt.Println(computed)

	if computed == segH.Checksum {
		os.Exit(0)
	} else {
		os.Exit(1)
	}
}
