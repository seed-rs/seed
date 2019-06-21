//! This module contains code related to event handling; ie things that update the dom, related to
//! `web_sys::Event`

use crate::{dom_types::MessageMapper, util};

use serde::de::DeserializeOwned;
use std::{fmt, mem};
use wasm_bindgen::{closure::Closure, JsCast};
pub type Event = web_sys::Event;

pub const UPDATE_TRIGGER_EVENT_ID: &str = "triggerupdate";

/// Similar to tag population.
macro_rules! make_events {
    // Create shortcut macros for any element; populate these functions in this module.
    { $($event_camel:ident => $event:expr),+ } => {

        /// The Ev enum restricts element-creation to only valid event names, as defined here:
        /// [https://developer.mozilla.org/en-US/docs/Web/Evs](https://developer.mozilla.org/en-US/docs/Web/Evs)
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum Ev {
            $(
                $event_camel,
            )+
        }

        impl Ev {
            pub fn as_str(&self) -> &str {
                match self {
                    $ (
                        Ev::$event_camel => $event,
                    ) +
                }
            }
        }

        impl From<&str> for Ev {
            fn from(event: &str) -> Self {
                match event {
                    $ (
                          $event => Ev::$event_camel,
                    ) +
                    _ => {
                        crate::log(&format!("Can't find this event: {}", event));
                        Ev::Click
                    }
                }
            }
        }

        impl From<String> for Ev {
            fn from(event: String) -> Self {
                match event.as_ref(){
                    $ (
                          $event => Ev::$event_camel,
                    ) +
                    _ => {
                        crate::log(&format!("Can't find this event: {}", event));
                        Ev::Click
                    }
                }
            }
        }

        impl ToString for Ev {
            fn to_string( & self ) -> String {
                match self {
                    $ (
                        Ev::$ event_camel => $ event.into(),
                    ) +

                }
            }
        }
    }
}

// Comprehensive list: https://developer.mozilla.org/en-US/docs/Web/Events
make_events! {
    Cached => "cached", Error => "error", Abort => "abort", Load => "load", BeforeUnload => "beforeunload",
    Unload => "unload", Online => "online", Offline => "offline", Focus => "focus", Blur => "blur",
    Open => "open", Message => "message", Close => "close", PageHide => "pagehide",
    PageShow => "pageshow", PopState => "popstate", AnimationStart => "animationstart", AnimationEnd => "animationend",
    AnimationIteration => "animationiteration", TransitionStart => "transtionstart", TransitionEnd => "transitionend",
    TranstionRun => "transitionrun",

    Rest => "rest", Submit => "submit", BeforePrint => "beforeprint", AfterPrint => "afterprint",
    CompositionStart => "compositionstart", CompositionUpdate => "compositionupdate", CompositionEnd => "compositionend",

    FullScreenChange => "fullscreenchange", FullScreenError => "fullscreenerror", Resize => "resize",
    Scroll => "scroll", Cut => "cut", Copy => "copy", Paste => "paste",

    KeyDown => "keydown", KeyUp => "keyup",
    KeyPress => "keypress", AuxClick => "auxclick", Click => "click", ContextMenu => "contextmenu", DblClick => "dblclick",
    MouseDown => "mousedown", MouseEnter => "mouseenter", MouseLeave => "mouseleave",
    MouseMove => "mousemove", MouseOver => "mouseover", MouseOut => "mouseout", MouseUp => "mouseup",
    PointerLockChange => "pointerlockchange", PointerLockError => "pointerlockerror", Select => "select",
    Wheel => "wheel",

    PointerOver => "pointerover", PointerEnter => "pointerenter",
    PointerDown => "pointerdown", PointerMove => "pointermove", PointerUp => "pointerup",
    PointerCancel => "pointercancel", PointerOut => "pointerout", PointerLeave => "pointerleave",
    GotPointerCapture => "gotpointercapture", LostPointerCapture => "lostpointercapture",

    Drag => "drag", DragEnd => "dragend", DragEnter => "dragenter", DragStart => "dragstart", DragLeave => "dragleave",
    DragOver => "dragover", Drop => "drop",

    AudioProcess => "audioprocess", CanPlay => "canplay", CanPlayThrough => "canplaythrough", Complete => "complete",
    DurationChange => "durationchange", Emptied => "emptied", Ended => "ended", LoadedData => "loadeddata",
    LoadedMetaData => "loadedmetadata", Pause => "pause", Play => "play", Playing => "playing", RateChange => "ratechange",
    Seeked => "seeked", Seeking => "seeking", Stalled => "stalled", Suspend => "suspend", TimeUpdate => "timeupdate",
    VolumeChange => "volumechange",

    // todo finish this

    Change => "change",

    Input => "input",

    TriggerUpdate => "triggerupdate"
}

