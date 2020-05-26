// Some Clippy linter rules are ignored for the sake of simplicity.
#![allow(clippy::needless_pass_by_value, clippy::trivially_copy_pass_by_ref)]

use seed::{prelude::*, *};

use fluent::{FluentArgs, FluentBundle, FluentResource, FluentValue};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use unic_langid::LanguageIdentifier;

use std::borrow::Borrow;

mod resource;

// ------ ------
//     Init
// ------ ------

fn init(_: Url, _: &mut impl Orders<Msg>) -> Model {
    let mut model = Model::default();
    change_lang(&mut model);
    model
}

// ------ ------
//     Model
// ------ ------
pub struct Model {
    lang: Lang,
    resource: FluentBundle<FluentResource>,
}

impl Default for Model {
    fn default() -> Self {
        Model {
            lang: Lang::en_US,
            resource: FluentBundle::default(),
        }
    }
}

// ------ Lang ------
#[allow(non_camel_case_types)]
#[derive(Clone, Copy, EnumIter, PartialEq)]
enum Lang {
    en_US,
    de_DE,
}

impl Lang {
    fn id(&self) -> &str {
        match self {
            Lang::en_US => "en-US",
            Lang::de_DE => "de-DE",
        }
    }
    fn label(&self) -> String {
        match self {
            Lang::en_US => "English (US)".to_string(),
            Lang::de_DE => "Deutsch (Deutschland)".to_string(),
        }
    }
}

// ------ ------
//    Update
// ------ ------

enum Msg {
    LangChanged(String),
}

fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::LangChanged(lang) => {
            match lang.as_str() {
                "en-US" => model.lang = Lang::en_US,
                "de-DE" => model.lang = Lang::de_DE,
                _ => model.lang = Lang::en_US,
            }
            change_lang(model)
        }
    }
}

// ------ ------
//     View
// ------ ------

fn view(model: &Model) -> impl IntoNodes<Msg> {
    let mut langs: Vec<Node<Msg>> = Vec::new();
    for lang in Lang::iter() {
        langs.push(option![attrs! {At::Value => lang.id()}, lang.label()]);
    }

    let mut args_male_sg = FluentArgs::new();
    args_male_sg.insert("userName", FluentValue::from("Stephan"));
    args_male_sg.insert("photoCount", FluentValue::from(1));
    args_male_sg.insert("userGender", FluentValue::from("male"));
    args_male_sg.insert("tabCount", FluentValue::from(1));
    args_male_sg.insert("formal", FluentValue::from("true"));

    let mut args_female_pl = FluentArgs::new();
    args_female_pl.insert("userName", FluentValue::from("Anna"));
    args_female_pl.insert("photoCount", FluentValue::from(5));
    args_female_pl.insert("userGender", FluentValue::from("female"));
    args_female_pl.insert("tabCount", FluentValue::from(7));
    args_female_pl.insert("formal", FluentValue::from("false"));

    div![
        div![select![
            attrs! {At::Name => "lang"},
            langs,
            input_ev(Ev::Change, Msg::LangChanged),
        ],],
        div![p!["Language in Model: ", model.lang.label()]],
        div![],
        div![
            p![translate(model, None, "hello-world")],
            p![translate(model, Some(&args_male_sg), "hello-user")],
            p![translate(model, Some(&args_male_sg), "shared-photos")],
            p![translate(model, None, "tabs-close-button")],
            p![translate(model, Some(&args_male_sg), "tabs-close-tooltip")],
            p![translate(model, Some(&args_male_sg), "tabs-close-warning")],
            p![translate(model, Some(&args_female_pl), "hello-user")],
            p![translate(model, Some(&args_female_pl), "shared-photos")],
            p![translate(model, None, "tabs-close-button")],
            p![translate(
                model,
                Some(&args_female_pl),
                "tabs-close-tooltip"
            )],
            p![translate(
                model,
                Some(&args_female_pl),
                "tabs-close-warning"
            )],
            p![translate(model, None, "sync-dialog-title")],
            p![translate(model, None, "sync-headline-title")],
            p![translate(model, None, "sync-signedout-title")],
        ],
    ]
}

fn change_lang(model: &mut Model) {
    let res = FluentResource::try_new(
        resource::Resources::new()
            .get(model.lang.id().to_string().borrow())
            .unwrap()
            .parse()
            .unwrap(),
    )
    .expect("Failed to parse an FTL string.");
    let locale: LanguageIdentifier = model.lang.id().parse().expect("Parsing failed");
    let mut bundle = FluentBundle::new(&[locale]);
    bundle
        .add_resource(res)
        .expect("Failed to add FTL resources to the bundle.");

    model.resource = bundle;
}

fn translate(model: &Model, args: Option<&FluentArgs>, label: &str) -> String {
    let msg = model
        .resource
        .get_message(label)
        .expect("Message doesn't exist.");
    let mut errors = vec![];
    let pattern = msg.value.expect("Message has no value.");

    let value = model.resource.format_pattern(&pattern, args, &mut errors);
    value.into()
}

// ------ ------
//     Start
// ------ ------

#[wasm_bindgen(start)]
pub fn start() {
    App::start("app", init, update, view);
}
