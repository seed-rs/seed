//! This module contains structs and enums that represent dom types, and their parts.
//! These are the types used internally by our virtual dom.

use crate::{
    events::{self, Listener},
    util, websys_bridge,
};
use core::convert::AsRef;
use indexmap::IndexMap;
use pulldown_cmark;
use std::{borrow::Cow, fmt};
use web_sys;

pub use values::*;

pub mod values;

pub trait MessageMapper<Ms, OtherMs> {
    type SelfWithOtherMs;
    fn map_message(self, f: impl FnOnce(Ms) -> OtherMs + 'static + Clone) -> Self::SelfWithOtherMs;
}

/// Common Namespaces
#[derive(Debug, Clone, PartialEq)]
pub enum Namespace {
    Html,
    Svg,
    MathMl,
    Xul,
    Xbl,
    Custom(String),
}

// https://developer.mozilla.org/en-US/docs/Web/API/Document/createElementNS
impl Namespace {
    pub fn as_str(&self) -> &str {
        use Namespace::*;
        match self {
            Html => "http://www.w3.org/1999/xhtml",
            Svg => "http://www.w3.org/2000/svg",
            MathMl => "http://www.w3.org/1998/mathml",
            Xul => "http://www.mozilla.org/keymaster/gatekeeper/there.is.only.xul",
            Xbl => "http://www.mozilla.org/xbl",
            Custom(s) => s,
        }
    }
}

impl From<String> for Namespace {
    fn from(ns: String) -> Self {
        match ns.as_ref() {
            "http://www.w3.org/1999/xhtml" => Namespace::Html,
            "http://www.w3.org/2000/svg" => Namespace::Svg,
            "http://www.w3.org/1998/mathml" => Namespace::MathMl,
            "http://www.mozilla.org/keymaster/gatekeeper/there.is.only.xul" => Namespace::Xul,
            "http://www.mozilla.org/xbl" => Namespace::Xbl,
            _ => Namespace::Custom(ns),
        }
    }
}

/// `UpdateEl` is used to distinguish arguments in element-creation macros, and handle
/// each type appropriately.
pub trait UpdateEl<T> {
    // T is the type of thing we're updating; eg attrs, style, events etc.
    fn update(self, el: &mut T);
}

impl<Ms> UpdateEl<El<Ms>> for Attrs {
    fn update(self, el: &mut El<Ms>) {
        el.attrs.merge(self);
    }
}

impl<Ms> UpdateEl<El<Ms>> for &Attrs {
    fn update(self, el: &mut El<Ms>) {
        el.attrs.merge(self.clone());
    }
}

impl<Ms> UpdateEl<El<Ms>> for Style {
    fn update(self, el: &mut El<Ms>) {
        el.style.merge(self);
    }
}

impl<Ms> UpdateEl<El<Ms>> for &Style {
    fn update(self, el: &mut El<Ms>) {
        el.style.merge(self.clone());
    }
}

impl<Ms> UpdateEl<El<Ms>> for Listener<Ms> {
    fn update(self, el: &mut El<Ms>) {
        el.listeners.push(self)
    }
}

impl<Ms> UpdateEl<El<Ms>> for Vec<Listener<Ms>> {
    fn update(mut self, el: &mut El<Ms>) {
        el.listeners.append(&mut self);
    }
}

impl<Ms> UpdateEl<El<Ms>> for DidMount<Ms> {
    fn update(self, el: &mut El<Ms>) {
        el.hooks.did_mount = Some(self)
    }
}

impl<Ms> UpdateEl<El<Ms>> for DidUpdate<Ms> {
    fn update(self, el: &mut El<Ms>) {
        el.hooks.did_update = Some(self)
    }
}

impl<Ms> UpdateEl<El<Ms>> for WillUnmount<Ms> {
    fn update(self, el: &mut El<Ms>) {
        el.hooks.will_unmount = Some(self)
    }
}

impl<Ms> UpdateEl<El<Ms>> for &str {
    // This, or some other mechanism seems to work for String too... note sure why.
    fn update(self, el: &mut El<Ms>) {
        el.children.push(Node::Text(Text::new(self.to_string())))
    }
}

impl<Ms> UpdateEl<El<Ms>> for El<Ms> {
    fn update(self, el: &mut El<Ms>) {
        el.children.push(Node::Element(self))
    }
}

impl<Ms> UpdateEl<El<Ms>> for Vec<El<Ms>> {
    fn update(self, el: &mut El<Ms>) {
        el.children
            .append(&mut self.into_iter().map(Node::Element).collect());
    }
}

impl<Ms> UpdateEl<El<Ms>> for Node<Ms> {
    fn update(self, el: &mut El<Ms>) {
        el.children.push(self)
    }
}

impl<Ms> UpdateEl<El<Ms>> for Vec<Node<Ms>> {
    fn update(mut self, el: &mut El<Ms>) {
        el.children.append(&mut self);
    }
}

/// This is intended only to be used for the custom! element macro.
impl<Ms> UpdateEl<El<Ms>> for Tag {
    fn update(self, el: &mut El<Ms>) {
        el.tag = self;
    }
}

impl<Ms, I, U, F> UpdateEl<El<Ms>> for std::iter::Map<I, F>
where
    I: Iterator,
    U: UpdateEl<El<Ms>>,
    F: FnMut(I::Item) -> U,
{
    fn update(self, el: &mut El<Ms>) {
        self.for_each(|item| item.update(el));
    }
}

/// Similar to tag population.
macro_rules! make_attrs {
    // Create shortcut macros for any element; populate these functions in this module.
    { $($attr_camel:ident => $attr:expr),+ } => {

        /// The At enum restricts element-creation to only valid event names, as defined here:
        /// [https://developer.mozilla.org/en-US/docs/Web/HTML/Attributes](https://developer.mozilla.org/en-US/docs/Web/HTML/Attributes)
        #[derive(Clone, Debug, PartialEq, Eq, Hash)]
        pub enum At {
            $(
                $attr_camel,
            )+
            Custom(String)
        }

        impl At {
            pub fn as_str(&self) -> &str {
                match self {
                    $ (
                        At::$attr_camel => $attr,
                    ) +
                    At::Custom(val) => &val
                }
            }
        }

        impl From<&str> for At {
            fn from(attr: &str) -> Self {
                match attr {
                    $ (
                          $attr => At::$attr_camel,
                    ) +
                    _ => {
                        At::Custom(attr.to_owned())
                    }
                }
            }
        }
        impl From<String> for At {
            fn from(attr: String) -> Self {
                match attr.as_ref() {
                    $ (
                          $attr => At::$attr_camel,
                    ) +
                    _ => {
                        At::Custom(attr)
                    }
                }
            }
        }

    }
}

// Comprehensive list: https://www.w3schools.com/tags/ref_attributes.asp
make_attrs! {
    // Missing data-*
    Accept => "accept", AcceptCharset => "accept-charset", AccessKey => "accesskey", Action => "action",
    Alt => "alt", Async => "async", AutoComplete => "autocomplete", AutoFocus => "autofocus",
    AutoPlay => "autoplay", Charset => "charset", Checked => "checked", Cite => "cite", Class => "class",
    Color => "color", Cols => "cols", ColSpan => "colspan", Content => "content", ContentEditable => "contenteditable",
    Controls => "controls", Coords => "coords", Data => "data", DateTime => "datetime", Default => "default",
    Defer => "defer", Dir => "dir", DirName => "dirname", Disabled => "disabled", Download => "download",
    Draggable => "draggable", DropZone => "dropzone", EncType => "enctype", For => "for", Form => "form",
    FormAction => "formaction", Headers => "headers", Height => "height", Hidden => "hidden", High => "high",
    Href => "href", HrefLang => "hreflang", HttpEquiv => "http-equiv", Id => "id", IsMap => "ismap",
    Kind => "kind", Label => "label", Lang => "lang", List => "list", Loop => "loop", Low => "low",
    Max => "max", MaxLength => "maxlength", Media => "media", Method => "method", Min => "min", Multiple => "multiple",
    Muted => "muted", Name => "name", NoValidate => "novalidate", OnAbort => "onabort", OnAfterPrint => "onafterprint",
    OnBeforePrint => "onbeforeprint", OnBeforeUnload => "onbeforeunload", OnBlur => "onblur", OnCanPlay => "oncanplay",
    OnCanPlayThrough => "oncanplaythrough", OnChange => "onchange", OnClick => "onclick", OnContextMenu => "oncontextmenu",
    OnCopy => "oncopy", OnCueChange => "oncuechange", OnCut => "oncut", OnDblClick => "ondblclick",
    OnDrag => "ondrag", OnDragEnd => "ondragend", OnDragEnter => "ondragenter", OnDragLeave => "ondragleave",
    OnDragOver => "ondragover", OnDragStart => "ondragstart", OnDrop => "ondrop", OnDurationChange => "ondurationchange",
    OnEmptied => "onemptied", OnEnded => "onended", OnError => "onerror", OnFocus => "onfocus",
    OnHashChange => "onhashchange", OnInput => "oninput", OnInvalid => "oninvalid", OnKeyDown => "onkeydown",
    OnKeyPress => "onkeypress", OnKeyUp => "onkeyup", OnLoad => "onload", OnLoadedData => "onloadeddata",
    OnLoadedMetaData => "onloadedmetadata", OnLoadStart => "onloadstart", OnMouseDown => "onmousedown",
    OnMouseMove => "onmousemove", OnMouseOut => "onmouseout", OnMouseOver => "onmouseover", OnMouseUp => "onmouseup",
    OnMouseWheel => "onmousewheel", OnOffline => "onoffline", OnOnline => "ononline", OnPageHide => "onpagehide",
    OnPageShow => "onpageshow", OnPaste => "onpaste", OnPause => "onpause", OnPlay => "onplay",
    OnPlaying => "onplaying", OnPopState => "onpopstate", OnProgress => "onprogress", OnRateChange => "onratechange",
    OnRest => "onreset", OnResize => "onresize", OnScroll => "onscroll", OnSearch => "onsearch",
    OnSeeked => "onseeked", OnSeeking => "onseeking", OnSelect => "onselect", OnStalled => "onstalled",
    OnStorage => "onstorage", OnSubmit => "onsubmit", OnSuspend => "onsuspend", OnTimeUpdate => "ontimeupdate",
    OnToggle => "ontoggle", OnUnload => "onunload", OnVolumeChange => "onvolumechange", OnWaiting => "onwaiting",
    OnWheel => "onwheel", Open => "open", Optimum => "optimum", Pattern => "pattern", Placeholder => "placeholder",
    Poster => "poster", Preload => "preload", ReadOnly => "readonly", Rel => "rel", Required => "required",
    Reversed => "reversed", Rows => "rows", RowSpan => "rowspan", Sandbox => "sandbox", Scope => "scope",
    Selected => "selected", Shape => "shape", Size => "size", Span => "span", SpellCheck => "spellcheck",
    Src => "src", SrcDoc => "srcdoc", SrcLang => "srclang", SrcSet => "srcset", Start => "start",
    Step => "step", Style => "style", TabIndex => "tabindex", Target => "target", Title => "title",
    Translate => "translate", Type => "type", UseMap => "usemap", Value => "value", Width => "width",
    Wrap => "wrap",

    // SVG
    // https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute
    AccentHeight => "accent-height", Accumulate => "accumulate", Additive => "additive",
    AlignmentBaseline => "alignment-baseline", AllowReorder => "allowReorder", Amplitude => "amplitude",
    ArabicForm => "arabic-form", Ascent => "ascent", AttributeName => "attributeName", AttributeType => "attributeType",
    AutoReverse => "autoReverse", Azimuth => "azimumth", BaseFrequency => "baseFrequency", BaselineShift => "baseline-shift",
    BaseProfile => "baseProfile", Bbox => "bbox", Begin => "begin", Bias => "bias", By => "by",
    CalcMode => "calcMode", CapHeight => "cap-height", Clip => "clip",
    // todo fill in rest from link above.

    Path => "path", D => "d", Xmlns => "xmlns", ViewBox => "viewBox", Fill => "fill"
}

/// Similar to tag population.
/// // Tod: DRY with At (almost identical), Ev, and similar to Tag.
macro_rules! make_styles {
    // Create shortcut macros for any element; populate these functions in this module.
    { $($st_camel:ident => $st:expr),+ } => {

        /// The St enum restricts element-creation to only valid styles.
        #[derive(Clone, Debug, PartialEq, Eq, Hash)]
        pub enum St {
            $(
                $st_camel,
            )+
            Custom(String)
        }

        impl St {
            pub fn as_str(&self) -> &str {
                match self {
                    $ (
                        St::$st_camel => $st,
                    ) +
                    St::Custom(val) => &val
                }
            }
        }

        impl From<&str> for St {
            fn from(st: &str) -> Self {
                match st {
                    $ (
                          $st => St::$st_camel,
                    ) +
                    _ => {
                        crate::log(&format!("Can't find this attribute: {}", st));
                        St::Background
                    }
                }
            }
        }
        impl From<String> for St {
            fn from(st: String) -> Self {
                match st.as_ref() {
                    $ (
                          $st => St::$st_camel,
                    ) +
                    _ => {
                        crate::log(&format!("Can't find this attribute: {}", st));
                        St::Background
                    }
                }
            }
        }

    }
}

// todo finish and implement this.
// Comprehensive list: https://developer.mozilla.org/en-US/docs/Web/CSS/Reference
// Most common: https://developer.mozilla.org/en-US/docs/Web/CSS/CSS_Properties_Reference
make_styles! {
   AdditiveSymbols => "additive-symbols", AlignContent => "align-content", AlignItems => "align-items",
   AlignSelf => "align-self", All => "all", Angle => "angle", Animation => "animation", AnimationDelay => "animation-delay",
   AnimationDirection => "animation-direction", AnimationDuration => "animation-duration",

   AnimationFillMode => "animation-fill-mode", AnimationIterationCount => "animation-iteration-count",
   AnimationName => "animation-name", AnimationPlayState => "animation-play-state",

   // Most common
   Background => "background", BackgroundAttachment => "background-attachment", BackgroundColor => "background-color",
   BackgroundImage => "background-image", BackgroundPosition => "background-position", BackgroundRepeat => "background-repeat",
   Border => "border", BorderBottom => "border-bottom", BorderBottomColor => "border-bottom-color",
   BorderBottomStyle => "border-bottom-style", BorderBottomWidth => "border-bottom-width", BorderColor => "border-color"
}

/// A thinly-wrapped `HashMap` holding DOM attributes
#[derive(Clone, Debug, PartialEq)]
pub struct Attrs {
    // We use an IndexMap instead of HashMap here, and in Style, to preserve order.
    pub vals: IndexMap<At, AtValue>,
}

/// Create an HTML-compatible string representation
impl fmt::Display for Attrs {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let string = self
            .vals
            .iter()
            .filter_map(|(k, v)| match v {
                AtValue::Ignored => None,
                AtValue::None => Some(k.as_str().to_string()),
                AtValue::Some(value) => Some(format!("{}=\"{}\"", k.as_str(), value)),
            })
            .collect::<Vec<_>>()
            .join(" ");
        write!(f, "{}", string)
    }
}

