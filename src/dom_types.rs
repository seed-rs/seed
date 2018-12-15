//! This module contains structs and enums that represent dom types, and their parts.
//! These are the types used internally by our virtual dom.

use std::collections::HashMap;

use web_sys;
use wasm_bindgen::{prelude::*, JsCast};

use crate::vdom::Mailbox;  // todo temp


// todo cleanup enums vs &strs for restricting events/styles/attrs to
// todo valid ones.


// todo once you sort out events/listeners etc, organize/tidy this module.
// todo and reacttack when you need = 'static.


// TODO REATTACK when you need box

//pub trait UpdateListener<T> {
//    // T is the type of thing we're updating; eg attrs, style, events etc.
//    fn update_l(self, el: &mut T);
//}

//
//impl<Ms: Clone + 'static, F> UpdateListener<Listener<Ms>> for Box<F> where F: FnMut(String) -> Ms + 'static {
//    fn update_l(self, listener: &mut Listener<Ms>) {
//        listener.add_handler_input(self);
//    }
//}

//impl<Ms: Clone + 'static> UpdateListener<Listener<Ms>> for Box<Ms> {
//    fn update_l(self, listener: &mut Listener<Ms>) {
//        listener.add_handler_simple(self);
//    }
//}

/// Create an event that passes no data, other than it occured. Foregoes using a closure,
/// in favor of pointing to a message directly.
pub fn simple_ev<Ms: Clone + 'static>(trigger: &str, message: Ms) -> Listener<Ms> {
    let mut listener = Listener::empty(trigger.into());
    listener.add_handler_simple(message);
    listener
}

/// Create an event that passes a String of field text, for fast input handling.
pub fn input_ev<Ms: Clone + 'static>(trigger: &str, handler: impl FnMut(String) -> Ms + 'static) -> Listener<Ms> {
    let mut listener = Listener::empty(trigger.into());
    // handler must be boxed before passing to Listener's add method.
    listener.add_handler_input(Box::new(handler));
    listener
}

/// Create an event that passes a web_sys::Event, allowing full control of
/// event-handling
pub fn raw_ev<Ms: Clone + 'static>(trigger: &str, handler: impl FnMut(web_sys::Event) -> Ms + 'static) -> Listener<Ms> {
    let mut listener = Listener::empty(trigger.into());
    // handler must be boxed before passing to Listener's add method.
    listener.add_handler_raw(Box::new(handler));
    listener
}

/// Create an event that passes a web_sys::KeyboardEvent, allowing easy access
/// to items like key_code() and key().
pub fn keyboard_ev<Ms: Clone + 'static>(trigger: &str, handler: impl FnMut(web_sys::KeyboardEvent) -> Ms + 'static) -> Listener<Ms> {
    let mut listener = Listener::empty(trigger.into());
    // handler must be boxed before passing to Listener's add method.
    listener.handler_keyboard(Box::new(handler));
    listener
}

/// Event-handling for Elements
pub struct Listener<Ms: Clone> {
    pub trigger: String,
    pub handler: Option<Box<FnMut(web_sys::Event) -> Ms>>,
    // We store closure here so we can detach it later.
    pub closure: Option<Closure<FnMut(web_sys::Event)>>,
}