type EventHandler<Ms> = Box<FnMut(web_sys::Event) -> Ms>;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Category {
    Custom,
    Input,
    Keyboard,
    Mouse,
    Pointer,
    Raw,
    Simple,
}

/// Ev-handling for Elements
pub struct Listener<Ms> {
    pub trigger: Ev,
    // Handler describes how to handle the event, and is used to generate the closure.
    pub handler: Option<EventHandler<Ms>>,
    // We store closure here so we can detach it later.
    pub closure: Option<Closure<FnMut(web_sys::Event)>>,
    // Control listeners prevent input on controlled input elements, and
    // are not assoicated with a message.
    pub control_val: Option<String>,
    pub control_checked: Option<bool>,

    // category and message are used as an aid for comparing Listeners, and therefore diffing.
    // todo: Neither are fully implemented.
    category: Option<Category>,
    // An associated message, if applicable.
    message: Option<Ms>,
}

impl<Ms> fmt::Debug for Listener<Ms> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Listener {{ trigger:{:#?}, handler:{:#?}, closure:{:#?}, control:{:#?}{:#?}, category:{:#?}",
            self.trigger,
            fmt_hook_fn(&self.handler),
            fmt_hook_fn(&self.closure),
            self.control_val,
            self.control_checked,
            self.category,
        )
    }
}

impl<Ms> Listener<Ms> {
    pub fn new(
        trigger: &str,
        handler: Option<EventHandler<Ms>>,
        category: Option<Category>,
        message: Option<Ms>,
    ) -> Self {
        Self {
            // We use &str instead of Event here to allow flexibility in helper funcs,
            // without macros by using ToString.
            trigger: trigger.into(),
            handler,
            closure: None,
            control_val: None,
            control_checked: None,
            category,
            message,
        }
    }

    /// Set up a listener that keeps the field's value in sync with the specific value,
    /// from the model
    pub fn new_control(val: String) -> Self {
        Self {
            trigger: Ev::Input,
            handler: None,
            closure: None,
            control_val: Some(val),
            control_checked: None,
            category: None,
            message: None,
        }
    }

    /// Similar to new_control, but for checkboxes
    pub fn new_control_check(checked: bool) -> Self {
        Self {
            trigger: Ev::Click,
            handler: None,
            closure: None,
            control_val: None,
            control_checked: Some(checked),
            category: None,
            message: None,
        }
    }

