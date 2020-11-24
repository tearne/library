package main

import (
	"time"

	"github.com/davecgh/go-spew/spew"
)

func main() {
	panel := InitDisplay(256)

	panel.Fill(Colour{255, 0, 0})
	panel.Tx()
	time.Sleep(5 * time.Second)

	panel.Fill(Colour{0, 255, 0})
	panel.Tx()
	time.Sleep(5 * time.Second)

	panel.Fill(Colour{0, 0, 255})
	panel.Tx()
	time.Sleep(5 * time.Second)

	panel.Fill(black)
	panel.Tx()

	watcher := NewKeyWatcher()
	for event := range watcher {
		spew.Dump(event)
	}
}