// https://rustwasm.github.io/wasm-bindgen/api/wasm_bindgen/closure/struct.Closure.html
// todo + 'static ??
impl<Ms: Clone + 'static> Listener<Ms> {
    pub fn empty(event: Event) -> Self {
        Self {
            trigger: String::from(event.as_str()),
            handler: None,
            closure: None,
        }
    }

    /// Add a handler that doesn't process any details about the event, other
    /// than it happened. The message Enum should not take a value.
    fn add_handler_simple(&mut self, message: Ms) {
        let handler = || message;
        let closure = move |_| handler.clone()();
        self.handler = Some(Box::new(closure));
    }

     /// Add a handler that takes the event target's value as text; use this
     /// wiht input, textarea, and select elements.
     fn add_handler_input(&mut self, mut handler: Box<FnMut(String) -> Ms + 'static>){
         // We need to extract event.target.value, but value doesn't exist for generic
         // event targets. We must cast as the appropriate type.
         // todo: See if there's a way around this awkward behavior.
        let closure = move |event: web_sys::Event| {
            if let Some(target) = event.target() {
                if let Some(input) = target.dyn_ref::<web_sys::HtmlInputElement>() {
                    return handler(input.value());
                }
                if let Some(input) = target.dyn_ref::<web_sys::HtmlTextAreaElement>() {
                    return handler(input.value());
                }
                if let Some(input) = target.dyn_ref::<web_sys::HtmlSelectElement>() {
                    return handler(input.value());
                }
            }
            handler(String::new())
        };
         self.handler = Some(Box::new(closure));
     }

    fn add_handler_raw(&mut self, mut handler: Box<FnMut(web_sys::Event) -> Ms + 'static>){
        let closure = move |event: web_sys::Event| handler(event);
        self.handler = Some(Box::new(closure));
    }

    fn handler_keyboard(&mut self, mut handler: Box<FnMut(web_sys::KeyboardEvent) -> Ms + 'static>){
        let closure = move |event: web_sys::Event| {
            handler(event.dyn_ref::<web_sys::KeyboardEvent>().unwrap().clone())
        };
        self.handler = Some(Box::new(closure));
    }

    /// This method is where the processing logic for events happens.
    pub fn attach(&mut self, element: &web_sys::Element, mailbox: Mailbox<Ms>) {
        // This and detach taken from Draco.
        let mut handler = self.handler.take().expect("Can't find old handler");

        let closure = Closure::wrap(
            Box::new(move |event: web_sys::Event| {
                mailbox.send(handler(event))
            })
                as Box<FnMut(web_sys::Event) + 'static>,
        );
        (element.as_ref() as &web_sys::EventTarget)
            .add_event_listener_with_callback(&self.trigger, closure.as_ref().unchecked_ref())
            .expect("add_event_listener_with_callback");

        // Store the closure so we can detach it later. Not detaching it (when an element
        // is removed?) will cause a panic.
        self.closure = Some(closure);

//        self.handler.replace(handler);  // todo ?
    }

    pub fn detach(&self, el_ws: &web_sys::Element) {
        // This and attach taken from Draco.
        let closure = self.closure.as_ref().unwrap();
        (el_ws.as_ref() as &web_sys::EventTarget)
            .remove_event_listener_with_callback(&self.trigger, closure.as_ref().unchecked_ref())
            .expect("remove_event_listener_with_callback");
    }
}

impl<Ms: Clone + 'static>  PartialEq for Listener<Ms> {
    fn eq(&self, other: &Self) -> bool {
        // todo we're only checking the trigger - will miss changes if
        // todo only the fn passed changes!
        self.trigger == other.trigger
    }
}

/// UpdateEl is used to distinguish arguments in element-creation macros, and handle
/// each type appropriately.
pub trait UpdateEl<T> {
    // T is the type of thing we're updating; eg attrs, style, events etc.
    fn update(self, el: &mut T);
}

impl<Ms: Clone> UpdateEl<El<Ms>> for Attrs {
    fn update(self, el: &mut El<Ms>) {
        // todo decide if you want to allow multiples and compose them.
        el.attrs = self;
    }
}

impl<Ms: Clone> UpdateEl<El<Ms>> for &Attrs {
    fn update(self, el: &mut El<Ms>) {
        el.attrs = self.clone();
    }
}

impl<Ms: Clone> UpdateEl<El<Ms>> for Style {
    fn update(self, el: &mut El<Ms>) {
        el.style = self;
    }
}

impl<Ms: Clone> UpdateEl<El<Ms>> for &Style {
    fn update(self, el: &mut El<Ms>) {
        el.style = self.clone();
    }
}

impl<Ms: Clone> UpdateEl<El<Ms>> for Listener<Ms> {
    fn update(self, el: &mut El<Ms>) {
        el.listeners.push(self)
    }
}

impl<Ms: Clone> UpdateEl<El<Ms>> for Vec<Listener<Ms>> {
    fn update(self, el: &mut El<Ms>) {
        for listener in self.into_iter() {
            el.listeners.push(listener)
        }
    }
}

impl<Ms: Clone> UpdateEl<El<Ms>> for &str {
    // This, or some other mechanism seems to work for String too... note sure why.
    fn update(self, el: &mut El<Ms>) {
        el.text = Some(self.into());
    }
}

impl<Ms: Clone> UpdateEl<El<Ms>> for Vec<El<Ms>> {
    fn update(self, el: &mut El<Ms>) {
        for child in self.into_iter() {
            el.children.push(child);
        }
    }
}

