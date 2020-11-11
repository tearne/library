package main

import (
	"fmt"
	"log"

	"periph.io/x/periph/conn/physic"
	"periph.io/x/periph/conn/spi"
	"periph.io/x/periph/conn/spi/spireg"
	"periph.io/x/periph/host"
)

type Colour struct {
	r byte
	g byte
	b byte
}

func newColour(r uint8, g uint8, b uint8) Colour {
	return Colour{
		r, g, b,
	}
}

func (c Colour) asSlice() []uint8 {
	return []uint8{c.r, c.g, c.b}
}

func main() {
	var px [256]Colour
	for i := 0; i < 256; i++ {
		px[i] = newColour(uint8(i%256), uint8((i*i)%256), uint8(i%256))
	}

	if _, err := host.Init(); err != nil {
		log.Fatal(err)
	}

	const (
		height = 16
		width  = 16
		speed  = 9 * physic.MegaHertz
		bits   = 8
		prefix = 0x72
	)

	port, err := spireg.Open("SPI0.0")

	if err != nil {
		fmt.Println(err)
	}
	if port != nil {
		fmt.Println(port)
	}

	connector, _ := port.Connect(speed, spi.Mode0, bits)

	buffer := make([]byte, width*height*3+1)
	buffer[0] = prefix

	for i := 0; i < width*height; i++ {
		k := 3*i + 1
		copy(buffer[k:], px[i].asSlice())
	}

	connector.Tx(buffer, nil)
}
