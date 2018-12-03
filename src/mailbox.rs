use futures::Future;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::future_to_promise;
use web_sys as web;

pub struct Mailbox<Message: 'static> {
    func: Rc<Fn(Message)>,
}

impl<Message: 'static> Mailbox<Message> {
    pub fn new(func: impl Fn(Message) + 'static) -> Self {
        Mailbox {
            func: Rc::new(func),
        }
    }

    pub fn send(&self, message: Message) {
        crate::log("In mailbox crate's send");
        (self.func)(message)
    }

    pub fn send_after(&self, timeout: i32, f: impl Fn() -> Message + 'static) {
        crate::log("In send_after");
        let cloned = self.clone();
        let closure = Closure::wrap(Box::new(move || {
            cloned.send(f());
        }) as Box<FnMut()>);
        web::window()
            .unwrap()
            .set_timeout_with_callback_and_timeout_and_arguments_0(
                closure.as_ref().unchecked_ref(),
                timeout,
            )
            .unwrap();
        // TODO: Stash this closure in the Mailbox and drop it when the closure is first called.
        closure.forget();
    }

//    pub fn subscribe<S: Subscription + 'static>(
//        &self,
//        subscription: S,
//        f: impl Fn(S::Message) -> Message + 'static,
//    ) -> Unsubscribe {
//        crate::log("In subs");
//        let cloned = self.clone();
//        subscription.subscribe(Rc::new(move |message| cloned.send(f(message))))
//    }

    pub fn map<NewMessage: 'static>(
        self,
        f: impl Fn(NewMessage) -> Message + 'static,
    ) -> Mailbox<NewMessage> {
        Mailbox {
            func: Rc::new(move |message| (self.func)(f(message))),
        }
    }

    pub fn spawn<F>(&self, future: F, func: impl Fn(Result<F::Item, F::Error>) -> Message + 'static)
    where
        F: Future + 'static,
    {
        let cloned = self.clone();
        let future = future.then(move |result| {
            cloned.send(func(result));
            futures::future::ok(wasm_bindgen::JsValue::UNDEFINED)
        });
        future_to_promise(future);
    }
}

impl<Message> Clone for Mailbox<Message> {
    fn clone(&self) -> Self {
        Mailbox {
            func: self.func.clone(),
        }
    }
}

impl<Message> std::fmt::Debug for Mailbox<Message> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("Mailbox").finish()
    }
}
