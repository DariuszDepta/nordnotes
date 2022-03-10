package nordnotes

import (
	o "github.com/EngosSoftware/oxyde"
	c "tests/common"
)

const (
	ServiceUrl       = c.MappingNordnotes + "/info"
	ServiceDocTag    = "system"
	ServiceName      = "nordnotes"
	ServiceVersion   = "1.0.0"
	ServiceLegalNote = "Copyright © 2022 Dariusz Depta Engos Software"
)

type ServiceInfoDto struct {
	Name      string `json:"name"      api:"Name of the system."`
	Version   string `json:"version"   api:"Version of the system."`
	LegalNote string `json:"legalNote" api:"Legal note."`
}

type ServiceInfoResult struct {
	Data   ServiceInfoDto `json:"data"    api:"?Data object with system information."`
	Errors []c.ErrorDto   `json:"errors"  api:"?List of errors when request fails."`
}

func TsService(ctx *o.Context, dtx *o.DocContext) {
	o.Display(ctx)
	TdServiceInfo(ctx, dtx)
}

func TdServiceInfo(ctx *o.Context, dtx *o.DocContext) {
	o.Display(ctx)
	dtx.NewEndpoint(ctx.Version, ServiceDocTag, `System information`, "")
	TcServiceInfo(ctx, dtx)
	dtx.SaveEndpoint()
}

func TcServiceInfo(ctx *o.Context, dtx *o.DocContext) {
	o.Display(ctx)
	var result ServiceInfoResult
	dtx.CollectAll("System information", "")
	o.HttpGET(ctx, dtx, ServiceUrl, nil, nil, &result, 200)
	AssertServiceInfo(result)
	c.DisplayOK()
}

func AssertServiceInfo(result ServiceInfoResult) {
	o.AssertEqualString(ServiceName, result.Data.Name)
	o.AssertEqualString(ServiceVersion, result.Data.Version)
	o.AssertEqualString(ServiceLegalNote, result.Data.LegalNote)
}
