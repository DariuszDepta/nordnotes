package common

import (
	o "github.com/EngosSoftware/oxyde"
)

const (
	NordnotesURL     = "http://0.0.0.0:8871"
	MappingNordnotes = "/api/{apiVersion}"
	ApiVersion1      = "v1"
)

// ErrorDto defines attributes of the error returned from failing request.
type ErrorDto struct {
	Details string `json:"details" api:"Detailed description of the error."`
}

// NORDNOTES returns initialized context for nordnotes application testing.
func NORDNOTES() o.Context {
	return o.Context{Url: NordnotesURL, Version: ApiVersion1, Verbose: false}
}