impl Attrs {
    pub const fn new(vals: IndexMap<At, AtValue>) -> Self {
        Self { vals }
    }

    pub fn empty() -> Self {
        Self {
            vals: IndexMap::new(),
        }
    }

    /// Convenience function. Ideal when there's one id, and no other attrs.
    /// Generally called with the id! macro.
    pub fn from_id(name: impl Into<AtValue>) -> Self {
        let mut result = Self::empty();
        result.add(At::Id, name.into());
        result
    }

    /// Add a new key, value pair
    pub fn add(&mut self, key: At, val: impl Into<AtValue>) {
        self.vals.insert(key, val.into());
    }

    /// Add multiple values for a single attribute. Useful for classes.
    pub fn add_multiple(&mut self, key: At, items: &[&str]) {
        self.add(
            key,
            &items
                .iter()
                .filter_map(|item| {
                    if item.is_empty() {
                        None
                    } else {
                        #[allow(clippy::useless_asref)]
                        Some(item.as_ref())
                    }
                })
                .collect::<Vec<&str>>()
                .join(" "),
        );
    }

    /// Combine with another Attrs
    pub fn merge(&mut self, other: Self) {
        for (other_key, other_value) in other.vals {
            match self.vals.get_mut(&other_key) {
                Some(original_value) => {
                    Self::merge_attribute_values(&other_key, original_value, other_value);
                }
                None => {
                    self.vals.insert(other_key, other_value);
                }
            }
        }
    }

