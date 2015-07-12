// TODO lisence

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
