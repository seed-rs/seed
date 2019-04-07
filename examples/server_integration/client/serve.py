#!/usr/bin/env python3

import http.server
import os
import socketserver
import urllib
import sys

PORT = 8000


class Handler(http.server.SimpleHTTPRequestHandler):
    # Allow SPA routing by redirecting subpaths.
    def do_GET(self):
        urlparts = urllib.parse.urlparse(self.path)
        request_file_path = urlparts.path.strip('/')
        if not os.path.exists(request_file_path):
            self.path = '/'

        return http.server.SimpleHTTPRequestHandler.do_GET(self)


handler = Handler
# Add support for the WASM mime type.
handler.extensions_map.update({
    '.wasm': 'application/wasm',
})

socketserver.TCPServer.allow_reuse_address = True
print("Serving at port", PORT)
print("View at: http://localhost:{}/".format(PORT))
# The context manager protocol is support only since python 3.6 and higher.
if (3 <= sys.version_info[0]) and (6 <= sys.version_info[1]):
    with socketserver.TCPServer(("", PORT), handler) as httpd:
        httpd.allow_reuse_address = True
        httpd.serve_forever()
else:
    httpd = socketserver.TCPServer(("", PORT), handler)
    httpd.allow_reuse_address = True
    httpd.serve_forever()
    httpd.serve_close()