    fn merge_attribute_values(
        key: &At,
        mut original_value: &mut AtValue,
        mut other_value: AtValue,
    ) {
        match (key, &mut original_value, &mut other_value) {
            (At::Class, AtValue::Some(original), AtValue::Some(other)) => {
                if !original.is_empty() {
                    original.push(' ');
                }
                original.push_str(other);
            }
            (..) => *original_value = other_value,
        }
    }
}

/// Handle Style separately from Attrs, since it commonly involves multiple parts,
/// and has a different semantic meaning.
#[derive(Clone, Debug, PartialEq)]
pub struct Style {
    // todo enum for key?
    pub vals: IndexMap<Cow<'static, str>, CSSValue>,
}

impl Style {
    pub const fn new(vals: IndexMap<Cow<'static, str>, CSSValue>) -> Self {
        Self { vals }
    }

    pub fn empty() -> Self {
        Self {
            vals: IndexMap::new(),
        }
    }

    pub fn add(&mut self, key: impl Into<Cow<'static, str>>, val: impl Into<CSSValue>) {
        self.vals.insert(key.into(), val.into());
    }

    /// Combine with another Style; if there's a conflict, use the other one.
    pub fn merge(&mut self, other: Self) {
        self.vals.extend(other.vals.into_iter());
    }
}

