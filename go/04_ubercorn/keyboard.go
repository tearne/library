package main

import (
	"bytes"
	"encoding/binary"
	"fmt"
	"os"
	"syscall"
	"unsafe"
)

type KeyLogger struct {
	fd *os.File
}

const (
	// EvSyn is used as markers to separate events. Events may be separated in time or in space, such as with the multitouch protocol.
	EvSyn EventType = 0x00
	// EvKey is used to describe state changes of keyboards, buttons, or other key-like devices.
	EvKey EventType = 0x01
	// EvRel is used to describe relative axis value changes, e.g. moving the mouse 5 units to the left.
	EvRel EventType = 0x02
	// EvAbs is used to describe absolute axis value changes, e.g. describing the coordinates of a touch on a touchscreen.
	EvAbs EventType = 0x03
	// EvMsc is used to describe miscellaneous input data that do not fit into other types.
	EvMsc EventType = 0x04
	// EvSw is used to describe binary state input switches.
	EvSw EventType = 0x05
	// EvLed is used to turn LEDs on devices on and off.
	EvLed EventType = 0x11
	// EvSnd is used to output sound to devices.
	EvSnd EventType = 0x12
	// EvRep is used for autorepeating devices.
	EvRep EventType = 0x14
	// EvFf is used to send force feedback commands to an input device.
	EvFf EventType = 0x15
	// EvPwr is a special type for power button and switch input.
	EvPwr EventType = 0x16
	// EvFfStatus is used to receive force feedback device status.
	EvFfStatus EventType = 0x17
)

type EventType uint16

var eventsize = int(unsafe.Sizeof(InputEvent{}))

type InputEvent struct {
	Time  syscall.Timeval
	Type  EventType
	Code  uint16
	Value int32
}

func (k *KeyLogger) read() (*InputEvent, error) {
	buffer := make([]byte, eventsize)
	n, err := k.fd.Read(buffer)
	if err != nil {
		return nil, err
	}
	if n <= 0 {
		return nil, nil
	}

	event := &InputEvent{}
	err = binary.Read(bytes.NewBuffer(buffer), binary.LittleEndian, event)
	return event, err
}

func NewKeyWatcher() chan InputEvent {
	k := &KeyLogger{}
	fd, err := os.Open("/dev/input/event0")
	if err != nil {
		fmt.Println(err)
	}
	//fmt.Printf("Device is %+v\n", *fd.)
	//fmt.Printf("Device is: ", spew.Sdump(*fd))
	k.fd = fd

	out := make(chan InputEvent)
	go func(event chan InputEvent) {
		for {
			e, err := k.read()
			if err != nil {
				fmt.Println(err)
				close(event)
				break
			}

			if e != nil {
				event <- *e
			}
		}
	}(out)

	return out
}
