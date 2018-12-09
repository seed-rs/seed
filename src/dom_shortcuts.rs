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
//                ( $($part:expr),* ) => {
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
//    ( $($part:expr),* ) => {
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
macro_rules! address {
    ( $($part:expr),* ) => {
        {
            let mut el = El::empty(seed::dom_types::Tag::Address);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}

#[macro_export]
macro_rules! article {
    ( $($part:expr),* ) => {
        {
            let mut el = El::empty(seed::dom_types::Tag::Article);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}

#[macro_export]
macro_rules! aside {
    ( $($part:expr),* ) => {
        {
            let mut el = El::empty(seed::dom_types::Tag::Aside);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}

#[macro_export]
macro_rules! footer {
    ( $($part:expr),* ) => {
        {
            let mut el = El::empty(seed::dom_types::Tag::Footer);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}

#[macro_export]
macro_rules! header {
    ( $($part:expr),* ) => {
        {
            let mut el = El::empty(seed::dom_types::Tag::Header);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}

#[macro_export]
macro_rules! h1 {
    ( $($part:expr),* ) => {
        {
            let mut el = El::empty(seed::dom_types::Tag::H1);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}

#[macro_export]
macro_rules! h2 {
    ( $($part:expr),* ) => {
        {
            let mut el = El::empty(seed::dom_types::Tag::H2);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}

#[macro_export]
macro_rules! h3 {
    ( $($part:expr),* ) => {
        {
            let mut el = El::empty(seed::dom_types::Tag::H3);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}

#[macro_export]
macro_rules! h4 {
    ( $($part:expr),* ) => {
        {
            let mut el = El::empty(seed::dom_types::Tag::H4);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}

#[macro_export]
macro_rules! h5 {
    ( $($part:expr),* ) => {
        {
            let mut el = El::empty(seed::dom_types::Tag::H5);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}

#[macro_export]
macro_rules! h6 {
    ( $($part:expr),* ) => {
        {
            let mut el = El::empty(seed::dom_types::Tag::H6);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}

#[macro_export]
macro_rules! hgroup {
    ( $($part:expr),* ) => {
        {
            let mut el = El::empty(seed::dom_types::Tag::Hgroup);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}

#[macro_export]
macro_rules! main {
    ( $($part:expr),* ) => {
        {
            let mut el = El::empty(seed::dom_types::Tag::Main);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}

#[macro_export]
macro_rules! nav {
    ( $($part:expr),* ) => {
        {
            let mut el = El::empty(seed::dom_types::Tag::Nav);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}

#[macro_export]
macro_rules! section {
    ( $($part:expr),* ) => {
        {
            let mut el = El::empty(seed::dom_types::Tag::Section);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}

#[macro_export]
macro_rules! blockquote {
    ( $($part:expr),* ) => {
        {
            let mut el = El::empty(seed::dom_types::Tag::Blockquote);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}

#[macro_export]
macro_rules! dd {
    ( $($part:expr),* ) => {
        {
            let mut el = El::empty(seed::dom_types::Tag::Dd);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}

#[macro_export]
macro_rules! dir {
    ( $($part:expr),* ) => {
        {
            let mut el = El::empty(seed::dom_types::Tag::Dir);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}

#[macro_export]
macro_rules! div {
    ( $($part:expr),* ) => {
        {
            let mut el = El::empty(seed::dom_types::Tag::Div);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}

#[macro_export]
macro_rules! dl {
    ( $($part:expr),* ) => {
        {
            let mut el = El::empty(seed::dom_types::Tag::Dl);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}

#[macro_export]
macro_rules! dt {
    ( $($part:expr),* ) => {
        {
            let mut el = El::empty(seed::dom_types::Tag::Dt);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}

#[macro_export]
macro_rules! figure {
    ( $($part:expr),* ) => {
        {
            let mut el = El::empty(seed::dom_types::Tag::Figure);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}

#[macro_export]
macro_rules! figcaption {
    ( $($part:expr),* ) => {
        {
            let mut el = El::empty(seed::dom_types::Tag::FigCaption);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}

#[macro_export]
macro_rules! hr {
    ( $($part:expr),* ) => {
        {
            let mut el = El::empty(seed::dom_types::Tag::Hr);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}

#[macro_export]
macro_rules! li {
    ( $($part:expr),* ) => {
        {
            let mut el = El::empty(seed::dom_types::Tag::Li);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}

#[macro_export]
macro_rules! ol {
    ( $($part:expr),* ) => {
        {
            let mut el = El::empty(seed::dom_types::Tag::Ol);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}

#[macro_export]
macro_rules! p {
    ( $($part:expr),* ) => {
        {
            let mut el = El::empty(seed::dom_types::Tag::P);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}

#[macro_export]
macro_rules! pre {
    ( $($part:expr),* ) => {
        {
            let mut el = El::empty(seed::dom_types::Tag::Pre);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}

#[macro_export]
macro_rules! ul {
    ( $($part:expr),* ) => {
        {
            let mut el = El::empty(seed::dom_types::Tag::Ul);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}

#[macro_export]
macro_rules! a {
    ( $($part:expr),* ) => {
        {
            let mut el = El::empty(seed::dom_types::Tag::A);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}

#[macro_export]
macro_rules! abbr {
    ( $($part:expr),* ) => {
        {
            let mut el = El::empty(seed::dom_types::Tag::Abbr);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}

#[macro_export]
macro_rules! b {
    ( $($part:expr),* ) => {
        {
            let mut el = El::empty(seed::dom_types::Tag::B);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}

#[macro_export]
macro_rules! bdi {
    ( $($part:expr),* ) => {
        {
            let mut el = El::empty(seed::dom_types::Tag::Bdi);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}

#[macro_export]
macro_rules! bdo {
    ( $($part:expr),* ) => {
        {
            let mut el = El::empty(seed::dom_types::Tag::Bdo);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}

#[macro_export]
macro_rules! br {
    ( $($part:expr),* ) => {
        {
            let mut el = El::empty(seed::dom_types::Tag::Br);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}

#[macro_export]
macro_rules! cite {
    ( $($part:expr),* ) => {
        {
            let mut el = El::empty(seed::dom_types::Tag::Cite);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}

#[macro_export]
macro_rules! code {
    ( $($part:expr),* ) => {
        {
            let mut el = El::empty(seed::dom_types::Tag::Code);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}

#[macro_export]
macro_rules! data {
    ( $($part:expr),* ) => {
        {
            let mut el = El::empty(seed::dom_types::Tag::Data);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}

#[macro_export]
macro_rules! dfn {
    ( $($part:expr),* ) => {
        {
            let mut el = El::empty(seed::dom_types::Tag::Dfn);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}

#[macro_export]
macro_rules! em {
    ( $($part:expr),* ) => {
        {
            let mut el = El::empty(seed::dom_types::Tag::Em);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}

#[macro_export]
macro_rules! i {
    ( $($part:expr),* ) => {
        {
            let mut el = El::empty(seed::dom_types::Tag::I);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}

#[macro_export]
macro_rules! kbd {
    ( $($part:expr),* ) => {
        {
            let mut el = El::empty(seed::dom_types::Tag::Kbd);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}

#[macro_export]
macro_rules! mark {
    ( $($part:expr),* ) => {
        {
            let mut el = El::empty(seed::dom_types::Tag::Mark);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}

#[macro_export]
macro_rules! button {
    ( $($part:expr),* ) => {
        {
            let mut el = El::empty(seed::dom_types::Tag::Button);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}

#[macro_export]
macro_rules! fieldset {
    ( $($part:expr),* ) => {
        {
            let mut el = El::empty(seed::dom_types::Tag::FieldSet);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}

#[macro_export]
macro_rules! form {
    ( $($part:expr),* ) => {
        {
            let mut el = El::empty(seed::dom_types::Tag::Form);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}

#[macro_export]
macro_rules! input {
    ( $($part:expr),* ) => {
        {
            let mut el = El::empty(seed::dom_types::Tag::Input);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}

#[macro_export]
macro_rules! label {
    ( $($part:expr),* ) => {
        {
            let mut el = El::empty(seed::dom_types::Tag::Label);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}

#[macro_export]
macro_rules! legend {
    ( $($part:expr),* ) => {
        {
            let mut el = El::empty(seed::dom_types::Tag::Legend);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}

#[macro_export]
macro_rules! meter {
    ( $($part:expr),* ) => {
        {
            let mut el = El::empty(seed::dom_types::Tag::Meter);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}

#[macro_export]
macro_rules! optgroup {
    ( $($part:expr),* ) => {
        {
            let mut el = El::empty(seed::dom_types::Tag::OptGroup);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}

#[macro_export]
macro_rules! option {
    ( $($part:expr),* ) => {
        {
            let mut el = El::empty(seed::dom_types::Tag::Option);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}

#[macro_export]
macro_rules! output {
    ( $($part:expr),* ) => {
        {
            let mut el = El::empty(seed::dom_types::Tag::Output);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}

#[macro_export]
macro_rules! progress {
    ( $($part:expr),* ) => {
        {
            let mut el = El::empty(seed::dom_types::Tag::Progress);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}

#[macro_export]
macro_rules! select {
    ( $($part:expr),* ) => {
        {
            let mut el = El::empty(seed::dom_types::Tag::Select);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}

#[macro_export]
macro_rules! textarea {
    ( $($part:expr),* ) => {
        {
            let mut el = El::empty(seed::dom_types::Tag::TextArea);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}

#[macro_export]
macro_rules! span {
    ( $($part:expr),* ) => {
        {
            let mut el = El::empty(seed::dom_types::Tag::Span);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}

#[macro_export]
macro_rules! strong {
    ( $($part:expr),* ) => {
        {
            let mut el = El::empty(seed::dom_types::Tag::Strong);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}

#[macro_export]
macro_rules! sub {
    ( $($part:expr),* ) => {
        {
            let mut el = El::empty(seed::dom_types::Tag::Sub);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}

#[macro_export]
macro_rules! sup {
    ( $($part:expr),* ) => {
        {
            let mut el = El::empty(seed::dom_types::Tag::Sup);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}

#[macro_export]
macro_rules! img {
    ( $($part:expr),* ) => {
        {
            let mut el = El::empty(seed::dom_types::Tag::Img);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}

#[macro_export]
macro_rules! canvas {
    ( $($part:expr),* ) => {
        {
            let mut el = El::empty(seed::dom_types::Tag::Canvas);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}

#[macro_export]
macro_rules! noscript {
    ( $($part:expr),* ) => {
        {
            let mut el = El::empty(seed::dom_types::Tag::NoScript);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}

#[macro_export]
macro_rules! script {
    ( $($part:expr),* ) => {
        {
            let mut el = El::empty(seed::dom_types::Tag::Script);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}

#[macro_export]
macro_rules! del {
    ( $($part:expr),* ) => {
        {
            let mut el = El::empty(seed::dom_types::Tag::Del);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}

#[macro_export]
macro_rules! ins {
    ( $($part:expr),* ) => {
        {
            let mut el = El::empty(seed::dom_types::Tag::Ins);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}

#[macro_export]
macro_rules! map {
    ( $($part:expr),* ) => {
        {
            let mut el = El::empty(seed::dom_types::Tag::Map);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}

#[macro_export]
macro_rules! track {
    ( $($part:expr),* ) => {
        {
            let mut el = El::empty(seed::dom_types::Tag::Track);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}

#[macro_export]
macro_rules! video {
    ( $($part:expr),* ) => {
        {
            let mut el = El::empty(seed::dom_types::Tag::Video);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}

#[macro_export]
macro_rules! applet {
    ( $($part:expr),* ) => {
        {
            let mut el = El::empty(seed::dom_types::Tag::Applet);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}

#[macro_export]
macro_rules! embed {
    ( $($part:expr),* ) => {
        {
            let mut el = El::empty(seed::dom_types::Tag::Embed);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}

#[macro_export]
macro_rules! iframe {
    ( $($part:expr),* ) => {
        {
            let mut el = El::empty(seed::dom_types::Tag::Iframe);
            $ ( $part.update(&mut el); )*
            el
        }
    };
}

//// End element-creaion macros.


/// Provide a shortcut for creating attributes.
// todo DRY between thsi and style
#[macro_export]
macro_rules! attrs {
    { $($key:expr => $value:expr);* } => {
        {
            let mut vals = std::collections::HashMap::new();
            $(
                // We can handle arguments of multiple types by using this:
                // Strings, &strs, bools, numbers etc.
                vals.insert(String::from($key), $value.to_string());
            )*
            seed::dom_types::Attrs::new(vals)
        }
     };
}

/// Provide a shortcut for creating styles.
#[macro_export]
macro_rules! style {
    { $($key:expr => $value:expr);* } => {
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


#[macro_export]
macro_rules! events {
    { $($event_str:expr => $handler:expr);+ } => {
        {
            let mut result = Vec::new();
            $(
                match $event_str {
                    _ => result.push(seed::dom_types::Listener::new_input($event_str.into(), $handler)),
//                    "input" => result.push(seed::dom_types::Listener::new_input($event_str.into(), $handler)),
//                    _ => result.push(seed::dom_types::Listener::new($event_str.into(), $handler)),

                }

//                result.push(seed::dom_types::Listener::new($event_str.into(), $handler));
            )+
            result
        }
     };
}

