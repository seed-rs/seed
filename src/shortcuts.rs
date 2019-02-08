//! This file exports helper macros for element creation, populated by a higher-level macro,
//! and macros for creating the parts of elements. (attrs, style, events)

/// Create macros exposed to the package that allow shortcuts for Dom elements.
/// In the matching mattern below, we match the name we want to use with the name under
/// the seed::dom_types::Tag enum. Eg the div! macro uses seed::dom_types::Tag::Div.
///
///

//macro_rules! element {
//    // Create shortcut macros for any element; populate these functions in this module.
//    ($($seed::dom_types::Tag:ident => $seed::dom_types::Tag_camel:ident);+) => {
//        $(
//            #[macro_export]
//            macro_rules! $seed::dom_types::Tag {
//                ( $($part:expr),* $(,)* ) => {
//                    {
//                        let mut el = El::empty(seed::dom_types::Tag::$seed::dom_types::Tag_camel);
//                        $ (
//                             $part.update(&mut el);
//                        )*
//                        el
//                    }
//                };
//            }
//        )+
//    }
//}

//
//
///// El must be exposed in the module where this is called for these to work.
//element! {
//    div => Div; span => Span;
//    h1 => H1; h2 => H2; h3 => H3; h4 => H4; h5 => H5; h6 => H6; p => P;
//    button => Button; img => Img
//}

// todo: Currently rust doesn't allow nested macros to repeat in binding patterns
// https://github.com/rust-lang/rust/issues/35853

// todo so we can't iterate both through macros to make, and arguments in the macro.
// todo for now, use this workaround by populating a whole bunch of macros manually.

//
//macro_rules! div {
//    ( $($part:expr),* $(,)* ) => {
//        {
//            let mut el = El::empty(seed::dom_types::Tag::Div);
//            let mut arg_count = 0;
//            $ ( $part.update(&mut el); arg_count += 1;)*
//            if arg_count > 5{
//            // todo there are ways to sort this out, I think. Have update methods
//            // todo accept/return a value; have them return an err type etc.
//                crate::log(&"Element-creation macros can take no more than 5 arguments.");
//            }
//            el
//        }
//    };
//}

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

