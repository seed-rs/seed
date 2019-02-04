#!/usr/bin/env python3

import http.server
import os
import socketserver
import urllib

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
with socketserver.TCPServer(("", PORT), handler) as httpd:
    httpd.allow_reuse_address = True
    print("Serving at port", PORT)
    httpd.serve_forever()
