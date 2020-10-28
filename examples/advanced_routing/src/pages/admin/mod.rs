use crate::router;
use crate::Routes;
pub use router::View;
use router::*;

use seed::prelude::wasm_bindgen::__rt::std::collections::HashMap;
use seed::{prelude::*, *};

pub fn init(
    _: Url,
    _: &mut Model,
    id: &str,
    children: &AdminRoutes,
    orders: &mut impl Orders<Msg>,
) -> Model {
    let models = load_models();
    let model_to_load = models.get(id);

    if let Some((name, description)) = model_to_load {
        Model {
            id: id.to_string(),
            name: name.to_string(),
            description: description.to_string(),
        }
    } else if !children.eq(&AdminRoutes::NotFound) {
        // todo need to simplify with reusing the root of the url;
        // maybe a function like to_parent_url which merge the root url for example
        orders.notify(subs::UrlRequested::new(
            Routes::Admin {
                id: id.to_string(),
                children: AdminRoutes::NotFound,
            }
            .to_url(),
        ));
        let mut not_found_model = Model::default();
        not_found_model.id = id.to_string();
        not_found_model
    } else {
        let mut not_found_model = Model::default();
        not_found_model.id = id.to_string();
        not_found_model
    }
}
#[derive(Default)]
pub struct Model {
    id: String,
    name: String,
    description: String,
}

pub enum Msg {}

#[derive(Debug, PartialEq, Clone, RoutingModules)]
pub enum AdminRoutes {
    #[view = " => root"]
    Root,
    #[view = " => manager"]
    Manager,
    #[default_route]
    #[view = " => not_found"]
    NotFound,
}

pub fn update(_: Msg, _: &mut Model, _: &mut impl Orders<Msg>) {}

pub fn view(routes: &AdminRoutes, model: &Model) -> Node<Msg> {
    routes.view(model)
}
fn manager(model: &Model) -> Node<Msg> {
    div![
        "Management",
        h3![&model.name],
        br![],
        p![&model.description]
    ]
}
fn root(model: &Model) -> Node<Msg> {
    div!["Root", h3![&model.name], br![], p![&model.description]]
}
fn not_found(model: &Model) -> Node<Msg> {
    div!["model not found with id ", span![&model.id]]
}

fn load_models() -> HashMap<String, (String, String)> {
    let mut models: HashMap<String, (String, String)> = HashMap::new();

    models.insert(
        "1".to_string(),
        (
            "Custom Router".to_string(),
            "Develop a Custom Router for Seed".to_string(),
        ),
    );
    models.insert(
        "2".to_string(),
        (
            "Seed Router".to_string(),
            "Help to make an official Router for Seed".to_string(),
        ),
    );

    models
}
