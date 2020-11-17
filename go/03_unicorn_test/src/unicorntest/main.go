package unicorntest

import (
	"fmt"
	"log"
	"time"

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

const (
	height = 16
	width  = 16
	speed  = 9 * physic.MegaHertz
	bits   = 8
	prefix = 0x72
)

func main() {
	if _, err := host.Init(); err != nil {
		log.Fatal(err)
	}
	port, err := spireg.Open("SPI0.0")

	if err != nil {
		fmt.Println(err)
	}
	if port != nil {
		fmt.Println(port)
	}

	connector, _ := port.Connect(speed, spi.Mode0, bits)

	var px_arr [256*3 + 1]Colour
	buffer := make([]byte, len(px_arr))

	red := newColour(255, 0, 0)
	for i := 0; i < 256; i++ {
		px_arr[i] = red
	}
	buffer[0] = prefix
	for i := 0; i < 256; i++ {
		k := 3*i + 1
		copy(buffer[k:], px_arr[i].asSlice())
	}
	connector.Tx(buffer, nil)

	time.Sleep(5 * time.Second)

	green := newColour(0, 255, 0)
	for i := 0; i < 256; i++ {
		px_arr[i] = green
	}
	buffer[0] = prefix
	for i := 0; i < width*height; i++ {
		k := 3*i + 1
		copy(buffer[k:], px_arr[i].asSlice())
	}
	connector.Tx(buffer, nil)

	time.Sleep(5 * time.Second)

	blue := newColour(0, 0, 255)
	for i := 0; i < 256; i++ {
		px_arr[i] = blue
	}
	buffer[0] = prefix
	for i := 0; i < width*height; i++ {
		k := 3*i + 1
		copy(buffer[k:], px_arr[i].asSlice())
	}
	connector.Tx(buffer, nil)

	time.Sleep(5 * time.Second)

	black := newColour(0, 0, 0)
	for i := 0; i < 256; i++ {
		px_arr[i] = black
	}
	buffer[0] = prefix
	for i := 0; i < width*height; i++ {
		k := 3*i + 1
		copy(buffer[k:], px_arr[i].asSlice())
	}
	connector.Tx(buffer, nil)
}