/// Output style as a string, as would be set in the DOM as the attribute value
/// for 'style'. Eg: "display: flex; font-size: 1.5em"
impl fmt::Display for Style {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let string = if self.vals.keys().len() > 0 {
            self.vals
                .iter()
                .filter_map(|(k, v)| match v {
                    CSSValue::Ignored => None,
                    CSSValue::Some(value) => Some(format!("{}:{}", k, value)),
                })
                .collect::<Vec<_>>()
                .join(";")
        } else {
            String::new()
        };
        write!(f, "{}", string)
    }
}

// Populate tags using a macro, to reduce code repetition.
// The tag enum primarily exists to ensure only valid elements are allowed.
// We leave out non-body tags like html, meta, title, and body.
macro_rules! make_tags {
    // Create shortcut macros for any element; populate these functions in this module.
    { $($tag_camel:ident => $tag:expr),+ } => {

        /// The Tag enum restricts element-creation to only valid tags, as defined here:
        /// [https://developer.mozilla.org/en-US/docs/Web/HTML/Element](https://developer.mozilla.org/en-US/docs/Web/HTML/Element)
        #[derive(Clone, Debug, PartialEq)]
        pub enum Tag {
            Custom(String),
            $(
                $tag_camel,
            )+
        }

        impl Tag {
            pub fn as_str(&self) -> &str {
                match self {
                    Tag::Custom(name) => &name,
                    $ (
                        Tag::$tag_camel => $tag,
                    ) +
                }
            }
        }

        impl From<String> for Tag {
            fn from(tag: String) -> Self {
                match tag.as_ref() {
                    $ (
                          $tag => Tag::$tag_camel,
                    ) +
                    _ => {
                        Tag::Span
                    }
                }
            }
        }
    }
}

// Comprehensive lists:
// - https://developer.mozilla.org/en-US/docs/Web/HTML/Element
// - https://developer.mozilla.org/en-US/docs/Web/SVG/Element
// Grouped here by category on Mozilla's pages, linked above.
make_tags! {
    // -------- Standard HTML Tags -------- //

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

    Content => "content", Element => "element", Shadow => "shadow", Slot => "slot", Template => "template",

    // -------- SVG Tags -------- //

    // Animation elements
    Animate => "animate", AnimateColor => "animateColor", AnimateMotion => "animateMotion",
    AnimateTransform => "animateTransform", Discard => "discard", Mpath => "mpath", Set => "set",

    // Shape elements
    Circle => "circle", Ellipse => "ellipse", Line => "line", Polygon => "polygon",
    Polyline => "polyline", Rect => "rect", Mesh => "mesh", Path => "path",

    // Container elements
    Defs => "defs", G => "g", Marker => "marker", Mask => "mask", MissingGlyph => "missing-glyph",
    Pattern => "pattern", Svg => "svg", Switch => "switch", Symbol => "symbol", Unknown => "unknown",

    // Descriptive elements
    Desc => "desc", Metadata => "metadata", Title => "title",

    // Filter primitive elements
    FeBlend             => "feBlend",
    FeColorMatrix       => "feColorMatrix",
    FeComponentTransfer => "feComponentTransfer",
    FeComposite         => "feComposite",
    FeConvolveMatrix    => "feConvolveMatrix",
    FeDiffuseLighting   => "feDiffuseLighting",
    FeDisplacementMap   => "feDisplacementMap",
    FeDropShadow        => "feDropShadow",
    FeFlood             => "feFlood",
    FeFuncA             => "feFuncA",
    FeFuncB             => "feFuncB",
    FeFuncG             => "feFuncG",
    FeFuncR             => "feFuncR",
    FeGaussianBlur      => "feGaussianBlur",
    FeImage             => "feImage",
    FeMerge             => "feMerge",
    FeMergeNode         => "feMergeNode",
    FeMorphology        => "feMorphology",
    FeOffset            => "feOffset",
    FeSpecularLighting  => "feSpecularLighting",
    FeTile              => "feTile",
    FeTurbulence        => "feTurbulence",

    // Light source elements
    FeDistantLight => "feDistantLight", FePointLight => "fePointLight",  FeSpotLight => "feSpotLight",

    // Font elements
    Font => "font",
    FontFace => "font-face",
    FontFaceFormat => "font-face-format",
    FontFaceName => "font-face-name",
    FontFaceSrc => "font-face-src",
    FontFaceUri => "font-face-uri",
    HKern => "hkern",
    VKern => "vkern",

    // Gradient elements
    LinearGradient => "linearGradient", MeshGradient => "meshGradient",
    RadialGradient => "radialGradient", Stop => "stop",

    // Graphics elements
    Image => "image",

    // Graphics referencing elements
    Use => "use",

    // Paint server elements
    Hatch => "hatch", SolidColor => "solidcolor",

    // Text content elements
    AltGlyph => "altGlyph", AltGlyphDef => "altGlyphDef", AltGlyphItem => "altGlyphItem", Glyph => "glyph",
    GlyphRef => "glyphRef", TextPath => "textPath", Text => "text", TRef => "tref", TSpan => "tspan",

    // Uncategorized elements
    ClipPath => "clipPath", ColorProfile => "color-profile", Cursor => "cursor", Filter => "filter",
    ForeignObject => "foreignObject", HatchPath => "hatchpath", MeshPatch => "meshpatch", MeshRow => "meshrow",
    Style => "style", View => "view",

    // A custom placeholder tag, for internal use
    Placeholder => "placeholder"
}

