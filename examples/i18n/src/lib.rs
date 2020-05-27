use seed::{prelude::*, *};

use fluent::{FluentArgs, FluentValue};
use strum::IntoEnumIterator;

mod i18n;
use crate::i18n::{I18n, Lang, translate};
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
                "en-US" => model.i18n.set_lang(Lang::en_US),
                "de-DE" => model.i18n.set_lang(Lang::de_DE),
                _ => model.i18n.set_lang(DEFAULT_LANG),
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
        div![p!["Language in Model: ", model.i18n.lang().label()]],
        div![],
        div![
            p![translate(&model.i18n, None, "hello-world")],
            p![translate(&model.i18n, Some(&args_male_sg), "hello-user")],
            p![translate(&model.i18n, Some(&args_male_sg), "shared-photos")],
            p![translate(&model.i18n, None, "tabs-close-button")],
            p![translate(&model.i18n, Some(&args_male_sg), "tabs-close-tooltip")],
            p![translate(&model.i18n, Some(&args_male_sg), "tabs-close-warning")],
            p![translate(&model.i18n, Some(&args_female_pl), "hello-user")],
            p![translate(&model.i18n, Some(&args_female_pl), "shared-photos")],
            p![translate(&model.i18n, None, "tabs-close-button")],
            p![translate(
                &model.i18n,
                Some(&args_female_pl),
                "tabs-close-tooltip"
            )],
            p![translate(
                &model.i18n,
                Some(&args_female_pl),
                "tabs-close-warning"
            )],
            p![translate(&model.i18n, None, "sync-dialog-title")],
            p![translate(&model.i18n, None, "sync-headline-title")],
            p![translate(&model.i18n, None, "sync-signedout-title")],
        ],
    ]
}

// ------ ------
//     Start
// ------ ------

#[wasm_bindgen(start)]
pub fn start() {
    App::start("app", init, update, view);
}