    /// This method is where the processing logic for events happens.
    pub fn attach<T>(&mut self, el_ws: &T, mailbox: crate::vdom::Mailbox<Ms>)
    where
        T: AsRef<web_sys::EventTarget>,
    {
        // This and detach taken from Draco.
        let mut handler = self.handler.take().expect("Can't find old handler");
        let trigger = self.trigger;
        // This is the closure ran when a DOM element has an user defined callback
        let closure = Closure::wrap(Box::new(move |event: web_sys::Event| {
            // Let the seed user handle the event
            let msg = handler(event.clone());
            mailbox.send(msg);
            // update the value field if needed
            // The update is needed when the event is of type input, the input field is
            // of type number, text or password and the DOM value field is different from
            // the default_value. Default value is the one set by the seed user when he sets the
            // value, setting the value in HTML only changes the default value, not
            // the actual value. To change the actual value it is necessary to call set_value
            if trigger == Ev::Input {
                let target = event.target().unwrap();

                if let Some(input_el) = target.dyn_ref::<web_sys::HtmlInputElement>() {
                    let input_type = input_el.type_();
                    // For number, text and password, might be useful for other inputs too
                    // but breaks the file input for example, which cannot have its value
                    // set by using the set_value function
                    let should_trigger_rerender_with_set_value = {
                        (input_type == "number" || input_type == "text" || input_type == "password")
                    };
                    if should_trigger_rerender_with_set_value {
                        let value_set_by_seed_user = input_el.default_value();
                        let actual_value = input_el.value();
                        if value_set_by_seed_user != actual_value {
                            input_el.set_value(&value_set_by_seed_user);
                        }
                    }
                }
                //                            else if let Some(select_el) = target.dyn_ref::<web_sys::HtmlSelectElement>() {
                //                            let value_set_by_seed_user = select_el.default_value();
                //                            let actual_value = select_el.value();
                //                            if value_set_by_seed_user != actual_value {
                //                                select_el.set_value(&value_set_by_seed_user);
                //                            }
                //                        }
                // todo do we need to handle textarea separately?
                // todo should just get attach_control (below) working
            }
        }) as Box<FnMut(web_sys::Event) + 'static>);

        (el_ws.as_ref() as &web_sys::EventTarget)
            .add_event_listener_with_callback(
                self.trigger.as_str(),
                closure.as_ref().unchecked_ref(),
            )
            .expect("problem adding listener to element");

        // Store the closure so we can detach it later. Not detaching it when an element
        // is removed will trigger a panic.
        if self.closure.replace(closure).is_some() {
            panic!("self.closure already set in attach");
        }
    }

    //    /// This method is where the processing logic for events happens.
    //    pub fn attach<T>(&mut self, el_ws: &T, mailbox: crate::vdom::Mailbox<Ms>)
    //        where
    //            T: AsRef<web_sys::EventTarget>,
    //    {
    //        let mut handler = self.handler.take().expect("Can't find old handler");
    //
    //        // https://rustwasm.github.io/wasm-bindgen/api/wasm_bindgen/closure/struct.Closure.html
    //        let closure =
    //            Closure::wrap(
    //                Box::new(move |event: web_sys::Event| mailbox.send(handler(event)))
    //                    as Box<FnMut(web_sys::Event) + 'static>,
    //            );
    //
    //        (el_ws.as_ref() as &web_sys::EventTarget)
    //            .add_event_listener_with_callback(
    //                self.trigger.as_str(),
    //                closure.as_ref().unchecked_ref(),
    //            )
    //            .expect("problem adding listener to element");
    //
    //        // Store the closure so we can detach it later. Not detaching it when an element
    //        // is removed will trigger a panic.
    //        self.closure = Some(closure);
    //    }

    // todo: Note this func and the above commented-out code: This approach, of passing
    // todo control values from the model appears not to work, perhaps due to a clash between
    // todo user-inputted, and control Ev::input listeners. This solution will not keep
    // todo the model and field synced if the model changes due to somethign other
    // todo than input, or if a value attribute isn't specified.

    /// todo: Would like this in the same fn as attach, but run into issues
    /// between el_ws as EventTarget, and as Node. Could possibly resolve using traits.
    pub fn attach_control(&mut self, el_ws: &web_sys::Node) {
        // Dummy vars outside the closure to avoid lifetime problems.
        let val2 = self.control_val.clone();
        let checked2 = self.control_checked;
        let el_ws2 = el_ws.clone();
        let closure = Closure::wrap(Box::new(move |_| {
            if let Some(val) = val2.clone() {
                if util::get_value(&el_ws2) != val {
                    util::set_value(&el_ws2, &val);
                }
            }
            if let Some(checked) = checked2 {
                let input_el = &el_ws2
                    .dyn_ref::<web_sys::HtmlInputElement>()
                    .expect("Problem casting as checkbox");
                if input_el.checked() != checked {
                    input_el.set_checked(checked);
                }
            }
        }) as Box<FnMut(web_sys::Event) + 'static>);

        (el_ws.as_ref() as &web_sys::EventTarget)
            .add_event_listener_with_callback(
                self.trigger.as_str(),
                closure.as_ref().unchecked_ref(),
            )
            .expect("problem adding listener to element");

        // Store the closure so we can detach it later. Not detaching it when an element
        // is removed will trigger a panic.
        if self.closure.replace(closure).is_some() {
            panic!("self.closure already set in attach_control");
        }
    }

    pub fn detach<T>(&mut self, el_ws: &T)
    where
        T: AsRef<web_sys::EventTarget>,
    {
        let closure = self.closure.take().expect("Can't find closure to detach");

        (el_ws.as_ref() as &web_sys::EventTarget)
            .remove_event_listener_with_callback(
                self.trigger.as_str(),
                closure.as_ref().unchecked_ref(),
            )
            .expect("problem removing listener from element");
    }
}

impl<Ms> PartialEq for Listener<Ms> {
    fn eq(&self, other: &Self) -> bool {
        // Todo: This isn't (yet) a comprehensive check, but can catch some differences.
        self.trigger == other.trigger
            && self.category == other.category
            // We use discriminant so we don't have to force Ms to impl PartialEq.
            && mem::discriminant(&self.message) == mem::discriminant(&other.message)
    }
}

impl<Ms: 'static, OtherMs: 'static> MessageMapper<Ms, OtherMs> for Listener<Ms> {
    type SelfWithOtherMs = Listener<OtherMs>;
    fn map_message(self, f: fn(Ms) -> OtherMs) -> Listener<OtherMs> {
        Listener {
            trigger: self.trigger,
            handler: self.handler.map(|mut eh| {
                Box::new(move |event| {
                    let m = (*eh)(event);
                    (f)(m)
                }) as EventHandler<OtherMs>
            }),
            closure: self.closure,
            control_val: self.control_val,
            control_checked: self.control_checked,

            category: self.category,
            message: None,
        }
    }
}

/// Create an event that passes a String of field text, for fast input handling.
pub fn input_ev<Ms, T: ToString + Copy>(
    trigger: T,
    mut handler: impl FnMut(String) -> Ms + 'static,
) -> Listener<Ms> {
    let closure = move |event: web_sys::Event| {
        if let Some(target) = event.target() {
            return handler(util::get_value(&target));
        }
        handler(String::new())
    };

    Listener::new(
        &trigger.to_string(),
        Some(Box::new(closure)),
        Some(Category::Input),
        None,
    )
}

// todo: Attempt to get something of the below form working.
/// Create an event that passes a String of field text, for fast input handling.
//pub fn input_ev<Ms, T, Q>(trigger: T, message: Ms) -> Listener<Ms>
//    where
//        Ms: Clone + 'static,
//        T: ToString + Copy,
//{
//    let msg_closure = message.clone();
//    let mut handler = |text: String| msg_closure;
//
//    let closure = move |event: web_sys::Event| {
////        if let Some(target) = event.target() {
////            return handler.clone()(util::get_value(&target));
////        }
//        handler.clone()(String::new())
//    };
//
////    let handler = || msg_closure;
////    let closure = move |_| handler.clone()();
//
//    Listener::new(
//        &trigger.to_string(),
//        Some(Box::new(closure)),
//        Some(Category::Input),
//        Some(message),
//    )
//}

/// Create an event that passes a `web_sys::KeyboardEvent`, allowing easy access
/// to items like `key_code`() and key().
pub fn keyboard_ev<Ms: Clone, T: ToString + Copy>(
    trigger: T,
    mut handler: impl FnMut(web_sys::KeyboardEvent) -> Ms + 'static,
) -> Listener<Ms> {
    let closure = move |event: web_sys::Event| {
        handler(event.dyn_ref::<web_sys::KeyboardEvent>().unwrap().clone())
    };
    Listener::new(
        &trigger.to_string(),
        Some(Box::new(closure)),
        Some(Category::Keyboard),
        None,
    )
}

/// See `keyboard_ev`
pub fn mouse_ev<Ms: Clone, T: ToString + Copy>(
    trigger: T,
    mut handler: impl FnMut(web_sys::MouseEvent) -> Ms + 'static,
) -> Listener<Ms> {
    let closure = move |event: web_sys::Event| {
        handler(event.dyn_ref::<web_sys::MouseEvent>().unwrap().clone())
    };
    Listener::new(
        &trigger.to_string(),
        Some(Box::new(closure)),
        Some(Category::Mouse),
        None,
    )
}

/// See `keyboard_ev`
pub fn pointer_ev<Ms, T: ToString + Copy>(
    trigger: T,
    mut handler: impl FnMut(web_sys::PointerEvent) -> Ms + 'static,
) -> Listener<Ms> {
    let closure = move |event: web_sys::Event| {
        handler(event.dyn_ref::<web_sys::PointerEvent>().unwrap().clone())
    };
    Listener::new(
        &trigger.to_string(),
        Some(Box::new(closure)),
        Some(Category::Pointer),
        None,
    )
}

/// Create an event that accepts a closure, and passes a `web_sys::Event`, allowing full control of
/// event-handling
pub fn raw_ev<Ms, T: ToString + Copy>(
    trigger: T,
    mut handler: impl FnMut(web_sys::Event) -> Ms + 'static,
) -> Listener<Ms> {
    let closure = move |event: web_sys::Event| handler(event);
    Listener::new(
        &trigger.to_string(),
        Some(Box::new(closure)),
        Some(Category::Raw),
        None,
    )
}

/// Create an event that passes no data, other than it occurred. Foregoes using a closure,
/// in favor of pointing to a message directly.
pub fn simple_ev<Ms: Clone, T>(trigger: T, message: Ms) -> Listener<Ms>
where
    Ms: Clone + 'static,
    T: ToString + Copy,
{
    let msg_closure = message.clone();
    let handler = || msg_closure;
    let closure = move |_| handler.clone()();
    Listener::new(
        &trigger.to_string(),
        Some(Box::new(closure)),
        Some(Category::Simple),
        Some(message),
    )
}

/// Create an event that passes a `web_sys::CustomEvent`, allowing easy access
/// to detail() and then trigger update
pub fn trigger_update_ev<Ms: Clone>(
    mut handler: impl FnMut(web_sys::CustomEvent) -> Ms + 'static,
) -> Listener<Ms> {
    let closure = move |event: web_sys::Event| {
        handler(event.dyn_ref::<web_sys::CustomEvent>().unwrap().clone())
    };
    Listener::new(
        UPDATE_TRIGGER_EVENT_ID,
        Some(Box::new(closure)),
        Some(Category::Custom),
        None,
    )
}

pub(crate) fn fmt_hook_fn<T>(h: &Option<T>) -> &'static str {
    match h {
        Some(_) => "Some(.. a dynamic handler ..)",
        None => "None",
    }
}

///// Update app state directly, ie not from a Listener/event.
//pub fn update<Ms>() -> Listener<Ms> {
//    let closure = move |event: web_sys::Event| handler(event);
//    Listener::new(&trigger.to_string(), Some(Box::new(closure)))
//}

/// Trigger update function from outside of App
pub fn trigger_update_handler<Ms: Clone + DeserializeOwned>() -> Listener<Ms> {
    trigger_update_ev(|ev| {
        ev.detail()
            .into_serde()
            .expect("trigger_update_handler: Deserialization failed!")
    })
}
