//! This file exports helper macros for element creation, populated by a higher-level macro,
//! and macros for creating the parts of elements. (attrs, style, events)

use wasm_bindgen::JsValue;

/// Copied from [https://github.com/rust-lang/rust/issues/35853](https://github.com/rust-lang/rust/issues/35853)
macro_rules! with_dollar_sign {
    ($($body:tt)*) => {
        macro_rules! __with_dollar_sign { $($body)* }
        __with_dollar_sign!($);
    }
}

/// Create macros exposed to the package that allow shortcuts for Dom elements.
/// In the matching pattern below, we match the name we want to use with the name under
/// the `seed::dom_types::Tag` enum. Eg the div! macro uses `seed::dom_types::Tag::Div`.
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
                                let mut el = El::empty($crate::dom_types::Tag::$Tag_camel);
                                $d ( $d part.update(&mut el); )*
                                $crate::dom_types::Node::Element(el)
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
                                let mut el = El::empty_svg($crate::dom_types::Tag::$Tag_camel);
                                $d ( $d part.update(&mut el); )*
                                $crate::dom_types::Node::Element(el)
                            }
                        };
                    }
                )+
            }
        }
   }
}

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
    linear_gradient => LinearGradient; radial_gradient => RadialGradient; mesh_gradient => MeshGradient;
    stop => Stop;
    // SVG gradphics elements
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
        $crate::dom_types::Node::Empty
    };
}

#[macro_export]
macro_rules! raw {
    ($raw_html:expr) => {
        El::from_html($raw_html)
    };
}

#[macro_export]
macro_rules! md {
    ($md:expr) => {
        El::from_markdown($md)
    };
}

#[macro_export]
macro_rules! plain {
    ($text:expr) => {
        $crate::dom_types::Node::new_text($text)
    };
}

#[macro_export]
macro_rules! custom {
    ( $($part:expr),* $(,)? ) => {
        {
            let mut el = El::empty($crate::dom_types::Tag::Custom("missingtagname".into()));
            $ ( $part.update(&mut el); )*
            $crate::dom_types::Node::Element(el)
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
            $crate::dom_types::Attrs::new(vals)
        }
     };
}

/// Convenience macro. Ideal when there are multiple classes, and no other attrs.
#[macro_export]
macro_rules! class {
    { $($class:expr $(=> $predicate:expr)? $(,)?)* } => {
        {
            let mut result = $crate::dom_types::Attrs::empty();
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

/// Convenience macro, for brevity.
#[macro_export]
macro_rules! id {
    { $id:expr } => {
        {
            $crate::dom_types::Attrs::from_id($id)
        }
     };
}

/// Provide a shortcut for creating styles.
#[macro_export]
macro_rules! style {
    { $($key:expr => $value:expr $(;)?$(,)?)* } => {
        {
            let mut vals = IndexMap::new();
            $(
                // We can handle arguments of multiple types by using this:
                // Strings, &strs, bools, numbers etc.
                // And cases like `CSSValue::Ignored`.
                vals.insert($key.into(), (&$value).into());
            )*
            $crate::dom_types::Style::new(vals)
        }
     };
}

//#[macro_export]
//macro_rules! events {
//    { $($event_str:expr => $handler:expr);+ } => {
//        {
//            let mut result = Vec::new();
//            $(
//                match $event_str {
////                    _ => result.push($crate::dom_types::Listener::new_input($event_str.into(), $handler)),
//
//
//                    _ => result.push($crate::dom_types::Listener::new_input($event_str.into(), $handler)),
////
////
////
////  "input" => result.push($crate::dom_types::Listener::new_input($event_str.into(), $handler)),
////                    _ => result.push($crate::dom_types::Listener::new($event_str.into(), $handler)),
//
//                }
//
////                result.push($crate::dom_types::Listener::new($event_str.into(), $handler));
//            )+
//            result
//        }
//     };
//}
//

///// Attempt to apply the correct input type based on its trigger.
//#[macro_export]
//macro_rules! ev2 {
//    { $event_str:expr, $handler:expr } => {
//        {
//            match event_str {
//                "input" => $crate::dom_types::input_ev($event_str, $handler),
////                "change" => $crate::dom_types::input_ev($event_str, $handler),
////
////                "keydown" => $crate::dom_types::keyboard_ev($event_str, $handler),
////
////                "click" => $crate::dom_types::ev($event_str, $handler),
////                "auxclick" => $crate::dom_types::ev($event_str, $handler),
////                "dblclick" => $crate::dom_types::ev($event_str, $handler),
////                "contextmenu" => $crate::dom_types::input_ev($event_str, $handler),
//
//                _ => $crate::dom_types::raw_ev($event_str, $handler),
//
//            }
//        }
//    };
//}

///// Attempt to apply the correct input type based on its trigger.
//#[macro_export]
//macro_rules! ev2 {
//    { input, $handler:expr } => {
//        {
//            $crate::dom_types::input_ev("input", $handler),
//        }
//    };
//    { click, $handler:expr } => {
//        {
//            $crate::dom_types::simple_ev("click", $handler),
//        }
//    };
//
//
//
//}

/// A convenience function for logging to the web browser's console.  We use
/// a macro to supplement the log function to allow multiple inputs.
#[macro_export]
macro_rules! log {
    { $($expr:expr),* $(,)? } => {
        {
            let mut formatted_exprs = Vec::new();
            $(
                formatted_exprs.push(format!("{:#?}", $expr));
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
                formatted_exprs.push(format!("{:#?}", $expr));
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