impl<Ms: Clone> UpdateEl<El<Ms>> for El<Ms> {
    fn update(self, el: &mut El<Ms>) {
        el.children.push(self)
    }
}


//#[derive(Debug)]
pub enum _Attr {
    // https://www.w3schools.com/tags/ref_attributes.asp
    // This enum primarily exists to ensure only valid attrs are allowed.
    Action,
    Alt,
    Class,
    Disabled,
    Height,
    Href,
    Id,
    Lang,
    OnChange,
    OnClick,
    OnContextMenu,
    OnDblClick,
    OnMouseOver,
    Src,
    Title,
    Width,
}

/// A thinly-wrapped HashMap holding DOM attributes
#[derive(Clone, PartialEq)]
pub struct Attrs {
    // todo: Custom implm where we ignore checked.
    // todo enum of only allowed attrs?
    pub vals: HashMap<String, String>
}

//impl PartialEq for Attrs {
//    fn eq(&self, other: &Self) -> bool {
//        for (key, val) in &self.vals {
//            if key == "Checked".into() {
//                continue
//            }
//            match other.vals.get(key).unwrap() {
//                Some(other_val) => {
//                    if val != other_val { return false }
//                },
//                None => return false,
//            }
//        }
//        // todo iter through remaining other keys.
//        true
//    }
//}


impl Attrs {
    pub fn new(vals: HashMap<String, String>) -> Self {
        Self { vals }
    }

    pub fn empty() -> Self {
        Self { vals: HashMap::new() }
    }

    pub fn as_str(&self) -> String {
        let mut result = String::new();
        for (key, val) in &self.vals {
            result += &format!(" {k}=\"{v}\"", k=key, v=val);
        }
        result
    }

    pub fn add(&mut self, key: &str, val: &str) {
        self.vals.insert(key.to_string(), val.to_string());
    }
}


/// Handle Style separately from Attrs, since it commonly involves multiple parts,
/// and has a different semantic meaning.
#[derive(Clone, PartialEq)]
pub struct Style {
    // todo enum for key?
    pub vals: HashMap<String, String>
}

impl Style {
    // todo avoid Dry code between this and Attrs.
    pub fn new(vals: HashMap<String, String>) -> Self {
        let mut new_vals = HashMap::new();
        for (key, val) in vals.into_iter() {
            let val_backup = val.clone();
            match val.parse::<u32>() {
                Ok(_) => new_vals.insert(key, val_backup + "px"),
                Err(_) => new_vals.insert(key, val_backup),
            };
        }

        Self { vals: new_vals }
    }

    pub fn empty() -> Self {
        Self { vals: HashMap::new() }
    }

    /// Output style as a string, as would be set in the DOM as the attribute value
    /// for 'style'. Eg: "display: flex; font-size: 1.5em"
    pub fn as_str(&self) -> String {
        let mut result = String::new();
        if self.vals.keys().len() > 0 {
            for (key, val) in &self.vals {
                result += &format!("{k}: {v}; ", k = key, v = val);
            }
        }

        result
    }

    pub fn add(&mut self, key: &str, val: &str) {
        self.vals.insert(key.to_string(), val.to_string());
    }
}

/// Similar to tag population.
macro_rules! make_events {
    // Create shortcut macros for any element; populate these functions in this module.
    { $($event_camel:ident => $event:expr),+ } => {

        /// The Event enum restricts element-creation to only valid event names, as defined here:
        /// https://developer.mozilla.org/en-US/docs/Web/Events
        #[derive(Clone)]
        pub enum Event {
            $(
                $event_camel,
            )+
        }

        impl Event {
            pub fn as_str(&self) -> &str {
                match self {
                    $ (
                        Event::$event_camel => $event,
                    ) +
                }
            }
        }

        impl From<&str> for Event {
            fn from(event: &str) -> Self {
                match event {
                    $ (
                          $event => Event::$event_camel,
                    ) +
                    _ => {
                        crate::log(&format!("Can't find this event: {}", event));
                        Event::Click
                    }
                }
            }
        }

    }
}