pub trait View<Ms: 'static> {
    fn els(self) -> Vec<Node<Ms>>;
}

impl<Ms> View<Ms> for El<Ms> {
    fn els(self) -> Vec<Node<Ms>> {
        vec![Node::Element(self)]
    }
}

impl<Ms> View<Ms> for Vec<El<Ms>> {
    fn els(self) -> Vec<Node<Ms>> {
        self.into_iter().map(Node::Element).collect()
    }
}

impl<Ms: 'static> View<Ms> for Node<Ms> {
    fn els(self) -> Vec<Node<Ms>> {
        vec![self]
    }
}

impl<Ms: 'static> View<Ms> for Vec<Node<Ms>> {
    fn els(self) -> Vec<Node<Ms>> {
        self
    }
}

/// For representing text nodes.
#[derive(Clone, Debug)]
pub struct Text {
    pub text: Cow<'static, str>,
    pub node_ws: Option<web_sys::Node>,
}

impl PartialEq for Text {
    fn eq(&self, other: &Self) -> bool {
        self.text == other.text
    }
}

impl Text {
    pub fn new(text: impl Into<Cow<'static, str>>) -> Self {
        Self {
            text: text.into(),
            node_ws: None,
        }
    }
}

/// An component in our virtual DOM. Related to, but different from
/// [DOM Nodes](https://developer.mozilla.org/en-US/docs/Web/API/Node/nodeType)
#[derive(Clone, Debug, PartialEq)]
pub enum Node<Ms: 'static> {
    Element(El<Ms>),
    //    Svg(El<Ms>),  // May be best to handle using namespace field on El
    Text(Text),
    Empty,
}

impl<Ms> Node<Ms> {
    fn is_text(&self) -> bool {
        if let Node::Text(_) = self {
            true
        } else {
            false
        }
    }

    pub fn new_text(text: impl Into<Cow<'static, str>>) -> Self {
        Node::Text(Text::new(text))
    }

    /// See `El::from_markdown`
    pub fn from_markdown(markdown: &str) -> Vec<Node<Ms>> {
        El::from_markdown(markdown)
    }

    /// See `El::from_html`
    pub fn from_html(html: &str) -> Vec<Node<Ms>> {
        El::from_html(html)
    }

    /// See `El::add_child`
    pub fn add_child(self, node: Node<Ms>) -> Self {
        if let Node::Element(el) = self {
            Node::Element(el.add_child(node))
        } else {
            self
        }
    }

    /// See `El::add_attr`
    pub fn add_attr(self, key: impl Into<Cow<'static, str>>, val: impl Into<AtValue>) -> Self {
        if let Node::Element(el) = self {
            Node::Element(el.add_attr(key, val))
        } else {
            self
        }
    }

    /// /// See `El::add_class``
    pub fn add_class(self, name: impl Into<Cow<'static, str>>) -> Self {
        if let Node::Element(el) = self {
            Node::Element(el.add_class(name))
        } else {
            self
        }
    }

    /// See `El::add_style`
    pub fn add_style(self, key: impl Into<Cow<'static, str>>, val: impl Into<CSSValue>) -> Self {
        if let Node::Element(el) = self {
            Node::Element(el.add_style(key, val))
        } else {
            self
        }
    }

    /// See `El::add_listener`
    pub fn add_listener(self, listener: Listener<Ms>) -> Self {
        if let Node::Element(el) = self {
            Node::Element(el.add_listener(listener))
        } else {
            self
        }
    }

    /// See `El::add_text`
    pub fn add_text(self, text: impl Into<Cow<'static, str>>) -> Self {
        if let Node::Element(el) = self {
            Node::Element(el.add_text(text))
        } else {
            self
        }
    }

    /// See `El::replace_text`
    pub fn replace_text(self, text: impl Into<Cow<'static, str>>) -> Self {
        if let Node::Element(el) = self {
            Node::Element(el.replace_text(text))
        } else {
            self
        }
    }

    /// See `El::get_text`
    pub fn get_text(&self) -> String {
        match self {
            Node::Element(el) => el.get_text(),
            Node::Text(text) => text.text.to_string(),
            _ => "".to_string(),
        }
    }
}

impl<Ms: 'static, OtherMs: 'static> MessageMapper<Ms, OtherMs> for Node<Ms> {
    type SelfWithOtherMs = Node<OtherMs>;
    /// See note on impl for El
    fn map_message(self, f: impl FnOnce(Ms) -> OtherMs + 'static + Clone) -> Node<OtherMs> {
        match self {
            Node::Element(el) => Node::Element(el.map_message(f)),
            Node::Text(text) => Node::Text(text),
            Node::Empty => Node::Empty,
        }
    }
}

