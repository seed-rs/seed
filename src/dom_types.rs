//! This module contains structs and enums that represent dom types, and their parts.
//! These are the types used internally by our virtual dom.

use std::collections::HashMap;

use web_sys;


// todo cleanup enums vs &strs for restricting events/styles/attrs to
// todo valid ones.


/// UpdateEl is used to distinguish arguments in element-creation macros.
pub trait UpdateEl<T> {
    // T is the type of thing we're updating; eg attrs, style, events etc.
    fn update(self, el: &mut T);
}

impl<Ms> UpdateEl<El<Ms>> for Attrs {
    fn update(self, el: &mut El<Ms>) {
        el.attrs = self;
    }
}

impl<Ms> UpdateEl<El<Ms>> for Style {
    fn update(self, el: &mut El<Ms>) {
        el.style = self;
    }
}

impl<Ms> UpdateEl<El<Ms>> for Events<Ms> {
    fn update(self, el: &mut El<Ms>) {
        el.events = self;
    }
}
impl<Ms> UpdateEl<El<Ms>> for &str {
    fn update(self, el: &mut El<Ms>) {
        el.text = Some(self.into());
    }
}

impl<Ms> UpdateEl<El<Ms>> for Vec<El<Ms>> {
    fn update(self, el: &mut El<Ms>) {
        el.children = self;
    }
}


#[derive(Debug)]
pub enum Attr {
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

#[derive(Clone, Debug)]
pub struct Attrs {
    // todo enum of only allowed attrs?
    pub vals: HashMap<&'static str, &'static str>
}

impl Attrs {
    pub fn new(vals: HashMap<&'static str, &'static str>) -> Self {
        Self {vals}
    }

    pub fn empty() -> Self {
        Self {vals: HashMap::new()}
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

#[derive(Clone, Debug)]
pub struct Style {
    // Handle Style separately from Attrs, since it commonly involves multiple parts.
    // todo enum for key?
    pub vals: HashMap<&'static str, &'static str>
}

impl Style {
    // todo avoid Dry code between this and Attrs.
    pub fn new(vals: HashMap<&'static str, &'static str>) -> Self {
        Self {vals}
    }

    pub fn empty() -> Self {
        Self {vals: HashMap::new()}
    }

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
/// Comprehensive list: https://developer.mozilla.org/en-US/docs/Web/Events
macro_rules! make_events {
    // Create shortcut macros for any element; populate these functions in this module.
    { $($event_camel:ident => $event:expr),+ } => {

        #[derive(Clone, Debug)]
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
                        web_sys::console::log_1(&format!("Can't find this event: {}", event).into());
                        Event::Click
                    }
                }
            }
        }

    }
}

make_events! {
    AuxClick => "auxclick", Click => "click", ContextMenu => "contextmenu", DblClick => "dblclick",
    MouseDown => "mousedown", MouseEnter => "mouseenter", MouseLeave => "mouseleave",
    MouseMove => "mousemove", MouseOver => "mouseover", MouseOut => "mouseout", MouseUp => "mouseup",
    PointerLockChange => "pointerlockchange", PointerLockError => "pointerlockerror", Select => "select",
    Wheel => "wheel"
}


#[derive(Clone, Debug)]
pub struct Events<Ms> {
    // Msg is an enum of types of Msg.
    // This is not tied to the real DOM, unlike attrs and style; used internally
    // by the virtual dom.
    // HashMap might be more appropriate, but Event would need
    // to implement Eq and Hash.
    pub vals: Vec<(Event, Ms)>
}

impl<Ms> Events<Ms> {
    pub fn new(vals: Vec<(Event, Ms)>) -> Self {
        Self {vals}
    }

    pub fn empty() -> Self {
        Self {vals: Vec::new()}
    }
}


