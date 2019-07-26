//! This module contains code related to event handling; ie things that update the dom, related to
//! `web_sys::Event`

use crate::{
    dom_types::MessageMapper,
    util::{self, ClosureNew},
};

use enclose::enclose;
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

type EventHandler<Ms> = Box<dyn FnMut(web_sys::Event) -> Ms>;

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
    pub closure: Option<Closure<dyn FnMut(web_sys::Event)>>,
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
        let mut handler = self.handler.take().expect("Can't find old handler");
        // This is the closure ran when a DOM element has an user defined callback
        let closure = Closure::new(move |event: web_sys::Event| {
            let msg = handler(event);
            mailbox.send(msg);
        });

        (el_ws.as_ref() as &web_sys::EventTarget)
            .add_event_listener_with_callback(
                self.trigger.as_str(),
                closure.as_ref().unchecked_ref(),
            )
            .expect("Problem adding listener to element");

        // Store the closure so we can detach it later. Not detaching it when an element
        // is removed will trigger a panic.
        if self.closure.replace(closure).is_some() {
            panic!("self.closure already set in attach");
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
            .expect("Problem removing listener from element");
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
    fn map_message(self, f: impl FnOnce(Ms) -> OtherMs + 'static + Clone) -> Listener<OtherMs> {
        Listener {
            trigger: self.trigger,
            handler: self.handler.map(enclose!((f) |mut eh| {
                Box::new(move |event| {
                    let m = (*eh)(event);
                    (f.clone())(m)
                }) as EventHandler<OtherMs>
            })),
            closure: self.closure,
            control_val: self.control_val,
            control_checked: self.control_checked,

            category: self.category,
            message: self.message.map(f),
        }
    }
}

/// Create an event that passes a String of field text, for fast input handling.
pub fn input_ev<Ms, T: ToString + Copy>(
    trigger: T,
    handler: impl FnOnce(String) -> Ms + 'static + Clone,
) -> Listener<Ms> {
    let closure = move |event: web_sys::Event| {
        let value = event
            .target()
            .as_ref()
            .ok_or("Can't get event target reference")
            .and_then(util::get_value)
            .map_err(crate::error)
            .unwrap_or_default();

        (handler.clone())(value)
    };

    Listener::new(
        &trigger.to_string(),
        Some(Box::new(closure)),
        Some(Category::Input),
        None,
    )
}

/// Create an event that passes a `web_sys::KeyboardEvent`, allowing easy access
/// to items like `key_code`() and key().
pub fn keyboard_ev<Ms: Clone, T: ToString + Copy>(
    trigger: T,
    handler: impl FnOnce(web_sys::KeyboardEvent) -> Ms + 'static + Clone,
) -> Listener<Ms> {
    let closure = move |event: web_sys::Event| {
        (handler.clone())(event.dyn_ref::<web_sys::KeyboardEvent>().unwrap().clone())
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
    handler: impl FnOnce(web_sys::MouseEvent) -> Ms + 'static + Clone,
) -> Listener<Ms> {
    let closure = move |event: web_sys::Event| {
        (handler.clone())(event.dyn_ref::<web_sys::MouseEvent>().unwrap().clone())
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
    handler: impl FnOnce(web_sys::PointerEvent) -> Ms + 'static + Clone,
) -> Listener<Ms> {
    let closure = move |event: web_sys::Event| {
        (handler.clone())(event.dyn_ref::<web_sys::PointerEvent>().unwrap().clone())
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
    handler: impl FnOnce(web_sys::Event) -> Ms + 'static + Clone,
) -> Listener<Ms> {
    let closure = move |event: web_sys::Event| (handler.clone())(event);
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
    handler: impl FnOnce(web_sys::CustomEvent) -> Ms + 'static + Clone,
) -> Listener<Ms> {
    let closure = move |event: web_sys::Event| {
        (handler.clone())(event.dyn_ref::<web_sys::CustomEvent>().unwrap().clone())
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
