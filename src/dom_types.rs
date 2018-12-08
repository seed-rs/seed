//! This module contains structs and enums that represent dom types, and their parts.
//! These are the types used internally by our virtual dom.

use std::collections::HashMap;

use std::borrow::Cow;
use std::cmp;
use std::rc::Rc;

use web_sys;
use wasm_bindgen::{prelude::*, JsCast};

use crate::vdom::Mailbox;  // todo temp


// todo cleanup enums vs &strs for restricting events/styles/attrs to
// todo valid ones.



pub struct Listener<Ms: Clone> {
    pub trigger: Cow<'static, str>,
    pub handler: Option<Box<FnMut(web_sys::Event) -> Ms>>,
    pub closure: Option<Closure<FnMut(web_sys::Event)>>,
}


// https://rustwasm.github.io/wasm-bindgen/api/wasm_bindgen/closure/struct.Closure.html
impl<Ms: Clone + 'static> Listener<Ms> {
    //    pub fn new(vals: Vec<(Event, Box<EventFn<Ms>>)>) -> Self {
    pub fn new(event: Event, handler: impl FnMut(web_sys::Event) -> Ms + 'static) -> Self {
        Self {
            trigger: String::from(event.as_str()).into(),
            handler: Some(Box::new(handler)),
            closure: None
        }
    }

    pub fn new_input(event: Event, mut handler: impl FnMut(String) -> Ms + 'static) -> Self {
        let func = move |event: web_sys::Event| {
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
            handler("".into())
        };

        Self::new(event, func)
    }

    /// This method is where the processing logic for events happens.
    fn attach(&mut self, element: &web_sys::Element, mailbox: Mailbox<Ms>) {
        let mut handler = self.handler.take().unwrap();

        let closure = Closure::wrap(
            Box::new(move |event: web_sys::Event| {
                mailbox.send(handler(event))
            })
                as Box<FnMut(web_sys::Event) + 'static>,
        );
        (element.as_ref() as &web_sys::EventTarget)
            .add_event_listener_with_callback(&self.trigger, closure.as_ref().unchecked_ref())
            .expect("add_event_listener_with_callback");

        closure.forget();
    }

    fn detach(&self, element: &web_sys::Element) {
        let closure = self.closure.as_ref().unwrap();
        (element.as_ref() as &web_sys::EventTarget)
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

/// UpdateEl is used to distinguish arguments in element-creation macros.
pub trait UpdateEl<T> {
    // T is the type of thing we're updating; eg attrs, style, events etc.
    fn update(self, el: &mut T);
}

impl<Ms: Clone> UpdateEl<El<Ms>> for Attrs {
    fn update(self, el: &mut El<Ms>) {
        el.attrs = self;
    }
}

impl<Ms: Clone> UpdateEl<El<Ms>> for Style {
    fn update(self, el: &mut El<Ms>) {
        el.style = self;
    }
}

impl<Ms: Clone> UpdateEl<El<Ms>> for Vec<Listener<Ms>> {
    fn update(self, el: &mut El<Ms>) {
        el.listeners = self;
    }
}

impl<Ms: Clone> UpdateEl<El<Ms>> for &str {
    fn update(self, el: &mut El<Ms>) {
        el.text = Some(self.into());
    }
}

impl<Ms: Clone> UpdateEl<El<Ms>> for Vec<El<Ms>> {
    fn update(self, el: &mut El<Ms>) {
        el.children = self;
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

//#[derive(Clone, Debug)]
#[derive(Clone, PartialEq)]
pub struct Attrs {
    // todo enum of only allowed attrs?
    pub vals: HashMap<String, String>
}

impl Attrs {
    pub fn new(vals: HashMap<String, String>) -> Self {
        Self { vals }
    }

    pub fn empty() -> Self {
        Self { vals: HashMap::new() }
    }

    // todo from/into instead of as_str?
    pub fn as_str(&self) -> String {
        let mut result = String::new();
        for (key, val) in &self.vals {
            result += &format!(" {k}=\"{v}\"", k=key, v=val);
        }
        result
    }
}


// Handle Style separately from Attrs, since it commonly involves multiple parts.
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
            result += "\"";
        }

        result
    }
}

