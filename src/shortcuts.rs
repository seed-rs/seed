//! This file exports helper macros for element creation, populated by a higher-level macro,
//! and macros for creating the parts of elements. (attrs, style, events)

// @TODO merge with `pub use` & `prelude` in `lib.rs` and `browser::util`?

use crate::virtual_dom::{At, Attrs};
use wasm_bindgen::JsValue;

/// Copied from [https://github.com/rust-lang/rust/issues/35853](https://github.com/rust-lang/rust/issues/35853)
macro_rules! with_dollar_sign {
    ($($body:tt)*) => {
        macro_rules! __with_dollar_sign { $($body)* }
        __with_dollar_sign!($);
    }
}

#[macro_export]
/// Create struct `Urls`. It's useful especially for building `Url`s in nested modules.
///
/// # Example
///
/// ```rust,no_run
///
/// mod page;
/// const ADMIN: &str = "admin";
///
/// fn init(url: Url, _: &mut impl Orders<Msg>) -> Model {
///     Model {
///         base_url: url.to_base_url(),
///     }
/// }
///
/// // ------ ------
/// //     Urls
/// // ------ ------
///
/// struct_urls!();
/// impl<'a> Urls<'a> {
///     pub fn home(self) -> Url {
///         self.base_url()
///     }
///     pub fn admin_urls(self) -> page::admin::Urls<'a> {
///         page::admin::Urls::new(self.base_url().add_path_part(ADMIN))
///     }
/// }
///
/// fn view(model: &Model) -> Node<Msg> {
///     a![
///         attrs!{ At::Href => Urls::new(base_url).home() }
///     ]
/// }
/// ```
macro_rules! struct_urls {
    () => {
        pub struct Urls<'a> {
            base_url: std::borrow::Cow<'a, $crate::browser::Url>,
        }

        impl<'a> Urls<'a> {
            /// Create a new `Urls` instance.
            ///
            /// # Example
            ///
            /// ```rust,no_run
            /// Urls::new(base_url).home()
            /// ```
            pub fn new(base_url: impl Into<std::borrow::Cow<'a, $crate::browser::Url>>) -> Self {
                Self {
                    base_url: base_url.into(),
                }
            }

            /// Return base `Url`. If `base_url` isn't owned, it will be cloned.
            ///
            /// # Example
            ///
            /// ```rust,no_run
            /// pub fn admin_urls(self) -> page::admin::Urls<'a> {
            ///     page::admin::Urls::new(self.base_url().add_path_part(ADMIN))
            /// }
            /// ```
            pub fn base_url(self) -> $crate::browser::Url {
                self.base_url.into_owned()
            }
        }
    };
}

/// Create macros exposed to the package that allow shortcuts for Dom elements.
/// In the matching pattern below, we match the name we want to use with the name under
/// the `seed::virtual_dom::Tag` enum. Eg the div! macro uses `seed::virtual_dom::Tag::Div`.
macro_rules! element {
    // Create shortcut macros for any element; populate these functions in this module.
    ($($Tag:ident => $Tag_camel:ident);+) => {
        // This replaces $d with $ in the inner macro.
        with_dollar_sign! {
            ($d:tt) => {
                $(
                    #[macro_export]
                    macro_rules! $Tag {
                        ( $d($d part:expr),* $d(,)? ) => {
                            {
                                #[allow(unused_mut)]
                                let mut el = El::empty($crate::virtual_dom::Tag::$Tag_camel);
                                $d (
                                    $d part.update_el(&mut el);
                                )*
                                $crate::virtual_dom::Node::Element(el)
                            }
                        };
                    }
                )+
            }
        }
   }
}
/// Similar to the element! macro above, but with a namespace for svg.
macro_rules! element_svg {
    // Create shortcut macros for any element; populate these functions in this module.
    ($($Tag:ident => $Tag_camel:ident);+) => {
        // This replaces $d with $ in the inner macro.
        with_dollar_sign! {
            ($d:tt) => {
                $(
                    #[macro_export]
                    macro_rules! $Tag {
                        ( $d($d part:expr),* $d(,)? ) => {
                            {
                                #[allow(unused_mut)]
                                let mut el = El::empty_svg($crate::virtual_dom::Tag::$Tag_camel);
                                $d ( $d part.update_el(&mut el); )*
                                $crate::virtual_dom::Node::Element(el)
                            }
                        };
                    }
                )+
            }
        }
   }
}

