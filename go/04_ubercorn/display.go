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

var red = Colour{255, 0, 0}
var green = Colour{0, 255, 0}
var blue = Colour{0, 0, 255}
var black = Colour{0, 0, 0}

type Display struct {
	connector spi.Conn
	numPix    int
	buffer    []byte
}

const (
	speed  = 9 * physic.MegaHertz
	bits   = 8
	prefix = 0x72
)

func InitDisplay(numPix int) Display {
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

	conn, _ := port.Connect(speed, spi.Mode0, bits)

	buffSize := numPix*3 + 1
	buff := make([]byte, buffSize)
	buff[0] = prefix

	return Display{
		conn,
		numPix,
		buff,
	}
}

func (d *Display) Update(idx int, c Colour) {
	pos := idx*3 + 1
	d.buffer[pos] = c.r
	d.buffer[pos+1] = c.g
	d.buffer[pos+2] = c.b
}

func (d *Display) Fill(c Colour) {
	for i := 0; i < d.numPix; i++ {
		d.Update(i, c)
	}
}

func (d *Display) Tx() {
	d.connector.Tx(d.buffer, nil)
}
