package lib

import (
	"encoding/binary"
	"io"
	"io/ioutil"
	"os/exec"
	"strconv"
	"strings"
)

type BlockDevice struct {
	name string
}

func NewDev(name string) *BlockDevice {
	return &BlockDevice{name}
}

func (this *BlockDevice) Name() string {
	return this.name
}

func (this *BlockDevice) Size() int {
	out, _ := exec.Command("blockdev", "--getsize", this.name).Output()
	i, _ := strconv.Atoi(strings.TrimRight(string(out), "\n"))
	return i
}

func (this *BlockDevice) NrSegments() int {
	return (this.Size() - (1 << 11)) / (1 << 10)
}

func (this *BlockDevice) CalcSegmentStart(id int) int {
	idx := (id - 1) % this.NrSegments()
	return (1 << 11) + idx*(1<<10)
}

type SegmentHeader struct {
	Id       uint64
	Checksum uint32
	Length   uint8
	// 512 - (8 + 4 + 1)
}

func ReadSegmentHeader(f io.Reader) SegmentHeader {
	var v SegmentHeader
	binary.Read(f, binary.LittleEndian, &v.Id)
	binary.Read(f, binary.LittleEndian, &v.Checksum)
	binary.Read(f, binary.LittleEndian, &v.Length)
	io.CopyN(ioutil.Discard, f, 512-(8+4+1))
	return v
}