// @TODO merge with make_tags!
// El must be exposed in the module where this is called for these to work.
element! {
    address => Address; article => Article; aside => Aside; footer => Footer;
    header => Header; h1 => H1;
    h2 => H2; h3 => H3; h4 => H4; h5 => H5; h6 => H6;
    hgroup => Hgroup; main => Main; nav => Nav; section => Section;

    blockquote => BlockQuote;
    dd => Dd; dir => Dir; div => Div; dl => Dl; dt => Dt; figcaption => FigCaption; figure => Figure;
    hr => Hr; li => Li; ol => Ol; p => P; pre => Pre; ul => Ul;

    a => A; abbr => Abbr;
    b => B; bdi => Bdi; bdo => Bdo; br => Br; cite => Cite; code => Code; data => Data;
    dfn => Dfn; em => Em; i => I; kbd => Kbd; mark => Mark; q => Q; rb => Rb;
    rp => Rp; rt => Rt; rtc => Rtc; ruby => Ruby; s => S; samp => Samp; small => Small;
    span => Span; strong => Strong; sub => Sub; sup => Sup; time => Time; tt => Tt;
    u => U; var => Var; wbr => Wbr;

    area => Area; audio => Audio; img => Img; map => Map; track => Track; video => Video;

    applet => Applet; embed => Embed; iframe => Iframe;
    noembed => NoEmbed; object => Object; param => Param; picture => Picture; source => Source;

    canvas => Canvas; noscript => NoScript; Script => Script;

    del => Del; ins => Ins;

    caption => Caption; col => Col; colgroup => ColGroup; table => Table; tbody => Tbody;
    td => Td; tfoot => Tfoot; th => Th; thead => Thead; tr => Tr;

    button => Button; datalist => DataList; fieldset => FieldSet; form => Form; input => Input;
    label => Label; legend => Legend; meter => Meter; optgroup => OptGroup; option => Option;
    output => Output; progress => Progress; select => Select; textarea => TextArea;

    details => Details; dialog => Dialog; menu => Menu; menuitem => MenuItem; summary => Summary;

    content => Content; element => Element; shadow => Shadow; slot => Slot; template => Template
}

// @TODO merge with make_tags!
element_svg! {
    // SVG shape elements
    line_ => Line;  // line is a builtin rust macro.
    rect => Rect; circle => Circle; ellipse => Elipse; polygon => Polygon; polyline => Polyline;
    mesh => Mesh; path => Path;
    // SVG container elements
    defs => Defs; g => G; marker => Marker; mask => Mask;
    // missing-glyph => MissingGlyph; // todo unable to populate with macro due to hyphen
    pattern => Pattern; svg => Svg; switch => Switch; symbol => Symbol; unknown => Unknown;
    // SVG gradient elements
    linearGradient => LinearGradient; radialGradient => RadialGradient; meshGradient => MeshGradient;
    stop => Stop;
    // SVG graphics elements
    image => Image;
    // SVG graphics referencing elements
    r#use => Use;
    // SVG text content elements
    altGlyph => AltGlyph; altGlyphDef => AltGlyphDef; altGlyphItem => AltGlyphItem; glyph => Glyph;
    glyphRef => GlyphRef; textPath => TextPath; text => Text; tref => TRef; tspan => TSpan;
    // SVG uncategorized elements
    clipPath => ClipPath; cursor => Cursor; filter => Filter; foreignObject => ForeignObject;
    hatchpath => HatchPath; meshPatch => MeshPatch; meshrow => MeshRow; view => View;
    // style missing due to conflict with the style macro.
    // colorProfile => ColorProfile;  // todo hypthen-issue
    // SVG animation elements
    animate => Animate; animateColor => AnimateColor; animateMotion => AnimateMotion;
    animateTransform => AnimateTransform; discard => Discard; mpath => Mpath; set => Set;
    // SVG descriptive elements
    desc => Desc; metadata => Metadata; title => Title;
    // SVG filter primitive elements
    feBlend => FeBlend; feColorMatrix => FeColorMatrix; feComponentTransfer => FeComponentTransfer;
    feComposite => FeComposite; feConvolveMatrix => FeConvolveMatrix;
    feDiffuseLighting => FeDiffuseLighting; feDisplacementMap => FeDisplacementMap;
    feDropShadow => FeDropShadow; feFlood => FeFlood; feFuncA => FeFuncA; feFuncB => FeFuncB;
    feFuncG => FeFuncG; feFuncR => FeFuncR; feGaussianBlur => FeGaussianBlur; feImage => FeImage;
    feMerge => FeMerge; feMergeNode => FeMergeNode; feMorphology => FeMorphology;
    feOffset => FeOffset; feSpecularLighting => FeSpecularLighting; feTile => FeTile;
    feTurbulence => FeTurbulence;
    // SVG font elements
    font => Font; hkern => HKern; vkern => VKern;
    // todo many font elements with hyphen issue
    // SVG Paint sever elements
    hatch => Hatch; solidcolor => SolidColor
}

