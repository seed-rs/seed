# Service Worker example

Service worker is an exciting technology that is available in most major browsers. To summarize, it can be used to cache
assets, providing a positive offline experience. Additionally, service worker has the ability to send notifications that
are generated both locally from a web application as well as from a remote server.

The example in this crate demonstrates the following features:

1. Use service worker to cache all assets (including the generated wasm file).
1. Register the service worker.
1. If the service worker is not yet activated, an even listener will be registered, waiting for the
   state to reach "activated".
1. When the state reaches "activated", the Notification object will request permission for notifications.
1. If permission is granted, the PushManager will subscribe to the service using an example vapid key.
1. Finally, a PushSubscription will be returned, containing the information that can be passed to a
   notification back-end server.

---

## Prerequisites - Openssl Installation

The server requires that openssl be installed. For OS X systems, openssl can be installed using [`Homebrew`](https://brew.sh/).
For windows, openssl can be installed by running the following:

1. `git clone https://github.com/microsoft/vcpkg`
1. cd vcpkg
1. ./bootstrap-vcpkg.bat
1. ./vcpkg integrate install
1. ./vcpkg install openssl-windows:x64-windows
1. ./vcpkg install openssl:x64-windows-static-md

## Running

Running this example requires both a server and a client to be running at the same time. This is easily achieved by running each
in its own terminal from the root of the seed project:

Terminal A:

```bash
cargo make start_server service_worker
```

Terminal B:

```bash
cargo make start service_worker
```

1. Open [http://127.0.0.1:8000/](http://127.0.0.1:8000/) in your browser. This will cache the assets.
1. Kill the running cargo process to terminate the local dev server.
1. Refresh the browser and notice that the page loads with all assets.
1. Click the `Request Push Subscription` button. This will register the subscription with the Push Manager.
1. Click on the `Send Push Notification` button. A notification should pop up on the browser.
1. Open Dev Tools and select the `Application` tab.
1. Click on the `Service Workers` item and view the registered service worker.
1. In the web page, click the `Unregister Service Worker` button and notice that the service worker is unregistered.
1. View the `Cache Storage` item in dev tools and take note of the cached items.
1. In the web page, click the `Clear Cache` button and notice the cache is erased in Dev Tools (Firefox is a bit finicky at showing this).
