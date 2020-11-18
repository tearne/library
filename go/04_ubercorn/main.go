package main

func main() {
	panel := InitDisplay(256)
	panel.Fill(Colour{255, 50, 230})
	panel.Tx()
}