#[macro_export]
macro_rules! empty {
    () => {
        $crate::virtual_dom::Node::Empty
    };
}

#[macro_export]
macro_rules! raw {
    ($raw_html:expr) => {
        Node::from_html($raw_html)
    };
}

#[macro_export]
macro_rules! md {
    ($md:expr) => {
        Node::from_markdown($md)
    };
}

#[macro_export]
macro_rules! plain {
    ($text:expr) => {
        $crate::virtual_dom::Node::new_text($text)
    };
}

#[macro_export]
macro_rules! custom {
    ( $($part:expr),* $(,)? ) => {
        {
            let default_tag_name = "missing-tag-name";
            let mut el = El::empty($crate::virtual_dom::Tag::from(default_tag_name));
            $ ( $part.update_el(&mut el); )*

            if let $crate::virtual_dom::Tag::Custom(tag_name) = &el.tag {
                let tag_changed = tag_name != default_tag_name;
                assert!(tag_changed, "Tag has not been set in `custom!` element. Add e.g. `Tag::from(\"code-block\")`.");
            }

            $crate::virtual_dom::Node::Element(el)
        }
    };
}

/// Provide a shortcut for creating attributes.
#[macro_export]
macro_rules! attrs {
    { $($key:expr => $value:expr $(;)?$(,)?)* } => {
        {
            let mut vals = IndexMap::new();
            $(
                // We can handle arguments of multiple types by using this:
                // Strings, &strs, bools, numbers etc.
                // And cases like `true.as_attr_value()` or `AtValue::Ignored`.
                vals.insert($key.into(), (&$value).into());
            )*
            $crate::virtual_dom::Attrs::new(vals)
        }
     };
}

#[deprecated(since = "0.7.0", note = "use [`C!`](macro.C!.html) instead")]
/// Convenience macro. Ideal when there are multiple classes, and no other attrs.
#[macro_export]
macro_rules! class {
    { $($class:expr $(=> $predicate:expr)? $(,)?)* } => {
        {
            let mut result = $crate::virtual_dom::Attrs::empty();
            let mut classes = Vec::new();
            $(
                // refactor to labeled block once stable (https://github.com/rust-lang/rust/issues/48594)
                (||{
                    $(
                        if !$predicate { return }
                    )?
                    classes.push($class);
                })();
            )*
            result.add_multiple(At::Class, &classes);
            result
        }
     };
}

/// Add classes into the element.
///
/// # Example
///
/// ```rust,no_run
///div![
///    C!["btn", IF!(active => "active")],
///    "Button",
///]
/// ```
#[macro_export]
macro_rules! C {
    ( $($class:expr $(,)?)* ) => {
        {
            let mut all_classes = Vec::new();
            $(
                $crate::shortcuts::_fill_all_classes(&mut all_classes, $class.to_classes());
            )*
            $crate::shortcuts::_all_classes_to_attrs(&all_classes)
        }
    };
}

pub fn _fill_all_classes(all_classes: &mut Vec<String>, classes: Option<Vec<String>>) {
    if let Some(classes) = classes {
        for class in classes {
            if !class.is_empty() {
                all_classes.push(class);
            }
        }
    }
}

pub fn _all_classes_to_attrs(all_classes: &[String]) -> Attrs {
    let mut attrs = Attrs::empty();
    if !all_classes.is_empty() {
        attrs.add_multiple(
            At::Class,
            &all_classes.iter().map(String::as_str).collect::<Vec<_>>(),
        );
    }
    attrs
}