impl<Ms: 'static, OtherMs: 'static> MessageMapper<Ms, OtherMs> for Vec<Node<Ms>> {
    type SelfWithOtherMs = Vec<Node<OtherMs>>;
    fn map_message(self, f: impl FnOnce(Ms) -> OtherMs + 'static + Clone) -> Vec<Node<OtherMs>> {
        self.into_iter()
            .map(|node| node.map_message(f.clone()))
            .collect()
    }
}

/// An component in our virtual DOM.
#[derive(Debug)] // todo: Custom debug implementation where children are on new lines and indented.
pub struct El<Ms: 'static> {
    // Ms is a message type, as in part of TEA.
    // We call this 'El' instead of 'Element' for brevity, and to prevent
    // confusion with web_sys::Element.
    pub tag: Tag,
    pub attrs: Attrs,
    pub style: Style,
    pub listeners: Vec<Listener<Ms>>,
    pub children: Vec<Node<Ms>>,
    /// The actual web element/node
    pub node_ws: Option<web_sys::Node>,
    pub namespace: Option<Namespace>,
    pub hooks: LifecycleHooks<Ms>,
}

type _HookFn = Box<dyn FnMut(&web_sys::Node)>; // todo

pub struct LifecycleHooks<Ms> {
    pub did_mount: Option<DidMount<Ms>>,
    pub did_update: Option<DidUpdate<Ms>>,
    pub will_unmount: Option<WillUnmount<Ms>>,
}

impl<Ms> LifecycleHooks<Ms> {
    const fn new() -> Self {
        Self {
            did_mount: None,
            did_update: None,
            will_unmount: None,
        }
    }
}

impl<Ms> fmt::Debug for LifecycleHooks<Ms> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "LifecycleHooks {{ did_mount:{:?}, did_update:{:?}, will_unmount:{} }}",
            events::fmt_hook_fn(&self.did_mount),
            events::fmt_hook_fn(&self.did_update),
            events::fmt_hook_fn(&self.will_unmount)
        )
    }
}

impl<Ms: 'static, OtherMs: 'static> MessageMapper<Ms, OtherMs> for LifecycleHooks<Ms> {
    type SelfWithOtherMs = LifecycleHooks<OtherMs>;
    fn map_message(self, f: impl FnOnce(Ms) -> OtherMs + 'static + Clone) -> Self::SelfWithOtherMs {
        LifecycleHooks {
            did_mount: self.did_mount.map(|d| DidMount {
                actions: d.actions,
                message: d.message.map(f.clone()),
            }),
            did_update: self.did_update.map(|d| DidUpdate {
                actions: d.actions,
                message: d.message.map(f.clone()),
            }),
            will_unmount: self.will_unmount.map(|d| WillUnmount {
                actions: d.actions,
                message: d.message.map(f),
            }),
        }
    }
}

impl<Ms: 'static, OtherMs: 'static> MessageMapper<Ms, OtherMs> for El<Ms> {
    type SelfWithOtherMs = El<OtherMs>;
    /// Maps an element's message to have another message.
    ///
    /// This allows third party components to integrate with your application without
    /// having to know about your Msg type beforehand.
    ///
    /// # Note
    /// There is an overhead to calling this versus keeping all messages under one type.
    /// The deeper the nested structure of children, the more time this will take to run.
    fn map_message(self, f: impl FnOnce(Ms) -> OtherMs + 'static + Clone) -> El<OtherMs> {
        El {
            tag: self.tag,
            attrs: self.attrs,
            style: self.style,
            listeners: self
                .listeners
                .into_iter()
                .map(|l| l.map_message(f.clone()))
                .collect(),
            children: self
                .children
                .into_iter()
                .map(|c| c.map_message(f.clone()))
                .collect(),
            node_ws: self.node_ws,
            namespace: self.namespace,
            hooks: self.hooks.map_message(f),
        }
    }
}

impl<Ms: 'static, OtherMs: 'static> MessageMapper<Ms, OtherMs> for Vec<El<Ms>> {
    type SelfWithOtherMs = Vec<El<OtherMs>>;
    fn map_message(self, f: impl FnOnce(Ms) -> OtherMs + 'static + Clone) -> Vec<El<OtherMs>> {
        self.into_iter()
            .map(|el| el.map_message(f.clone()))
            .collect()
    }
}

impl<Ms> El<Ms> {
    /// Create an empty element, specifying only the tag
    pub fn empty(tag: Tag) -> Self {
        Self {
            tag,
            attrs: Attrs::empty(),
            style: Style::empty(),
            listeners: Vec::new(),
            children: Vec::new(),
            node_ws: None,
            namespace: None,
            hooks: LifecycleHooks::new(),
        }
    }

    /// Create an empty SVG element, specifying only the tag
    pub fn empty_svg(tag: Tag) -> Self {
        let mut el = El::empty(tag);
        el.namespace = Some(Namespace::Svg);
        el
    }

    // todo: Return El instead of Node here? (Same with from_html)
    /// Create elements from a markdown string.
    pub fn from_markdown(markdown: &str) -> Vec<Node<Ms>> {
        let parser = pulldown_cmark::Parser::new(markdown);
        let mut html_text = String::new();
        pulldown_cmark::html::push_html(&mut html_text, parser);

        Self::from_html(&html_text)
    }

