#![allow(clippy::needless_pass_by_value, clippy::trivially_copy_pass_by_ref)]

use seed::{prelude::*, *};

// ------ ------
//     Init
// ------ ------

fn init(_: Url, _: &mut impl Orders<Msg>) -> Model {
    Model::default()
}

// ------ ------
//     Model
// ------ ------

type Model = i32;

// ------ ------
//    Update
// ------ ------

enum Msg {
    Increment,
    Decrement,
}

fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::Increment => *model += 1,
        Msg::Decrement => *model -= 1,
    }
}

// ------ ------
//     View
// ------ ------

fn view(model: &Model) -> Node<Msg> {
    div![
        C!["container", "my-container"],
        style! {St::Margin => px(5), St::Padding => 0},
        button![ev(Ev::Click, |_| Msg::Decrement), "-"],
        div![model],
        button![ev(Ev::Click, |_| Msg::Increment), "+"],
        hr![],
        svg![
            attrs! {
                At::Width => 50,
                At::Height => 50,
                At::ViewBox => "0 0 100 100",
            },
            circle![attrs! {
                At::Cx => 50, At::Cy => 50, At::R => 50,
            },],
        ],
        hr![],
        img![attrs! {
            At::Src => "/public/leopard.jpg",
            At::Alt => "An intimidating leopard.",
        },],
    ]
}

// ------ ------
//     Start
// ------ ------

#[wasm_bindgen(start)]
pub fn start() {
    App::start("app", init, update, view);
}

// ------ ------
//     Tests
// ------ ------

#[cfg(test)]
mod tests {
    use super::{view, Model};
    use regex::Regex;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    pub fn view_test() {
        // ---- ARRANGE ----
        let model: Model = 123;
        let node = view(&model);

        // ---- ACT ----
        let html = node.to_string();

        // ---- ASSERT
        let expected_html = r#"
            <div class="container my-container" style="margin:5px;padding:0">
                <button>-</button>
                <div>123</div>
                <button>+</button>
                <hr>
                <svg width="50" height="50" viewBox="0 0 100 100" xmlns="http://www.w3.org/2000/svg">
                    <circle cx="50" cy="50" r="50" xmlns="http://www.w3.org/2000/svg"></circle>
                </svg>
                <hr>
                <img src="/public/leopard.jpg" alt="An intimidating leopard.">
            </div>
        "#;
        let extra_whitespace = Regex::new(r"\n *").unwrap();
        let expected_ugly_html = extra_whitespace.replace_all(expected_html, "");
        assert_eq!(html, expected_ugly_html)
    }
}
