package common

const (
	NordnotesURL     = "http://0.0.0.0:8085"
	ApiVersion       = "v1"
	MappingApiRest   = "/api/{apiVersion}"
	MappingNordnotes = MappingApiRest
)

type Context struct {
	Url      string // URL of the tested endpoint.
	Token    string // Authorization token.
	Login    string // User's login.
	RoleName string // Name of the user's role.
	Verbose  bool   // Flag indicating if the details of the request should be printed.
	Version  string // Version of the called API.
}