    /// Create elements from an HTML string.
    pub fn from_html(html: &str) -> Vec<Node<Ms>> {
        // Create a web_sys::Element, with our HTML wrapped in a (arbitrary) span tag.
        // We allow web_sys to parse into a DOM tree, then analyze the tree to create our vdom
        // element.
        let wrapper = util::document()
            .create_element("placeholder")
            .expect("Problem creating web-sys element");
        wrapper.set_inner_html(html);

        let mut result = Vec::new();
        let children = wrapper.child_nodes();
        for i in 0..children.length() {
            let child = children
                .get(i)
                .expect("Can't find child in raw html element.");

            if let Some(child_vdom) = websys_bridge::node_from_ws(&child) {
                result.push(child_vdom)
            }
        }
        result
    }

    /// Add a new child to the element
    pub fn add_child(mut self, element: Node<Ms>) -> Self {
        self.children.push(element);
        self
    }

    /// Add an attribute (eg class, or href)
    pub fn add_attr(mut self, key: impl Into<Cow<'static, str>>, val: impl Into<AtValue>) -> Self {
        self.attrs
            .vals
            .insert(key.into().as_ref().into(), val.into());
        self
    }

    /// Add a class. May be cleaner than `add_attr`
    pub fn add_class(mut self, name: impl Into<Cow<'static, str>>) -> Self {
        let name = name.into();
        self.attrs
            .vals
            .entry(At::Class)
            .and_modify(|at_value| match at_value {
                AtValue::Some(v) => {
                    if !v.is_empty() {
                        *v += " ";
                    }
                    *v += name.as_ref();
                }
                _ => *at_value = AtValue::Some(name.clone().into_owned()),
            })
            .or_insert(AtValue::Some(name.into_owned()));
        self
    }

    /// Add a new style (eg display, or height)
    pub fn add_style(
        mut self,
        key: impl Into<Cow<'static, str>>,
        val: impl Into<CSSValue>,
    ) -> Self {
        self.style.vals.insert(key.into(), val.into());
        self
    }

    /// Add a new listener
    pub fn add_listener(mut self, listener: Listener<Ms>) -> Self {
        self.listeners.push(listener);
        self
    }

    /// Add a text node to the element. (ie between the HTML tags).
    pub fn add_text(mut self, text: impl Into<Cow<'static, str>>) -> Self {
        self.children.push(Node::Text(Text::new(text)));
        self
    }

    /// Replace the element's text.
    /// Removes all text nodes from element, then adds the new one.
    pub fn replace_text(mut self, text: impl Into<Cow<'static, str>>) -> Self {
        self.children.retain(|node| !node.is_text());
        self.children.push(Node::new_text(text));
        self
    }

    // Pull text from child text nodes
    pub fn get_text(&self) -> String {
        self.children
            .iter()
            .filter_map(|child| match child {
                Node::Text(text_node) => Some(text_node.text.to_string()),
                _ => None,
            })
            .collect()
    }
}

/// Allow the user to clone their Els. Note that there's no easy way to clone the
/// closures within listeners or lifestyle hooks, so we omit them.
impl<Ms> Clone for El<Ms> {
    fn clone(&self) -> Self {
        Self {
            tag: self.tag.clone(),
            attrs: self.attrs.clone(),
            style: self.style.clone(),
            // todo: Put this back - removed during troubleshooting Node change
            //            children: self.children.clone(),
            children: Vec::new(),
            node_ws: self.node_ws.clone(),
            listeners: Vec::new(),
            namespace: self.namespace.clone(),
            hooks: LifecycleHooks::new(),
        }
    }
}

impl<Ms> PartialEq for El<Ms> {
    fn eq(&self, other: &Self) -> bool {
        // todo Again, note that the listeners check only checks triggers.
        // Don't check children.
        self.tag == other.tag
            && self.attrs == other.attrs
            && self.style == other.style
            && self.listeners == other.listeners
            && self.namespace == other.namespace
    }
}

pub struct DidMount<Ms> {
    pub actions: Box<dyn FnMut(&web_sys::Node)>,
    pub message: Option<Ms>,
}

pub struct DidUpdate<Ms> {
    pub actions: Box<dyn FnMut(&web_sys::Node)>,
    pub message: Option<Ms>,
}

pub struct WillUnmount<Ms> {
    pub actions: Box<dyn FnMut(&web_sys::Node)>,
    pub message: Option<Ms>,
}

/// A constructor for `DidMount`, to be used in the API
pub fn did_mount<Ms>(mut actions: impl FnMut(&web_sys::Node) + 'static) -> DidMount<Ms> {
    let closure = move |el: &web_sys::Node| actions(el);
    DidMount {
        actions: Box::new(closure),
        message: None,
    }
}

/// A constructor for `DidUpdate`, to be used in the API
pub fn did_update<Ms>(mut actions: impl FnMut(&web_sys::Node) + 'static) -> DidUpdate<Ms> {
    let closure = move |el: &web_sys::Node| actions(el);
    DidUpdate {
        actions: Box::new(closure),
        message: None,
    }
}

/// A constructor for `WillUnmount`, to be used in the API
pub fn will_unmount<Ms>(mut actions: impl FnMut(&web_sys::Node) + 'static) -> WillUnmount<Ms> {
    let closure = move |el: &web_sys::Node| actions(el);
    WillUnmount {
        actions: Box::new(closure),
        message: None,
    }
}

#[cfg(test)]
pub mod tests {
    use wasm_bindgen_test::*;
    wasm_bindgen_test_configure!(run_in_browser);

    use super::*;

    use crate as seed;
    // required for macros to work.
    use crate::{patch, vdom};
    use std::collections::HashSet;
    use wasm_bindgen::{JsCast, JsValue};
    use web_sys::Element;

    #[derive(Debug)]
    enum Msg {}

    struct Model {}

