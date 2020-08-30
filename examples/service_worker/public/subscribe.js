// ------ ------
//  Subscribe
//  @manager: web_sys::PushManager - The PushManager interface of the Push API provides a way to
//            receive notifications from third-party servers as well as request URLs for push notifications.
//            - https://developer.mozilla.org/en-US/docs/Web/API/PushManager
//            - https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.PushManager.html
//  @api_key: Uint8Array - A Base64-encoded DOMString or ArrayBuffer containing an ECDSA P-256 public key that 
//            the push server will use to authenticate your application server.
//            - https://developer.mozilla.org/en-US/docs/Web/API/PushManager/subscribe
// @returns Promise<web_sys::PushSubscription>
// Until https://github.com/rustwasm/wasm-bindgen/pull/2288 is included in the new release of wasm-bindgen,
// this function should be called from the seed app to subscribe to the PushManager. The applicationServerKey 
// is the vapid key that is used for identification on the back-end server. The `userVisibleOnly` property
// indicates that the returned push subscription will only be used for messages whose effect is made visible 
// to the user. It must be set to `true` or the browser will reject the subscription request. This holds true
// for both chrome and firefox.
// ------ ------
window.subscribe = async (manager, api_key) => {
    let subscription = await manager.subscribe({
        applicationServerKey: api_key,
        userVisibleOnly: true
    });

    console.log("JS subscription", subscription);
    return subscription;
}