/// Comprehensive list: https://developer.mozilla.org/en-US/docs/Web/Events
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

    KeyDown => "keydown",
    KeyPress => "keypress", AuxClick => "auxclick", Click => "click", ContextMenu => "contextmenu", DblClick => "dblclick",
    MouseDown => "mousedown", MouseEnter => "mouseenter", MouseLeave => "mouseleave",
    MouseMove => "mousemove", MouseOver => "mouseover", MouseOut => "mouseout", MouseUp => "mouseup",
    PointerLockChange => "pointerlockchange", PointerLockError => "pointerlockerror", Select => "select",
    Wheel => "wheel",

    Drag => "drag", DragEnd => "dragend", DragEnter => "dragenter", DragStart => "dragstart", DragLeave => "dragleave",
    DragOver => "dragover", Drop => "drop",

    AudioProcess => "audioprocess", CanPlay => "canplay", CanPlayThrough => "canplaythrough", Complete => "complete",
    DurationChange => "durationchange", Emptied => "emptied", Ended => "ended", LoadedData => "loadeddata",
    LoadedMetaData => "loadedmetadata", Pause => "pause", Play => "play", Playing => "playing", RateChagne => "ratechange",
    Seeked => "seeked", Seeking => "seeking", Stalled => "stalled", Suspend => "suspend", TimeUpdate => "timeupdate",
    VolumeChange => "volumechange",

    // todo finish this

    Change => "change",

    Input => "input"
}


/// Populate tags using a macro, to reduce code repetition.
/// The tag enum primarily exists to ensure only valid elements are allowed.
/// We leave out non-body tags like html, meta, title, and body.
macro_rules! make_tags {
    // Create shortcut macros for any element; populate these functions in this module.
    { $($tag_camel:ident => $tag:expr),+ } => {

        /// The Tag enum restricts element-creation to only valid tags, as defined here:
        /// https://developer.mozilla.org/en-US/docs/Web/HTML/Element
        #[derive(Clone, PartialEq)]
        pub enum Tag {
            $(
                $tag_camel,
            )+
        }

        impl Tag {
            pub fn as_str(&self) -> &str {
                match self {
                    $ (
                        Tag::$tag_camel => $tag,
                    ) +
                }
            }
        }
    }
}

/// Comprehensive list: https://developer.mozilla.org/en-US/docs/Web/HTML/Element
/// Grouped here by category on Mozilla's page, linked above.
make_tags! {
    Address => "address", Article => "article", Aside => "aside", Footer => "footer",
    Header => "header", H1 => "h1",
    H2 => "h2", H3 => "h3", H4 => "h4", H5 => "h5", H6 => "h6",
    Hgroup => "hgroup", Main => "main", Nav => "nav", Section => "section",

    BlockQuote => "blockquote",
    Dd => "dd", Dir => "dir", Div => "div", Dl => "dl", Dt => "dt", FigCaption => "figcaption", Figure => "figure",
    Hr => "hr", Li => "li", Ol => "ol", P => "p", Pre => "pre", Ul => "ul",

    A => "a", Abbr => "abbr",
    B => "b", Bdi => "bdi", Bdo => "bdo", Br => "br", Cite => "cite", Code => "code", Data => "data",
    Dfn => "dfn", Em => "em", I => "i", Kbd => "kbd", Mark => "mark", Q => "q", Rb => "rb",
    Rp => "rp", Rt => "rt", Rtc => "rtc", Ruby => "ruby", S => "s", Samp => "samp", Small => "small",
    Span => "span", Strong => "strong", Sub => "sub", Sup => "sup", Time => "time", Tt => "tt",
    U => "u", Var => "var", Wbr => "wbr",

    Area => "area", Audio => "audio", Img => "img", Map => "map", Track => "track", Video => "video",

    Applet => "applet", Embed => "embed", Iframe => "iframe",
    NoEmbed => "noembed", Object => "object", Param => "param", Picture => "picture", Source => "source",

    Canvas => "canvas", NoScript => "noscript", Script => "Script",

    Del => "del", Ins => "ins",

    Caption => "caption", Col => "col", ColGroup => "colgroup", Table => "table", Tbody => "tbody",
    Td => "td", Tfoot =>"tfoot", Th => "th", Thead => "thead", Tr => "tr",

    Button => "button", DataList => "datalist", FieldSet => "fieldset", Form => "form", Input => "input",
    Label => "label", Legend => "legend", Meter => "meter", OptGroup => "optgroup", Option => "option",
    Output => "output", Progress => "progress", Select => "select", TextArea => "textarea",

    Details => "details", Dialog => "dialog", Menu => "menu", MenuItem => "menuitem", Summary => "summary",

    Content => "content", Element => "element", Shadow => "shadow", Slot => "slot", Template => "template"
}

