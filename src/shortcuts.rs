//! This file exports helper macros for element creation, populated by a higher-level macro,
//! and macros for creating the parts of elements. (attrs, style, events)

/// Copied from https://github.com/rust-lang/rust/issues/35853.
macro_rules! with_dollar_sign {
    ($($body:tt)*) => {
        macro_rules! __with_dollar_sign { $($body)* }
        __with_dollar_sign!($);
    }
}

/// Create macros exposed to the package that allow shortcuts for Dom elements.
/// In the matching mattern below, we match the name we want to use with the name under
/// the seed::dom_types::Tag enum. Eg the div! macro uses seed::dom_types::Tag::Div.
macro_rules! element {
    // Create shortcut macros for any element; populate these functions in this module.
    ($($Tag:ident => $Tag_camel:ident);+) => {
        // This replaces $d with $ in the inner macro.
        with_dollar_sign! {
            ($d:tt) => {
                $(
                    #[macro_export]
                    macro_rules! $Tag {
                        ( $d($d part:expr),* $d(,)* ) => {
                            {
                                let mut el = El::empty(seed::dom_types::Tag::$Tag_camel);
                                $d ( $d part.update(&mut el); )*
                                el
                            }
                        };
                    }
                )+
            }
        }
   }
}

/// El must be exposed in the module where this is called for these to work.
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

#[macro_export]
macro_rules! custom {
    ( $($part:expr),* $(,)* ) => {
        {
            let mut el = El::empty(seed::dom_types::Tag::Custom("missingtagname".into()));
            $ ( $part.update(&mut el); )*
            el
        }
    };
}

// --- SVG shape elements ---

