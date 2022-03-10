package main

import "os"

func main() {
	if len(os.Args) == 2 {
		switch os.Args[1] {
		case "ALL":
			println("testing")
		}
	} else {
		println("no input arguments specified")
	}
}
