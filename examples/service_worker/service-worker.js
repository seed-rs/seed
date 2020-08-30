importScripts('https://storage.googleapis.com/workbox-cdn/releases/5.1.2/workbox-sw.js');

// workbox.core - Provides core workbox functionality. Ths will be used for service worker updating.
const core = workbox.core;

// workbox.precaching - Helps to simplify the caching process
const precaching = workbox.precaching;

// We want to publish a new service worker and immediately update and control the page.
// - https://developers.google.com/web/tools/workbox/modules/workbox-core#skip_waiting_and_clients_claim
core.skipWaiting();
core.clientsClaim();

// Cache all of the assets for offline viewing. This can be done manually or by using a tool, such as 
// `workbox-cli`- https://developers.google.com/web/tools/workbox/modules/workbox-cli.
// By updating the revision hash after an asset has been updated, the cached resource will be
// updated in the browser's cache.
precaching.precacheAndRoute(
  [
    { "revision": "12345", "url": "index.html" },
    { "revision": "12345", "url": "public/subscribe.js" },
    { "revision": "12345", "url": "public/images/important-notes.png" },
    { "revision": "12345", "url": "/" },
    { "revision": "12345", "url": "pkg/package_bg.wasm" }, 
    { "revision": "12345", "url": "pkg/package.js" },
  ]
);

// Listen for and display a push notification if the push event is triggered from the server.
self.addEventListener('push', (event) => {
  const title = 'Seed service worker!';
  const options = {
    body: event.data.text()
  };
  event.waitUntil(self.registration.showNotification(title, options));
});