/// Similar to tag population.
macro_rules! make_events {
    // Create shortcut macros for any element; populate these functions in this module.
    { $($event_camel:ident => $event:expr),+ } => {

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
                        &Event::$event_camel => $event,
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

    Change => "change",

    Input => "input"
}


/// Populate tags using a macro, to reduce code repetition.
/// The tag enum primarily exists to ensure only valid elements are allowed.
/// We leave out non-body tags like html, meta, title, and body.
macro_rules! make_tags {
    // Create shortcut macros for any element; populate these functions in this module.
    { $($tag_camel:ident => $tag:expr),+ } => {

//        #[derive(PartialEq)]
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
                        &Tag::$tag_camel => $tag,
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

//#[derive(Clone)] // todo temp??
pub struct El<Ms: Clone + 'static> {
    // M is a message type, as in part of TEA.

    // Don't use 'Element' name verbatim, to avoid * import conflict with web_sys.
    // todo web_sys::Element is a powerful struct. Use that instead??
    // https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.Element.html
    // todo can we have both text and children?
    // pub id: u32,
    pub tag: Tag,
    pub attrs: Attrs,
    pub style: Style,
    //    pub events: Events<Ms>,
    pub text: Option<String>,
    pub children: Vec<El<Ms>>,

    // todo temp?
    pub key: Option<u32>,
    pub id: Option<u32>,
    pub nest_level: Option<u32>,


    // todo temp?
    pub el_ws: Option<web_sys::Element>,
    listeners: Vec<Listener<Ms>>,
}


impl<Ms: Clone + 'static> El<Ms> {
    pub fn new(tag: Tag, attrs: Attrs, style: Style,
               text: &str, children: Vec<El<Ms>>) -> Self {
        Self {tag, attrs, style, text: Some(text.into()), children,
            el_ws: None, listeners: Vec::new(), key: None, id: None, nest_level: None}
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

    pub fn add_listener(&mut self, event: Event, handler: impl FnMut(web_sys::Event) -> Ms + 'static) {
        self.listeners.push(Listener::new(event, handler));
    }

    pub fn set_text(&mut self, text: &str) {
        self.text = Some(text.into())
    }

    // todo do we need this method?
    /// Output the HTML of this node, including all its children, recursively.
    fn _html(&self) -> String {
        let text = self.text.clone().unwrap_or(String::new());

        let opening = String::from("<") + self.tag.as_str() + &self.attrs.as_str() + " style=\"" + &self.style.as_str() + & ">\n";

        let inner = self.children.iter().fold(String::new(), |result, child| result + &child._html());

        let closing = String::from("\n</") + self.tag.as_str() + ">";

        opening + &text + &inner + &closing
    }

    // todo could do this with a From implementaiton once web_sys node/elemetn stop conflicting?
    /// Create, and return a web_sys Element, from our virtual-dom El. The web_sys
    /// Element is a close analog to the DOM elements.
    /// web-sys reference: https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.Element.html
    /// Mozilla reference: https://developer.mozilla.org/en-US/docs/Web/HTML/Element\
    /// See also: https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.Node.html
    pub fn make_websys_el(&mut self, document: &web_sys::Document, ids: &Vec<u32>, mailbox: Mailbox<Ms>) -> web_sys::Element {
//    pub fn make_websys_el(&mut self, el_map: HashMap<u32, web_sys::Element>, document: &web_sys::Document,
//                          ids: &Vec<u32>, mailbox: Mailbox<Ms>) -> HashMap<u32, web_sys::Element> {
        //
        // todo do we want to repeat finding window/doc for each el like this??
//        let window = web_sys::window().expect("no global `window` exists");
//        let document = window.document().expect("should have a document on window");

        let el_ws = document.create_element(&self.tag.as_str()).unwrap();
        for (name, val) in &self.attrs.vals {
            el_ws.set_attribute(name, val).unwrap();
        }

        // Style is just an attribute in the actual Dom, but is handled specially in our vdom;
        // merge the different parts of style here.
        if self.style.vals.keys().len() > 0 {
            el_ws.set_attribute("style", &self.style.as_str()).unwrap();
        }

        // We store text as Option<String>, but set_text_content uses Option<&str>.
        // A naive match Some(t) => Some(&t) does not work.
        // See https://stackoverflow.com/questions/31233938/converting-from-optionstring-to-optionstr
        el_ws.set_text_content(self.text.as_ref().map(String::as_ref));


        for listener in &mut self.listeners {
            listener.attach(&el_ws, mailbox.clone());
        }

//        for child in &mut self.children {
//            el_ws.append_child(&child.make_websys_el(document, ids, mailbox.clone())).unwrap();
//        }

//        self.el_ws = Some(el_ws.clone());  // todo clone??

        el_ws
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
            key: self.key.clone(),
            id: self.id.clone(),
            nest_level: self.nest_level.clone(),
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

fn add_id(ids: Vec<u32>) -> Vec<u32> {
    // duped from vdom
    let new_id = ids.last().unwrap() + 1;
    let mut result = ids;
    result.push(new_id);
    result
}
