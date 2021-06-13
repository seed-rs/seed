use js_sys::Reflect;
use seed::prelude::*;

#[macro_export]
macro_rules! sl_input {
    ( $($part:expr),* $(,)? ) => {
        {
            custom![
                Tag::from("sl-input"),
                $( $part )*
            ]
        }
    };
}

pub fn on_input<Ms: 'static>(
    handler: impl FnOnce(String) -> Ms + Clone + 'static,
) -> EventHandler<Ms> {
    ev(Ev::from("sl-input"), |event| {
        let event_target = event.target().unwrap();
        let property_name = JsValue::from("value");
        let value = Reflect::get(&event_target, &property_name).unwrap();
        handler(value.as_string().unwrap())
    })
}