    fn create_app() -> seed::App<Msg, Model, Node<Msg>> {
        seed::App::build(|_,_| Model {}, |_, _, _| (), |_| seed::empty())
            // mount to the element that exists even in the default test html
            .mount(util::body())
            .finish()
    }

    fn el_to_websys(mut node: Node<Msg>) -> web_sys::Node {
        let document = crate::util::document();
        let parent = document.create_element("div").unwrap();
        let app = create_app();

        patch::patch(
            &document,
            seed::empty(),
            &mut node,
            &parent,
            None,
            &vdom::Mailbox::new(|_: Msg| {}),
            &app,
        );

        if let Node::Element(el) = node {
            el.node_ws.unwrap()
        } else {
            panic!("not an El node")
        }
    }

    /// Assumes Node is an Element
    fn get_node_html(node: &web_sys::Node) -> String {
        node.dyn_ref::<Element>().unwrap().outer_html()
    }

    /// Assumes Node is an Element
    fn get_node_attrs(node: &web_sys::Node) -> IndexMap<String, String> {
        let element = node.dyn_ref::<Element>().unwrap();
        element
            .get_attribute_names()
            .values()
            .into_iter()
            .map(|item_res| {
                item_res.map(|item| {
                    let name = item.as_string().unwrap();
                    let value = element.get_attribute(&name).unwrap();
                    (name, value)
                })
            })
            .collect::<Result<IndexMap<String, String>, JsValue>>()
            .unwrap()
    }

    #[wasm_bindgen_test]
    pub fn single_div() {
        let expected = "<div>test</div>";

        let node = el_to_websys(div!["test"]);

        assert_eq!(expected, get_node_html(&node));
    }

    #[wasm_bindgen_test]
    pub fn nested_divs() {
        let expected = "<section><div><div><h1>huge success</h1></div><p>\
                        I'm making a note here</p></div><span>This is a triumph</span></section>";

        let node = el_to_websys(section![
            div![div![h1!["huge success"]], p!["I'm making a note here"]],
            span!["This is a triumph"]
        ]);

        assert_eq!(expected, get_node_html(&node));
    }

    #[wasm_bindgen_test]
    pub fn attrs_work() {
        let expected = "<section src=\"https://seed-rs.org\" class=\"biochemistry\">ok</section>";
        let expected2 = "<section class=\"biochemistry\" src=\"https://seed-rs.org\">ok</section>";

        let node = el_to_websys(section![
            attrs! {"class" => "biochemistry"; "src" => "https://seed-rs.org"},
            "ok"
        ]);

        let actual_html = get_node_html(&node);
        assert!(expected == actual_html || expected2 == actual_html);
    }

    /// Tests that multiple attribute sections with unconflicting attributes are handled correctly
    #[wasm_bindgen_test]
    pub fn merge_different_attrs() {
        let node = el_to_websys(a![
            id!["my_id"],
            style!["background-color" => "red"],
            class!["my_class1"],
            attrs![
                At::Href => "#my_ref";
            ],
            attrs![
                At::Name => "whatever";
            ],
        ]);

        let mut expected = IndexMap::new();
        expected.insert("id".to_string(), "my_id".to_string());
        expected.insert("style".to_string(), "background-color:red".to_string());
        expected.insert("class".to_string(), "my_class1".to_string());
        expected.insert("href".to_string(), "#my_ref".to_string());
        expected.insert("name".to_string(), "whatever".to_string());
        assert_eq!(expected, get_node_attrs(&node));
    }

    /// Tests that multiple class attributes are handled correctly
    #[wasm_bindgen_test]
    pub fn merge_classes() {
        let node = el_to_websys(
            a![
                class!["", "cls_1", "cls_2"],
                class!["cls_3", "", ""],
                attrs![
                    At::Class => "cls_4 cls_5";
                ],
                class![
                    "cls_6"
                    "cls_7" => false
                    "cls_8" => 1 == 1
                ]
            ]
            .add_class("cls_9"),
        );

        let mut expected = IndexMap::new();
        expected.insert(
            "class".to_string(),
            "cls_1 cls_2 cls_3 cls_4 cls_5 cls_6 cls_8 cls_9".to_string(),
        );
        assert_eq!(expected, get_node_attrs(&node));
    }

    /// Tests that multiple style sections are handled correctly
    #[wasm_bindgen_test]
    pub fn merge_styles() {
        let node = el_to_websys(a![
            style!["border-top" => "1px"; "border-bottom" => "red"],
            style!["background-color" => "blue"],
        ]);

        let attrs = get_node_attrs(&node);
        let actual_styles = attrs["style"]
            .split(";")
            .map(|x| x.to_string())
            .collect::<HashSet<String>>();

        let mut expected = HashSet::new();
        expected.insert("border-top:1px".to_string());
        expected.insert("border-bottom:red".to_string());
        expected.insert("background-color:blue".to_string());
        assert_eq!(expected, actual_styles);
    }

    /// Tests that multiple id attributes are handled correctly (the last ID should override the
    /// previous values)
    #[wasm_bindgen_test]
    pub fn merge_id() {
        let node = el_to_websys(a![
            id!["my_id1"],
            attrs![
                At::Id => "my_id2";
            ]
        ]);

        let mut expected = IndexMap::new();
        expected.insert("id".to_string(), "my_id2".to_string());
        assert_eq!(expected, get_node_attrs(&node));
    }

    /// Tests that method `replace_text` removes all text nodes and then adds a new one
    #[wasm_bindgen_test]
    pub fn replace_text() {
        let expected = "<div><span>bbb</span>xxx</div>";

        let node =
            el_to_websys(div!["aaa", span!["bbb"], plain!["ccc"], "ddd"].replace_text("xxx"));

        assert_eq!(expected, get_node_html(&node));
    }
}