/// An component in our virtual DOM.
pub struct El<Ms: Clone + 'static> {
    // M sis a message type, as in part of TEA.
    // We call this 'El' instead of 'Element' for brevity, and to prevent
    // confusion with web_sys::Element.

    // Core attributes that correspond to the DOM element.
    pub tag: Tag,
    pub attrs: Attrs,
    pub style: Style,
    pub text: Option<String>,
    pub children: Vec<El<Ms>>,

    // Things that get filled in later, to assist with rendering.
    pub id: Option<u32>,
    pub nest_level: Option<u32>,
    pub el_ws: Option<web_sys::Element>,

    // todo temp?
    pub key: Option<u32>,

    // Event-handling
    pub listeners: Vec<Listener<Ms>>,
}

impl<Ms: Clone + 'static> El<Ms> {
    pub fn new(tag: Tag, attrs: Attrs, style: Style,
               listeners: Vec<Listener<Ms>>, text: &str, children: Vec<El<Ms>>) -> Self {
        Self {tag, attrs, style, text: Some(text.into()), children,
            el_ws: None, listeners, key: None, id: None, nest_level: None}
    }

    pub fn empty(tag: Tag) -> Self {
        Self {tag, attrs: Attrs::empty(), style: Style::empty(),
            text: None, children: Vec::new(), el_ws: None,
            listeners: Vec::new(), key: None, id: None, nest_level: None}
    }

    pub fn add_child(&mut self, element: El<Ms>) {
        self.children.push(element);
    }

    pub fn add_attr(&mut self, key: String, val: String) {
        self.attrs.vals.insert(key, val);
    }

    pub fn add_style(&mut self, key: String, val: String) {
        self.style.vals.insert(key, val);
    }

    pub fn set_text(&mut self, text: &str) {
        self.text = Some(text.into())
    }

    // todo do we need this method?
    /// Output the HTML of this node, including all its children, recursively.
    fn _html(&self) -> String {
        let text = self.text.clone().unwrap_or_default();

        let opening = String::from("<") + self.tag.as_str() + &self.attrs.as_str() +
            " style=\"" + &self.style.as_str() + ">\n";

        let inner = self.children.iter().fold(String::new(), |result, child| result + &child._html());

        let closing = String::from("\n</") + self.tag.as_str() + ">";

        opening + &text + &inner + &closing
    }

    /// This is used to provide access to el_ws while recursively appending children to it.
    pub fn quick_clone(&self) -> Self {
        Self {
            tag: self.tag.clone(),
            attrs: Attrs::empty(),
            style: Style::empty(),
            text: None,
            children: Vec::new(),
            key: None,
            id: None,
            nest_level: None,
            el_ws: self.el_ws.clone(),
            listeners: Vec::new(),
        }
    }

    /// Dummy elements are used when logic must return an El due to the type
    /// system, but we don't want to render anything.
    pub fn is_dummy(&self) -> bool {
    if let Tag::Del = self.tag {
        if self.attrs.vals.get("dummy-element").is_some() {
            return true;
        }
    }
    false
    }
}

/// Allow the user to clone their Els. Note that there's no easy way to clone the
/// closures within listeners, so we ommit it.
impl<Ms: Clone + 'static> Clone for El<Ms> {
    fn clone(&self) -> Self {
        Self {
            tag: self.tag.clone(),
            attrs: self.attrs.clone(),
            style: self.style.clone(),
            text: self.text.clone(),
            children: self.children.clone(),
            key: self.key,
            id: self.id,
            nest_level: self.nest_level,
            el_ws: self.el_ws.clone(),
            listeners: Vec::new(),
        }
    }
}

impl<Ms: Clone + 'static>  PartialEq for El<Ms> {
    fn eq(&self, other: &Self) -> bool {
        // todo Again, note that the listeners check only checks triggers.
        // Don't check children.
        self.tag == other.tag &&
        self.attrs == other.attrs &&
        self.style == other.style &&
        self.text == other.text &&
        self.listeners == other.listeners &&
        // TOdo not sure how nest-level should be used. Is it a given based on
        // todo how we iterate? Sanity-check for now.
        self.nest_level == other.nest_level
    }
}