/// `IF!(predicate => expression) -> Option<expression value>`
/// - `expression` is evaluated only when `predicate` is `true` (lazy eval).
/// - Alternative to `bool::then`.
///
/// # Example
///
/// ```rust,no_run
///div![
///    C!["btn", IF!(active => "active")],
///    "Button",
///    IF!(not(disabled) => ev(Ev::Click, Msg::Clicked)),
///]
/// ```
#[macro_export]
macro_rules! IF {
    ( $predicate:expr => $value:expr ) => {{
        // @TODO replace with `bool::then` once stable.
        if $predicate {
            Some($value)
        } else {
            None
        }
    }};
}

/// Convenience macro, for brevity.
#[macro_export]
macro_rules! id {
    { $id:expr } => {
        {
            $crate::virtual_dom::Attrs::from_id($id)
        }
     };
}

// reference for workaround used by style! macro
// (https://github.com/dtolnay/case-studies/blob/master/autoref-specialization/README.md)
/// Provide a shortcut for creating styles.
#[macro_export]
macro_rules! style {
    { $($key:expr => $value:expr $(;)?$(,)?)* } => {
        {
            #[allow(unused_imports)]
            use $crate::virtual_dom::values::{
                ToCSSValueForCSSValue, ToCSSValueForOptionToString, ToCSSValueForToString
            };
            let mut vals = IndexMap::new();
            $(
                vals.insert($key.into(), ($value).to_css_value());
            )*
            $crate::virtual_dom::Style::new(vals)
        }
     };
}

#[macro_export]
/// Converts items to `Vec<Node<Ms>` and returns flattened `Vec<Node<Ms>`.
///
/// Items have to implement the trait `IntoNodes`.
///
/// # Example
///
/// ```rust,no_run
/// nodes![
///     md!["# Hello"],
///     h2!["world"],
///     vec![
///         div!["Do you like"],
///         div!["Seed?"]
///     ],
/// ]
/// ```
macro_rules! nodes {
    (  $($element:expr $(,)?)* ) => {
        {
            use $crate::virtual_dom::IntoNodes;
            let mut nodes = Vec::new();
            $(
                nodes.append(&mut ($element).into_nodes());
            )*
            nodes
        }
    };
}

#[cfg(use_nightly)]
pub const fn wrap_debug<T>(object: T) -> dbg::WrapDebug<T> {
    dbg::WrapDebug(object)
}

#[cfg(not(use_nightly))]
pub fn wrap_debug<T: std::fmt::Debug>(object: T) -> T {
    object
}

/// A convenience function for logging to the web browser's console.  We use
/// a macro to supplement the log function to allow multiple inputs.
///
/// NOTE: `log!` also accepts entities which don't implement `Debug` on `nightly` Rust.
/// It's useful because you don't have to add `Debug` bound to many places - implementation for
/// logged entity is enough.
#[macro_export]
macro_rules! log {
    { $($expr:expr),* $(,)? } => {
        {
            let mut formatted_exprs = Vec::new();
            $(
                formatted_exprs.push(format!("{:#?}", $crate::shortcuts::wrap_debug(&$expr)));
            )*
            $crate::shortcuts::log_1(
                &formatted_exprs
                    .as_slice()
                    .join(" ")
                    .into()
            );
        }
     };
}
// wrapper for `log_1` because we don't want to "leak" `web_sys` dependency through macro
pub fn log_1(data_1: &JsValue) {
    web_sys::console::log_1(data_1);
}

/// Similar to log!
#[macro_export]
macro_rules! error {
    { $($expr:expr),* $(,)? } => {
        {
            let mut formatted_exprs = Vec::new();
            $(
                formatted_exprs.push(format!("{:#?}", $crate::shortcuts::wrap_debug(&$expr)));
            )*
            $crate::shortcuts::error_1(
                &formatted_exprs
                    .as_slice()
                    .join(" ")
                    .into()
            );
        }
     };
}
// wrapper for `error_1` because we don't want to "leak" `web_sys` dependency through macro
pub fn error_1(data_1: &JsValue) {
    web_sys::console::error_1(data_1);
}

/// A key-value pairs, where the keys and values must implement `ToString`.
#[macro_export]
macro_rules! key_value_pairs {
    { $($key:expr => $value:expr),* $(,)? } => {
        {
            let mut result = IndexMap::new();
            $(
                // We can handle arguments of multiple types by using this:
                // Strings, &strs, bools, numbers etc.
                result.insert($key.to_string(), $value.to_string());
            )*
            result
        }
     };
}
