// TODO lisence

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