/// Populate tags using a macro, to reduce code repetition.
/// The tag enum primarily exists to ensure only valid elements are allowed.
/// Comprehensive list: https://developer.mozilla.org/en-US/docs/Web/HTML/Element
/// We leave out non-body tags like html, meta, title, and body.
macro_rules! make_tags {
    // Create shortcut macros for any element; populate these functions in this module.
    { $($tag_camel:ident => $tag:expr),+ } => {

        #[derive(Debug)]
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

make_tags! {
    Address => "address", Article => "article", Aside => "aside", Footer => "footer",
    Hgroup => "hgroup", Main => "main", Nav => "nav", Section => "section", BlockQuote => "blockquote",
    Dd => "dd", Dir => "dir", Dl => "dl", Dt => "dt", FigCaption => "figcaption", Figure => "figure",
    Hr => "hr", Li => "li", Ol => "ol", Pre => "pre", Ul => "ul", Abbr => "abbr",
    B => "b", Bdi => "bdi", Bdo => "bdo", Br => "br", Cite => "cite", Code => "code", Data => "data",
    Dfn => "dfn", Em => "em", I => "i", Kbd => "kbd", Mark => "mark", Q => "q", Rb => "rb",
    Rp => "rp", Rt => "rt", Rtc => "rtc", Ruby => "ruby", S => "s", Samp => "samp", Small => "small",
    Span => "span", Strong => "strong", Sub => "sub", Sup => "sup", Time => "time", Tt => "tt",
    U => "u", Var => "var", Wbr => "wbr",

    A => "a", Img => "img", Div => "div", H1 => "h1",
    H2 => "h2", H3 => "h3", H4 => "h4", H5 => "h5", H6 => "h6", P => "p",
    Button => "button", Input => "input", Select => "select"
}

#[derive(Debug)]
pub struct El<Ms> {
    // M is a message type, as in part of TEA.

    // Don't use 'Element' name verbatim, to avoid * import conflict with web_sys.
    // todo web_sys::Element is a powerful struct. Use that instead??
    // https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.Element.html
    // todo can we have both text and children?
    // pub id: u32,
    pub tag: Tag,
    pub attrs: Attrs,
    pub style: Style,
    pub events: Events<Ms>,
    pub text: Option<String>,
    pub children: Vec<El<Ms>>,
}

impl<Ms> El<Ms> {
    pub fn new(tag: Tag, attrs: Attrs, style: Style, events: Events<Ms>,
                text: &str, children: Vec<El<Ms>>) -> Self {
        Self {tag, attrs, style, events, text: Some(text.into()), children}
    }

    pub fn empty(tag: Tag) -> Self {
        Self {tag, attrs: Attrs::empty(), style: Style::empty(), events: Events::empty(),
              text: None, children: Vec::new()}
    }

    pub fn add_child(&mut self, element: El<Ms>) {
        self.children.push(element);
    }

    pub fn add_attr(&mut self, key: &'static str, val: &'static str) {
        self.attrs.vals.insert(key, val);
    }

    pub fn add_style(&mut self, key: &'static str, val: &'static str) {
        self.style.vals.insert(key, val);
    }

    pub fn add_event(&mut self, event: Event, message: Ms) {
        self.events.vals.push((event, message));
    }

    pub fn set_text(&mut self, text: &str) {
        self.text = Some(text.into())
    }

    // todo do we need this method?
    fn _html(&self) -> String {
        let text = self.text.clone().unwrap_or(String::new());

        let opening = String::from("<") + self.tag.as_str() + &self.attrs.as_str() + " style=\"" + &self.style.as_str() + & ">\n";

        let inner = self.children.iter().fold(String::new(), |result, child| result + &child._html());

        let closing = String::from("\n</") + self.tag.as_str() + ">";
        
        opening + &text + &inner + &closing
    }
}

//// todo not working due to issues between web_sys Node and Element.
//impl<Msg> From<El<Msg>> for web_sys::Element {
//    fn from(el_vdom: El<Msg>) -> web_sys::Element {
//        let el = document.create_element(el_vdom.tag.as_str()).unwrap();
//
//        for (name, val) in &active_el_vdom.attrs.vals {
//            el.set_attribute(name, val);
//        }
//
//        // Style is just an attribute in the actual Dom, but is handled specially in our vdom;
//        // merge the different parts of style here.
//        if &el_vdom.style.vals.keys().len() > &0 {
//            el.set_attribute("style", &el_vdom.style.as_str());
//        }
//
//        // We store text as Option<String>, but set_text_content uses Option<&str>.
//        // A naive match Some(t) => Some(&t) does not work.
//        // See https://stackoverflow.com/questions/31233938/converting-from-optionstring-to-optionstr
//        el.set_text_content(el_vdom.text.as_ref().map(String::as_ref));
//        el
//    }
//}


