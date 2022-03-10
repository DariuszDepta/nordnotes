from common.result import data


class SystemController:

    @staticmethod
    def on_get(_req, resp):
        """Handles GET request for system controller."""
        system_info = {
            'name': 'nordnotes',
            'version': '1.0.0',
            'legalNote': 'Copyright © 2022 Dariusz Depta Engos Software'
        }
        resp.media = data(system_info)
