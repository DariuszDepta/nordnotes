package suites

import (
	o "github.com/EngosSoftware/oxyde"
	n "tests/cases/nordnotes"
	c "tests/common"
)

func Nordnotes(ctx *o.Context, dtx *o.DocContext) {
	dtx.Clear()
	f := func(version string) {
		ctx.Version = version
		o.Display2(ctx)
		n.TsService(ctx, dtx)
	}
	f(c.ApiVersion1)
	o.StartPreview(dtx)
}
