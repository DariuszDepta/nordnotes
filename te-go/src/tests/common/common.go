package common

import (
	"fmt"
	o "github.com/EngosSoftware/oxyde"
)

const (
	NordnotesURL     = "http://0.0.0.0:8871"
	MappingNordnotes = "/api/{apiVersion}"
	ApiVersion1      = "v1"
)

// ErrorDto defines attributes of the error.
type ErrorDto struct {
	Details string `json:"details" api:"Detailed description of the error."`
}

// NORDNOTES returns initialized context for nordnotes application testing.
func NORDNOTES() o.Context {
	return o.Context{Url: NordnotesURL, Version: ApiVersion1, Verbose: false}
}

// DisplayOK prints to standard output the text 'OK' followed by newline character.
func DisplayOK() {
	fmt.Println("OK")
}
