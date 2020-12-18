package main

import (
	"fmt"
	"io/ioutil"
	"path"
	"path/filepath"
	"os"
)

// https://yourbasic.org/golang/current-directory/

// Run different ways to see difference
//  -  go run relative-load.go
//  -  go build -o target/app relative-load.go && target/app

func main() {
	var err error

	// Working dir
	pwd, err := os.Getwd()
	if err != nil {
		panic(err)
	}
	fmt.Printf("Your working dir is:\n  %v\n", pwd)

	// Executable dir
	ex, err := os.Executable()
	if err != nil {
		panic(err)
	}
	exPath := filepath.Dir(ex)
	fmt.Printf("The executable is located at:\n  %v\n",exPath)

	// Load file from ralative path, assuming working dir is project root
	file_path := path.Join(pwd, "config", "data.txt")
	fmt.Printf("Data is at %v\n", file_path)
	bytes, err := ioutil.ReadFile(file_path)
	if err != nil {
		panic(err)
	}
	fmt.Printf("  Contents: %v\n", string(bytes))
}