use seed::{prelude::*, *};

use fluent::fluent_args;
use strum::IntoEnumIterator;

mod i18n;
use crate::i18n::{translate, I18n, Lang};
use std::str::FromStr;

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
            model
                .i18n
                .set_lang(Lang::from_str(lang.as_str()).expect("supported language"));
        }
    }
}

// ------ ------
//     View
// ------ ------

fn view(model: &Model) -> impl IntoNodes<Msg> {
    let args_male_sg = fluent_args![
      "userName" => "Stephan",
      "photoCount" => 1,
      "userGender" => "male",
      "tabCount" => 1,
      "formal" => "true"
    ];

    let args_female_pl = fluent_args![
      "userName" => "Anna",
      "photoCount" => 5,
      "userGender" => "female",
      "tabCount" => 7,
      "formal" => "false"
    ];

    div![
        div![select![
            attrs! {At::Name => "lang"},
            Lang::iter().map(|lang| option![attrs! {At::Value => lang.id()}, lang.label()]),
            input_ev(Ev::Change, Msg::LangChanged),
        ],],
        div![p!["Language in Model: ", model.i18n.lang().label()]],
        div![],
        div![
            p![translate(&model.i18n, None, "hello-world")],
            p![translate(&model.i18n, Some(&args_male_sg), "hello-user")],
            p![translate(&model.i18n, Some(&args_male_sg), "shared-photos")],
            p![translate(&model.i18n, None, "tabs-close-button")],
            p![translate(
                &model.i18n,
                Some(&args_male_sg),
                "tabs-close-tooltip"
            )],
            p![translate(
                &model.i18n,
                Some(&args_male_sg),
                "tabs-close-warning"
            )],
            p![translate(&model.i18n, Some(&args_female_pl), "hello-user")],
            p![translate(
                &model.i18n,
                Some(&args_female_pl),
                "shared-photos"
            )],
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