#[macro_export]
macro_rules! line_ {
    ( $($part:expr),* $(,)* ) => {
        {
            let mut el = El::empty_svg(seed::dom_types::Tag::Line);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}

#[macro_export]
macro_rules! rect {
    ( $($part:expr),* $(,)* ) => {
        {
            let mut el = El::empty_svg(seed::dom_types::Tag::Rect);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}

#[macro_export]
macro_rules! circle {
    ( $($part:expr),* $(,)* ) => {
        {
            let mut el = El::empty_svg(seed::dom_types::Tag::Circle);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}

#[macro_export]
macro_rules! ellipse {
    ( $($part:expr),* $(,)* ) => {
        {
            let mut el = El::empty_svg(seed::dom_types::Tag::Ellipse);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}

#[macro_export]
macro_rules! polygon {
    ( $($part:expr),* $(,)* ) => {
        {
            let mut el = El::empty_svg(seed::dom_types::Tag::Polygon);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}

#[macro_export]
macro_rules! polyline {
    ( $($part:expr),* $(,)* ) => {
        {
            let mut el = El::empty_svg(seed::dom_types::Tag::Polyline);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}

#[macro_export]
macro_rules! mesh {
    ( $($part:expr),* $(,)* ) => {
        {
            let mut el = El::empty_svg(seed::dom_types::Tag::Mesh);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}

#[macro_export]
macro_rules! path {
    ( $($part:expr),* $(,)* ) => {
        {
            let mut el = El::empty_svg(seed::dom_types::Tag::Path);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}

// --- SVG container elements --- //

// TODO:
// Create the following macros
// - missing-glyph
// - pattern
// - switch
// - symbol
// - unknown

#[macro_export]
macro_rules! svg {
    ( $($part:expr),* $(,)* ) => {
        {
            let mut el = El::empty_svg(seed::dom_types::Tag::Svg);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}

#[macro_export]
macro_rules! g {
    ( $($part:expr),* $(,)* ) => {
        {
            let mut el = El::empty_svg(seed::dom_types::Tag::G);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}

#[macro_export]
macro_rules! defs {
    ( $($part:expr),* $(,)* ) => {
        {
            let mut el = El::empty_svg(seed::dom_types::Tag::Defs);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}

#[macro_export]
macro_rules! marker {
    ( $($part:expr),* $(,)* ) => {
        {
            let mut el = El::empty_svg(seed::dom_types::Tag::Marker);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}

#[macro_export]
macro_rules! mask {
    ( $($part:expr),* $(,)* ) => {
        {
            let mut el = El::empty_svg(seed::dom_types::Tag::Mask);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}

// TODO:
// - add SVG animation elements
// - add SVG filter primitive elements
// - add SVG descriptive elements
// - add SVG font elements
// - add SVG paint server elements

// --- SVG gradient elements --- //

#[macro_export]
macro_rules! linear_gradient {
    ( $($part:expr),* $(,)* ) => {
        {
            let mut el = El::empty_svg(seed::dom_types::Tag::LinearGradient);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}

#[macro_export]
macro_rules! radial_gradient {
    ( $($part:expr),* $(,)* ) => {
        {
            let mut el = El::empty_svg(seed::dom_types::Tag::RadialGradient);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}

#[macro_export]
macro_rules! mesh_gradient {
    ( $($part:expr),* $(,)* ) => {
        {
            let mut el = El::empty_svg(seed::dom_types::Tag::MeshGradient);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}

#[macro_export]
macro_rules! stop {
    ( $($part:expr),* $(,)* ) => {
        {
            let mut el = El::empty_svg(seed::dom_types::Tag::Stop);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}

// --- SVG graphics elements --- //

#[macro_export]
macro_rules! image {
    ( $($part:expr),* $(,)* ) => {
        {
            let mut el = El::empty_svg(seed::dom_types::Tag::Image);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}

// --- SVG graphics referencing elements --- /

#[macro_export]
macro_rules! r#use {
    ( $($part:expr),* $(,)* ) => {
        {
            let mut el = El::empty_svg(seed::dom_types::Tag::Use);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}

// --- SVG text content elements --- //

// TODO:
// - altGlyph
// - altGlyphDef
// - altGlyphItem
// - glyph
// - glyphRef
// - textPath

#[macro_export]
macro_rules! text {
    ( $($part:expr),* $(,)* ) => {
        {
            let mut el = El::empty_svg(seed::dom_types::Tag::Text);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}

#[macro_export]
macro_rules! tref {
    ( $($part:expr),* $(,)* ) => {
        {
            let mut el = El::empty_svg(seed::dom_types::Tag::TRef);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}

#[macro_export]
macro_rules! tspan {
    ( $($part:expr),* $(,)* ) => {
        {
            let mut el = El::empty_svg(seed::dom_types::Tag::TSpan);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}

// TODO:
// Add the following SVG element macros:
//  - clipPath
//  - color-profile
//  - cursor
//  - filter
//  - foreignObject
//  - hatchpath
//  - meshpatch
//  - meshrow
//  - style
//  - view

//// End element-creation macros.

/// Provide a shortcut for creating attributes.
#[macro_export]
macro_rules! attrs {
    { $($key:expr => $value:expr);* $(;)* } => {
        {
//            let mut result = Attrs::empty();
            let mut vals = std::collections::HashMap::new();
            $(
                // We can handle arguments of multiple types by using this:
                // Strings, &strs, bools, numbers etc.
                vals.insert($key.into(), $value.to_string());
//                vals.insert(String::from($key), $value.to_string());
//                $key.update(&mut result);
            )*
            seed::dom_types::Attrs::new(vals)
//            result
        }
     };
}

/// Convenience macro. Ideal when there are multiple classes, and no other attrs.
#[macro_export]
macro_rules! class {
    { $($class:expr),* $(,)* } => {
        {
            let mut result = seed::dom_types::Attrs::empty();
            let mut classes = Vec::new();
            $(
                classes.push($class);
            )*
            result.add_multiple("class".into(), classes);
            result
        }
     };
}

/// Convenience macro, for brevity.
#[macro_export]
macro_rules! id {
    { $id:expr } => {
        {
            seed::dom_types::Attrs::from_id($id)
        }
     };
}

// todo: Once the macro_at_most_once_rep is in stable, you can use $(;)? here (
// todo: and in el creation macros) to make only trailing comma/semicolon acceptable.
/// Provide a shortcut for creating styles.
#[macro_export]
macro_rules! style {
    { $($key:expr => $value:expr);* $(;)* } => {
        {
            let mut vals = std::collections::HashMap::new();
            $(
                // We can handle arguments of multiple types by using this:
                // Strings, &strs, bools, numbers etc.
                vals.insert(String::from($key), $value.to_string());
            )*
            seed::dom_types::Style::new(vals)
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
////                    _ => result.push(seed::dom_types::Listener::new_input($event_str.into(), $handler)),
//
//
//                    _ => result.push(seed::dom_types::Listener::new_input($event_str.into(), $handler)),
////
////
////
////  "input" => result.push(seed::dom_types::Listener::new_input($event_str.into(), $handler)),
////                    _ => result.push(seed::dom_types::Listener::new($event_str.into(), $handler)),
//
//                }
//
////                result.push(seed::dom_types::Listener::new($event_str.into(), $handler));
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
//                "input" => seed::dom_types::input_ev($event_str, $handler),
////                "change" => seed::dom_types::input_ev($event_str, $handler),
////
////                "keydown" => seed::dom_types::keyboard_ev($event_str, $handler),
////
////                "click" => seed::dom_types::ev($event_str, $handler),
////                "auxclick" => seed::dom_types::ev($event_str, $handler),
////                "dblclick" => seed::dom_types::ev($event_str, $handler),
////                "contextmenu" => seed::dom_types::input_ev($event_str, $handler),
//
//                _ => seed::dom_types::raw_ev($event_str, $handler),
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
//            seed::dom_types::input_ev("input", $handler),
//        }
//    };
//    { click, $handler:expr } => {
//        {
//            seed::dom_types::simple_ev("click", $handler),
//        }
//    };
//
//
//
//}

/// A convenience function for logging to the web browser's console.  We use
/// a macro instead of a function to allow flexible input types, and multiple
/// inputs.
#[macro_export]
macro_rules! log {
    { $($expr:expr),* $(,)* } => {
        {
            let mut text = String::new();
            $(
                text += &$expr.to_string();
                text += " ";
            )*
            web_sys::console::log_1(&text.into());
        }
     };
}

/// A HashMap literal, where the keys and valsmust implement ToString.
#[macro_export]
macro_rules! hashmap_string {
    { $($key:expr => $value:expr),* $(,)* } => {
        {
            let mut result = std::collections::HashMap::new();
            $(
                // We can handle arguments of multiple types by using this:
                // Strings, &strs, bools, numbers etc.
                result.insert($key.to_string(), $value.to_string());
            )*
            result
        }
     };
}
