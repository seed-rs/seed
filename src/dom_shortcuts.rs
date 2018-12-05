//! This file exports helper macros for element creation, populated by a higher-level macro,
//! and macros for creating the parts of elements. (attrs, style, events)

/// Create macros exposed to the package that allow shortcuts for Dom elements.
/// In the matching mattern below, we match the name we want to use with the name under
/// the Tag enum. Eg the div! macro uses Tag::Div.
///
///


//macro_rules! element {
//    // Create shortcut macros for any element; populate these functions in this module.
//    ($($tag:ident => $tag_camel:ident);+) => {
//        $(
//            #[macro_export]
//            macro_rules! $tag {
//                ( $($part:expr),* ) => {
//                    {
//                        let mut el = El::empty(Tag::$tag_camel);
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
//    ( $($part:expr),* ) => {
//        {
//            let mut el = El::empty(Tag::Div);
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

macro_rules! address {
    ( $($part:expr),* ) => {
        {
            let mut el = El::empty(Tag::Address);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}

macro_rules! article {
    ( $($part:expr),* ) => {
        {
            let mut el = El::empty(Tag::Article);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}

macro_rules! aside {
    ( $($part:expr),* ) => {
        {
            let mut el = El::empty(Tag::Aside);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}

macro_rules! footer {
    ( $($part:expr),* ) => {
        {
            let mut el = El::empty(Tag::Footer);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}

macro_rules! header {
    ( $($part:expr),* ) => {
        {
            let mut el = El::empty(Tag::Header);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}

macro_rules! h1 {
    ( $($part:expr),* ) => {
        {
            let mut el = El::empty(Tag::H1);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}

macro_rules! h2 {
    ( $($part:expr),* ) => {
        {
            let mut el = El::empty(Tag::H2);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}

macro_rules! h3 {
    ( $($part:expr),* ) => {
        {
            let mut el = El::empty(Tag::H3);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}

macro_rules! h4 {
    ( $($part:expr),* ) => {
        {
            let mut el = El::empty(Tag::H4);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}

macro_rules! h5 {
    ( $($part:expr),* ) => {
        {
            let mut el = El::empty(Tag::H5);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}

macro_rules! h6 {
    ( $($part:expr),* ) => {
        {
            let mut el = El::empty(Tag::H6);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}

macro_rules! hgroup {
    ( $($part:expr),* ) => {
        {
            let mut el = El::empty(Tag::Hgroup);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}

macro_rules! main {
    ( $($part:expr),* ) => {
        {
            let mut el = El::empty(Tag::Main);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}

macro_rules! nav {
    ( $($part:expr),* ) => {
        {
            let mut el = El::empty(Tag::Nav);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}

macro_rules! section {
    ( $($part:expr),* ) => {
        {
            let mut el = El::empty(Tag::Section);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}

macro_rules! blockquote {
    ( $($part:expr),* ) => {
        {
            let mut el = El::empty(Tag::Blockquote);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}

macro_rules! dd {
    ( $($part:expr),* ) => {
        {
            let mut el = El::empty(Tag::Dd);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}

macro_rules! dir {
    ( $($part:expr),* ) => {
        {
            let mut el = El::empty(Tag::Dir);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}

macro_rules! div {
    ( $($part:expr),* ) => {
        {
            let mut el = El::empty(Tag::Div);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}

macro_rules! p {
    ( $($part:expr),* ) => {
        {
            let mut el = El::empty(Tag::P);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}

macro_rules! button {
    ( $($part:expr),* ) => {
        {
            let mut el = El::empty(Tag::Button);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}

macro_rules! fieldset {
    ( $($part:expr),* ) => {
        {
            let mut el = El::empty(Tag::FieldSet);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}

macro_rules! form {
    ( $($part:expr),* ) => {
        {
            let mut el = El::empty(Tag::Form);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}

macro_rules! input {
    ( $($part:expr),* ) => {
        {
            let mut el = El::empty(Tag::Input);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}

macro_rules! label {
    ( $($part:expr),* ) => {
        {
            let mut el = El::empty(Tag::Label);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}

macro_rules! legend {
    ( $($part:expr),* ) => {
        {
            let mut el = El::empty(Tag::Legend);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}

macro_rules! meter {
    ( $($part:expr),* ) => {
        {
            let mut el = El::empty(Tag::Meter);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}

macro_rules! optgroup {
    ( $($part:expr),* ) => {
        {
            let mut el = El::empty(Tag::OptGroup);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}


macro_rules! option {
    ( $($part:expr),* ) => {
        {
            let mut el = El::empty(Tag::Option);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}

macro_rules! output {
    ( $($part:expr),* ) => {
        {
            let mut el = El::empty(Tag::Output);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}

macro_rules! progress {
    ( $($part:expr),* ) => {
        {
            let mut el = El::empty(Tag::Progress);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}

macro_rules! select {
    ( $($part:expr),* ) => {
        {
            let mut el = El::empty(Tag::Select);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}

macro_rules! textarea {
    ( $($part:expr),* ) => {
        {
            let mut el = El::empty(Tag::TextArea);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}


macro_rules! a {
    ( $($part:expr),* ) => {
        {
            let mut el = El::empty(Tag::A);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}

macro_rules! abbr {
    ( $($part:expr),* ) => {
        {
            let mut el = El::empty(Tag::Abbr);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}

macro_rules! b {
    ( $($part:expr),* ) => {
        {
            let mut el = El::empty(Tag::B);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}

macro_rules! bdi {
    ( $($part:expr),* ) => {
        {
            let mut el = El::empty(Tag::Bdi);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}

macro_rules! bdo {
    ( $($part:expr),* ) => {
        {
            let mut el = El::empty(Tag::Bdo);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}

macro_rules! br {
    ( $($part:expr),* ) => {
        {
            let mut el = El::empty(Tag::Br);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}

macro_rules! cite {
    ( $($part:expr),* ) => {
        {
            let mut el = El::empty(Tag::Cite);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}

macro_rules! code {
    ( $($part:expr),* ) => {
        {
            let mut el = El::empty(Tag::Code);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}

macro_rules! data {
    ( $($part:expr),* ) => {
        {
            let mut el = El::empty(Tag::Data);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}

macro_rules! dfn {
    ( $($part:expr),* ) => {
        {
            let mut el = El::empty(Tag::Dfn);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}

macro_rules! em {
    ( $($part:expr),* ) => {
        {
            let mut el = El::empty(Tag::Em);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}

macro_rules! i {
    ( $($part:expr),* ) => {
        {
            let mut el = El::empty(Tag::I);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}

macro_rules! kbd {
    ( $($part:expr),* ) => {
        {
            let mut el = El::empty(Tag::Kbd);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}

macro_rules! mark {
    ( $($part:expr),* ) => {
        {
            let mut el = El::empty(Tag::Mark);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}

macro_rules! span {
    ( $($part:expr),* ) => {
        {
            let mut el = El::empty(Tag::Span);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}

macro_rules! strong {
    ( $($part:expr),* ) => {
        {
            let mut el = El::empty(Tag::Strong);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}

macro_rules! sub {
    ( $($part:expr),* ) => {
        {
            let mut el = El::empty(Tag::Sub);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}

macro_rules! sup {
    ( $($part:expr),* ) => {
        {
            let mut el = El::empty(Tag::Sup);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}

macro_rules! img {
    ( $($part:expr),* ) => {
        {
            let mut el = El::empty(Tag::Img);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}

macro_rules! canvas {
    ( $($part:expr),* ) => {
        {
            let mut el = El::empty(Tag::Canvas);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}

macro_rules! noscript {
    ( $($part:expr),* ) => {
        {
            let mut el = El::empty(Tag::NoScript);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}

macro_rules! script {
    ( $($part:expr),* ) => {
        {
            let mut el = El::empty(Tag::Script);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}

macro_rules! del {
    ( $($part:expr),* ) => {
        {
            let mut el = El::empty(Tag::Del);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}

macro_rules! ins {
    ( $($part:expr),* ) => {
        {
            let mut el = El::empty(Tag::Ins);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}

macro_rules! map {
    ( $($part:expr),* ) => {
        {
            let mut el = El::empty(Tag::Map);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}

macro_rules! track {
    ( $($part:expr),* ) => {
        {
            let mut el = El::empty(Tag::Track);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}

macro_rules! video {
    ( $($part:expr),* ) => {
        {
            let mut el = El::empty(Tag::Video);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}

macro_rules! applet {
    ( $($part:expr),* ) => {
        {
            let mut el = El::empty(Tag::Applet);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}

macro_rules! embed {
    ( $($part:expr),* ) => {
        {
            let mut el = El::empty(Tag::Embed);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}

macro_rules! iframe {
    ( $($part:expr),* ) => {
        {
            let mut el = El::empty(Tag::Iframe);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}


/// Provide a shortcut for creating styles.
//#[macro_export]
macro_rules! style {
    { $($key:expr => $value:expr);* } => {
        {
            let mut vals = std::collections::HashMap::new();
            $(
                vals.insert($key, $value);
            )*
            Style::new(vals)
        }
     };
}

/// Provide a shortcut for creating attributes.
// todo DRY between thsi and style
//#[macro_export]
macro_rules! attrs {
    { $($key:expr => $value:expr);* } => {
        {
            let mut vals = std::collections::HashMap::new();
            $(
                vals.insert($key, $value);
            )*
            Attrs::new(vals)
        }
     };
}

//////// todo DRY between thsi and style
//#[macro_export]
macro_rules! events {
    { $($event_str:expr => $handler:expr);+ } => {
        {
            let mut vals = Vec::new();
            $(
//                vals.push(($event_str.into(), Box::new($handler)));
                vals.push(($event_str.into(), $handler));
            )+
            Events::new(vals)
        }
     };
}

