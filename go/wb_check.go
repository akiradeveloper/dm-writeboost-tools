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
	"hash/crc32"
	"os"
	"strconv"

	"./lib"
)

func checkSum(data []byte) uint32 {
	table := crc32.MakeTable(crc32.Castagnoli)
	return crc32.Update(0, table, data)
}

func main() {
	dev := lib.NewDev(os.Args[1])
	id, _ := strconv.Atoi(os.Args[2])

	f, _ := os.Open(dev.Name())

	startByte := int64(dev.CalcSegmentStart(id)) << 9
	f.Seek(startByte, 0)
	segH := lib.ReadSegmentHeader(f)

	size := (4096 - 512) + int(segH.Length)<<12
	data := make([]byte, size)
	f.Seek(startByte+512, 0)
	f.Read(data)
	f.Close()

	computed := checkSum(data)

	if computed == segH.Checksum {
		os.Exit(0)
	} else {
		os.Exit(1)
	}
}