#[macro_export]
macro_rules! address {
    ( $($part:expr),* $(,)* ) => {
        {
            let mut el = El::empty(seed::dom_types::Tag::Address);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}

#[macro_export]
macro_rules! article {
    ( $($part:expr),* $(,)* ) => {
        {
            let mut el = El::empty(seed::dom_types::Tag::Article);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}

#[macro_export]
macro_rules! aside {
    ( $($part:expr),* $(,)* ) => {
        {
            let mut el = El::empty(seed::dom_types::Tag::Aside);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}

#[macro_export]
macro_rules! footer {
    ( $($part:expr),* $(,)* ) => {
        {
            let mut el = El::empty(seed::dom_types::Tag::Footer);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}

#[macro_export]
macro_rules! header {
    ( $($part:expr),* $(,)* ) => {
        {
            let mut el = El::empty(seed::dom_types::Tag::Header);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}

#[macro_export]
macro_rules! h1 {
    ( $($part:expr),* $(,)* ) => {
        {
            let mut el = El::empty(seed::dom_types::Tag::H1);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}

#[macro_export]
macro_rules! h2 {
    ( $($part:expr),* $(,)* ) => {
        {
            let mut el = El::empty(seed::dom_types::Tag::H2);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}

#[macro_export]
macro_rules! h3 {
    ( $($part:expr),* $(,)* ) => {
        {
            let mut el = El::empty(seed::dom_types::Tag::H3);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}

#[macro_export]
macro_rules! h4 {
    ( $($part:expr),* $(,)* ) => {
        {
            let mut el = El::empty(seed::dom_types::Tag::H4);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}

#[macro_export]
macro_rules! h5 {
    ( $($part:expr),* $(,)* ) => {
        {
            let mut el = El::empty(seed::dom_types::Tag::H5);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}

#[macro_export]
macro_rules! h6 {
    ( $($part:expr),* $(,)* ) => {
        {
            let mut el = El::empty(seed::dom_types::Tag::H6);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}

#[macro_export]
macro_rules! hgroup {
    ( $($part:expr),* $(,)* ) => {
        {
            let mut el = El::empty(seed::dom_types::Tag::Hgroup);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}

#[macro_export]
macro_rules! main {
    ( $($part:expr),* $(,)* ) => {
        {
            let mut el = El::empty(seed::dom_types::Tag::Main);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}

#[macro_export]
macro_rules! nav {
    ( $($part:expr),* $(,)* ) => {
        {
            let mut el = El::empty(seed::dom_types::Tag::Nav);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}

#[macro_export]
macro_rules! section {
    ( $($part:expr),* $(,)* ) => {
        {
            let mut el = El::empty(seed::dom_types::Tag::Section);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}

#[macro_export]
macro_rules! blockquote {
    ( $($part:expr),* $(,)* ) => {
        {
            let mut el = El::empty(seed::dom_types::Tag::Blockquote);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}

#[macro_export]
macro_rules! dd {
    ( $($part:expr),* $(,)* ) => {
        {
            let mut el = El::empty(seed::dom_types::Tag::Dd);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}

#[macro_export]
macro_rules! dir {
    ( $($part:expr),* $(,)* ) => {
        {
            let mut el = El::empty(seed::dom_types::Tag::Dir);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}

#[macro_export]
macro_rules! div {
    ( $($part:expr),* $(,)* ) => {
        {
            let mut el = El::empty(seed::dom_types::Tag::Div);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}

#[macro_export]
macro_rules! dl {
    ( $($part:expr),* $(,)* ) => {
        {
            let mut el = El::empty(seed::dom_types::Tag::Dl);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}

#[macro_export]
macro_rules! dt {
    ( $($part:expr),* $(,)* ) => {
        {
            let mut el = El::empty(seed::dom_types::Tag::Dt);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}

#[macro_export]
macro_rules! figure {
    ( $($part:expr),* $(,)* ) => {
        {
            let mut el = El::empty(seed::dom_types::Tag::Figure);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}

#[macro_export]
macro_rules! figcaption {
    ( $($part:expr),* $(,)* ) => {
        {
            let mut el = El::empty(seed::dom_types::Tag::FigCaption);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}

#[macro_export]
macro_rules! hr {
    ( $($part:expr),* $(,)* ) => {
        {
            let mut el = El::empty(seed::dom_types::Tag::Hr);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}

#[macro_export]
macro_rules! li {
    ( $($part:expr),* $(,)* ) => {
        {
            let mut el = El::empty(seed::dom_types::Tag::Li);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}

#[macro_export]
macro_rules! ol {
    ( $($part:expr),* $(,)* ) => {
        {
            let mut el = El::empty(seed::dom_types::Tag::Ol);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}

#[macro_export]
macro_rules! p {
    ( $($part:expr),* $(,)* ) => {
        {
            let mut el = El::empty(seed::dom_types::Tag::P);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}

#[macro_export]
macro_rules! pre {
    ( $($part:expr),* $(,)* ) => {
        {
            let mut el = El::empty(seed::dom_types::Tag::Pre);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}

#[macro_export]
macro_rules! ul {
    ( $($part:expr),* $(,)* ) => {
        {
            let mut el = El::empty(seed::dom_types::Tag::Ul);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}

#[macro_export]
macro_rules! a {
    ( $($part:expr),* $(,)* ) => {
        {
            let mut el = El::empty(seed::dom_types::Tag::A);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}

#[macro_export]
macro_rules! abbr {
    ( $($part:expr),* $(,)* ) => {
        {
            let mut el = El::empty(seed::dom_types::Tag::Abbr);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}

#[macro_export]
macro_rules! b {
    ( $($part:expr),* $(,)* ) => {
        {
            let mut el = El::empty(seed::dom_types::Tag::B);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}

#[macro_export]
macro_rules! bdi {
    ( $($part:expr),* $(,)* ) => {
        {
            let mut el = El::empty(seed::dom_types::Tag::Bdi);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}

#[macro_export]
macro_rules! bdo {
    ( $($part:expr),* $(,)* ) => {
        {
            let mut el = El::empty(seed::dom_types::Tag::Bdo);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}

#[macro_export]
macro_rules! br {
    ( $($part:expr),* $(,)* ) => {
        {
            let mut el = El::empty(seed::dom_types::Tag::Br);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}

#[macro_export]
macro_rules! cite {
    ( $($part:expr),* $(,)* ) => {
        {
            let mut el = El::empty(seed::dom_types::Tag::Cite);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}

#[macro_export]
macro_rules! code {
    ( $($part:expr),* $(,)* ) => {
        {
            let mut el = El::empty(seed::dom_types::Tag::Code);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}

#[macro_export]
macro_rules! data {
    ( $($part:expr),* $(,)* ) => {
        {
            let mut el = El::empty(seed::dom_types::Tag::Data);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}

#[macro_export]
macro_rules! dfn {
    ( $($part:expr),* $(,)* ) => {
        {
            let mut el = El::empty(seed::dom_types::Tag::Dfn);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}

#[macro_export]
macro_rules! em {
    ( $($part:expr),* $(,)* ) => {
        {
            let mut el = El::empty(seed::dom_types::Tag::Em);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}

#[macro_export]
macro_rules! i {
    ( $($part:expr),* $(,)* ) => {
        {
            let mut el = El::empty(seed::dom_types::Tag::I);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}

#[macro_export]
macro_rules! kbd {
    ( $($part:expr),* $(,)* ) => {
        {
            let mut el = El::empty(seed::dom_types::Tag::Kbd);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}

#[macro_export]
macro_rules! mark {
    ( $($part:expr),* $(,)* ) => {
        {
            let mut el = El::empty(seed::dom_types::Tag::Mark);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}

#[macro_export]
macro_rules! button {
    ( $($part:expr),* $(,)* ) => {
        {
            let mut el = El::empty(seed::dom_types::Tag::Button);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}

#[macro_export]
macro_rules! fieldset {
    ( $($part:expr),* $(,)* ) => {
        {
            let mut el = El::empty(seed::dom_types::Tag::FieldSet);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}

#[macro_export]
macro_rules! form {
    ( $($part:expr),* $(,)* ) => {
        {
            let mut el = El::empty(seed::dom_types::Tag::Form);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}

#[macro_export]
macro_rules! input {
    ( $($part:expr),* $(,)* ) => {
        {
            let mut el = El::empty(seed::dom_types::Tag::Input);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}

#[macro_export]
macro_rules! label {
    ( $($part:expr),* $(,)* ) => {
        {
            let mut el = El::empty(seed::dom_types::Tag::Label);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}

#[macro_export]
macro_rules! legend {
    ( $($part:expr),* $(,)* ) => {
        {
            let mut el = El::empty(seed::dom_types::Tag::Legend);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}

#[macro_export]
macro_rules! meter {
    ( $($part:expr),* $(,)* ) => {
        {
            let mut el = El::empty(seed::dom_types::Tag::Meter);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}

#[macro_export]
macro_rules! optgroup {
    ( $($part:expr),* $(,)* ) => {
        {
            let mut el = El::empty(seed::dom_types::Tag::OptGroup);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}

#[macro_export]
macro_rules! option {
    ( $($part:expr),* $(,)* ) => {
        {
            let mut el = El::empty(seed::dom_types::Tag::Option);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}

#[macro_export]
macro_rules! output {
    ( $($part:expr),* $(,)* ) => {
        {
            let mut el = El::empty(seed::dom_types::Tag::Output);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}

#[macro_export]
macro_rules! progress {
    ( $($part:expr),* $(,)* ) => {
        {
            let mut el = El::empty(seed::dom_types::Tag::Progress);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}

#[macro_export]
macro_rules! select {
    ( $($part:expr),* $(,)* ) => {
        {
            let mut el = El::empty(seed::dom_types::Tag::Select);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}

#[macro_export]
macro_rules! textarea {
    ( $($part:expr),* $(,)* ) => {
        {
            let mut el = El::empty(seed::dom_types::Tag::TextArea);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}

#[macro_export]
macro_rules! span {
    ( $($part:expr),* $(,)* ) => {
        {
            let mut el = El::empty(seed::dom_types::Tag::Span);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}

#[macro_export]
macro_rules! strong {
    ( $($part:expr),* $(,)* ) => {
        {
            let mut el = El::empty(seed::dom_types::Tag::Strong);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}

#[macro_export]
macro_rules! sub {
    ( $($part:expr),* $(,)* ) => {
        {
            let mut el = El::empty(seed::dom_types::Tag::Sub);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}

#[macro_export]
macro_rules! sup {
    ( $($part:expr),* $(,)* ) => {
        {
            let mut el = El::empty(seed::dom_types::Tag::Sup);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}

#[macro_export]
macro_rules! img {
    ( $($part:expr),* $(,)* ) => {
        {
            let mut el = El::empty(seed::dom_types::Tag::Img);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}

#[macro_export]
macro_rules! canvas {
    ( $($part:expr),* $(,)* ) => {
        {
            let mut el = El::empty(seed::dom_types::Tag::Canvas);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}

#[macro_export]
macro_rules! noscript {
    ( $($part:expr),* $(,)* ) => {
        {
            let mut el = El::empty(seed::dom_types::Tag::NoScript);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}

#[macro_export]
macro_rules! script {
    ( $($part:expr),* $(,)* ) => {
        {
            let mut el = El::empty(seed::dom_types::Tag::Script);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}

#[macro_export]
macro_rules! del {
    ( $($part:expr),* $(,)* ) => {
        {
            let mut el = El::empty(seed::dom_types::Tag::Del);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}

#[macro_export]
macro_rules! ins {
    ( $($part:expr),* $(,)* ) => {
        {
            let mut el = El::empty(seed::dom_types::Tag::Ins);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}

#[macro_export]
macro_rules! map {
    ( $($part:expr),* $(,)* ) => {
        {
            let mut el = El::empty(seed::dom_types::Tag::Map);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}

#[macro_export]
macro_rules! track {
    ( $($part:expr),* $(,)* ) => {
        {
            let mut el = El::empty(seed::dom_types::Tag::Track);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}

#[macro_export]
macro_rules! video {
    ( $($part:expr),* $(,)* ) => {
        {
            let mut el = El::empty(seed::dom_types::Tag::Video);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}

#[macro_export]
macro_rules! applet {
    ( $($part:expr),* $(,)* ) => {
        {
            let mut el = El::empty(seed::dom_types::Tag::Applet);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}

#[macro_export]
macro_rules! embed {
    ( $($part:expr),* $(,)* ) => {
        {
            let mut el = El::empty(seed::dom_types::Tag::Embed);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}

#[macro_export]
macro_rules! iframe {
    ( $($part:expr),* $(,)* ) => {
        {
            let mut el = El::empty(seed::dom_types::Tag::Iframe);
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
