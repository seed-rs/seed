use fluent::fluent_args;
use seed::{prelude::*, *};
use strum::IntoEnumIterator;

mod i18n;
use i18n::{I18n, Lang};

mod resource;

// ------ ------
//     Init
// ------ ------
const DEFAULT_LANG: Lang = Lang::EnUS;

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
                .set_lang(lang.parse().expect("supported language"));
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
            Lang::iter().map(|lang| option![attrs! {At::Value => lang.as_ref()}, lang.label()]),
            input_ev(Ev::Change, Msg::LangChanged),
        ],],
        div![p!["Language in Model: ", model.i18n.lang().label()]],
        div![],
        div![
            p![model.i18n.translate("hello-world", None)],
            p![model.i18n.translate("hello-user", Some(&args_male_sg))],
            p![model.i18n.translate("shared-photos", Some(&args_male_sg))],
            p![model.i18n.translate("tabs-close-button", None)],
            p![model
                .i18n
                .translate("tabs-close-tooltip", Some(&args_male_sg))],
            p![model
                .i18n
                .translate("tabs-close-warning", Some(&args_male_sg))],
            p![model.i18n.translate("hello-user", Some(&args_female_pl))],
            p![model.i18n.translate("shared-photos", Some(&args_female_pl))],
            p![model.i18n.translate("tabs-close-button", None)],
            p![model
                .i18n
                .translate("tabs-close-tooltip", Some(&args_female_pl))],
            p![model
                .i18n
                .translate("tabs-close-warning", Some(&args_female_pl))],
            p![model.i18n.translate("sync-dialog-title", None)],
            p![model.i18n.translate("sync-headline-title", None)],
            p![model.i18n.translate("sync-signedout-title", None)],
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
