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
	"io/ioutil"
	"os"
	"strings"
)

// TODO
// -h (help)

func main() {
	bytes, _ := ioutil.ReadAll(os.Stdin)
	toks := strings.Split(string(bytes), " ")

	fmt.Println("cursor pos:", toks[3])
	fmt.Println("# of cache blocks:", toks[4])
	fmt.Println("# of segments:", toks[5])
	fmt.Println("current id:", toks[6])
	fmt.Println("last flushed id:", toks[7])
	fmt.Println("last writeback id:", toks[8])
	fmt.Println("# of dirty cache blocks:", toks[9])
	fmt.Println("# of partial flushes:", toks[26])

	fmt.Println("write? hit? on_buffer? fullsize?")
	for i := 0; i < 16; i++ {
		p := func(a int, b uint) int {
			if (a & (1 << b)) > 0 {
				return 1
			} else {
				return 0
			}
		}

		v := toks[10+i]
		fmt.Println(p(i, 3), p(i, 2), p(i, 1), p(i, 0), ":", v)
	}

}
