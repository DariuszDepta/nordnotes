package main

import (
	"fmt"
	o "github.com/EngosSoftware/oxyde"
	"os"
	c "tests/common"
	"tests/suites"
)

// Function main is an entrypoint for all tests.
func main() {
	if len(os.Args) == 2 {
		switch os.Args[1] {
		case "ALL":
			ctx := c.NORDNOTES()
			dtx := o.CreateDocContext()
			suites.Nordnotes(&ctx, dtx)
		}
	} else {
		fmt.Println("no input arguments specified")
	}
}
