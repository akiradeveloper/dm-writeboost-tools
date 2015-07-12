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
	"os"
	"os/exec"
	"strconv"

	"./lib"
)

// TODO -h (help)
// TODO -s override base id

func main() {
	dev := lib.NewDev(os.Args[1])
	mb_idx, _ := strconv.Atoi(os.Args[2])

	var base_id int = 1
	// override if specified

	id := base_id + mb_idx/127
	idx_inseg := mb_idx % 127

	startByte := dev.CalcSegmentStart(id)<<9 + (1+idx_inseg)<<12
	out, _ := exec.Command("od", fmt.Sprintf("-j%d", startByte), "-N4096", "-Ax", dev.Name()).Output()
	fmt.Println(string(out))
}
