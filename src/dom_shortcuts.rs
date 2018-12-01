//! This file exports helper macros for element creation, populated by a higher-level macro,
//! and macros for creating the parts of elements. (attrs, style, events)


use web_sys;

/// Create macros exposed to the package that allow shortcuts for Dom elements.
/// In the matching mattern below, we match the name we want to use with the name under
/// the Tag enum. Eg the div! macro uses Tag::Div.
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


macro_rules! div {
    ( $($part:expr),* ) => {
        {
            let mut el = El::empty(Tag::Div);
            let mut arg_count = 0;
            $ ( $part.update(&mut el); arg_count += 1;)*
            if arg_count > 5{
            // todo there are ways to sort this out, I think. Have update methods
            // todo accept/return a value; have them return an err type etc.
                web_sys::console::log_1(&"Element-creation macros can take no more than 5 arguments.".into());
            }
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

macro_rules! select {
    ( $($part:expr),* ) => {
        {
            let mut el = El::empty(Tag::Select);
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

macro_rules! a {
    ( $($part:expr),* ) => {
        {
            let mut el = El::empty(Tag::A);
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


/// Provide a shortcut for creating styles.
#[macro_export]
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
#[macro_export]
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

//// todo DRY between thsi and style
#[macro_export]
macro_rules! events {
    { $($event_str:expr => $msg:expr);+ } => {
        {
            let mut vals = Vec::new();
            $(
                vals.push(($event_str.into(), $msg));
            )+
            Events::new(vals)
        }
     };
}

