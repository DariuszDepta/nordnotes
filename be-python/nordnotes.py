import falcon

from controllers.system_controller import SystemController

api = '/api'
version = '/v1'
api_version = api + version

app = falcon.App()
app.add_route(api_version + '/info', SystemController())
