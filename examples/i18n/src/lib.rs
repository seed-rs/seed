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
const DEFAULT_LANG: Lang = Lang::en_US;

fn init(_: Url, _: &mut impl Orders<Msg>) -> Model {
    Model {
        i18n: I18n::new(DEFAULT_LANG),
    }
}

// ------ ------
//     Model
// ------ ------
pub struct Model {
    i18n: I18n,
}

// ------ I18n ------

pub struct I18n {
    lang: Lang,
    resource: FluentBundle<FluentResource>,
}

impl I18n {
    fn new(lang: Lang) -> Self {
        let mut i18n = I18n {
            lang,
            resource: FluentBundle::default(),
        };
        i18n.lang(lang);
        i18n
    }
    fn lang(&mut self, lang: Lang) -> &Self {
        self.lang = lang;
        let res = FluentResource::try_new(
            resource::Resources::new()
                .get(lang.id().to_string().borrow())
                .unwrap()
                .parse()
                .unwrap(),
        )
        .expect("Failed to parse an FTL string.");
        let locale: LanguageIdentifier = lang.id().parse().expect("Parsing failed");
        let mut bundle = FluentBundle::new(&[locale]);
        bundle
            .add_resource(res)
            .expect("Failed to add FTL resources to the bundle.");

        self.resource = bundle;
        self
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
    fn label(&self) -> &str {
        match self {
            Lang::en_US => "English (US)",
            Lang::de_DE => "Deutsch (Deutschland)",
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
                "en-US" => model.i18n.lang(Lang::en_US),
                "de-DE" => model.i18n.lang(Lang::de_DE),
                _ => model.i18n.lang(DEFAULT_LANG),
            };
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
        div![p!["Language in Model: ", model.i18n.lang.label()]],
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

fn translate(model: &Model, args: Option<&FluentArgs>, label: &str) -> String {
    let msg = model
        .i18n
        .resource
        .get_message(label)
        .expect("Message doesn't exist.");
    let mut errors = vec![];
    let pattern = msg.value.expect("Message has no value.");

    let value = model
        .i18n
        .resource
        .format_pattern(&pattern, args, &mut errors);
    value.into()
}

// ------ ------
//     Start
// ------ ------

#[wasm_bindgen(start)]
pub fn start() {
    App::start("app", init, update, view);
}
