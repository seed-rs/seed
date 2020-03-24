#![allow(clippy::filter_map)]

use graphql_client::{GraphQLQuery, Response as GQLResponse};
use seed::{prelude::*, *};
use serde::{Deserialize, Serialize};

const API_URL: &str = "https://countries.trevorblades.com/";

type Code = String;

// ------ ------
//    GraphQL
// ------ ------

macro_rules! generate_query {
    ($query:ident) => {
        #[derive(GraphQLQuery)]
        #[graphql(
            schema_path = "graphql/schema.graphql",
            query_path = "graphql/queries.graphql",
            response_derives = "Debug"
        )]
        struct $query;
    };
}
generate_query!(QContinents);
generate_query!(QContinent);
generate_query!(QCountry);

async fn send_graphql_request<V, T>(variables: &V) -> fetch::Result<T>
where
    V: Serialize,
    T: for<'de> Deserialize<'de> + 'static,
{
    Request::new(API_URL)
        .method(Method::Post)
        .json(variables)?
        .fetch()
        .await?
        .check_status()?
        .json()
        .await
}

// ------ ------
//     Init
// ------ ------

fn init(_: Url, orders: &mut impl Orders<Msg>) -> Model {
    orders.perform_cmd(async {
        Msg::ContinentsFetched(
            send_graphql_request(&QContinents::build_query(q_continents::Variables)).await,
        )
    });
    Model::default()
}

// ------ ------
//     Model
// ------ ------

#[derive(Default)]
struct Model {
    continents: Option<Vec<Option<q_continents::QContinentsContinents>>>,
    selected_continent: Option<Code>,
    countries: Option<Vec<Option<q_continent::QContinentContinentCountries>>>,
    selected_country: Option<Code>,
    country: Option<q_country::QCountryCountry>,
}

// ------ ------
//    Update
// ------ ------

enum Msg {
    ContinentsFetched(fetch::Result<GQLResponse<q_continents::ResponseData>>),
    ContinentClicked(Code),
    CountriesFetched(fetch::Result<GQLResponse<q_continent::ResponseData>>),
    CountryClicked(Code),
    CountryFetched(fetch::Result<GQLResponse<q_country::ResponseData>>),
}

fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::ContinentsFetched(Ok(GQLResponse {
            data: Some(data), ..
        })) => {
            model.continents = data.continents;
        }
        Msg::ContinentsFetched(error) => log!(error),
        Msg::ContinentClicked(code) => {
            model.selected_continent = Some(code.clone());
            orders.perform_cmd(async {
                Msg::CountriesFetched(
                    send_graphql_request(&QContinent::build_query(q_continent::Variables {
                        code: Some(code),
                    }))
                    .await,
                )
            });
        }
        Msg::CountriesFetched(Ok(GQLResponse {
            data: Some(data), ..
        })) => {
            model.countries = data.continent.and_then(|continent| continent.countries);
        }
        Msg::CountriesFetched(error) => log!(error),
        Msg::CountryClicked(code) => {
            model.selected_country = Some(code.clone());
            orders.perform_cmd(async {
                Msg::CountryFetched(
                    send_graphql_request(&QCountry::build_query(q_country::Variables {
                        code: Some(code),
                    }))
                    .await,
                )
            });
        }
        Msg::CountryFetched(Ok(GQLResponse {
            data: Some(data), ..
        })) => {
            model.country = data.country;
        }
        Msg::CountryFetched(error) => log!(error),
    }
}

// ------ ------
//     View
// ------ ------

fn view(model: &Model) -> Node<Msg> {
    let continents = model.continents.as_ref().map(|continents| {
        continents
            .iter()
            .filter_map(Option::as_ref)
            .map(|continent| continent_row(continent, &model.selected_continent))
            .collect::<Vec<_>>()
    });

    let countries = model.countries.as_ref().map(|countries| {
        countries
            .iter()
            .filter_map(Option::as_ref)
            .map(|country| country_row(country, &model.selected_country))
            .collect::<Vec<_>>()
    });

    div![
        C!["container"],
        div![
            C!["columns"],
            column("Continents", continents),
            column("Countries", countries),
            column("Country", model.country.as_ref().map(country_detail)),
        ]
    ]
}

fn column(title: &str, content: impl IntoNodes<Msg>) -> Node<Msg> {
    div![
        C!["column"],
        div![
            C!["box"],
            div![
                C!["menu"],
                p![C!["menu-label"], title,],
                ul![
                    C!["menu-list"],
                    style! {
                        St::MaxHeight => vh(80),
                        St::OverflowY => "auto",
                    },
                    content.into_nodes(),
                ]
            ],
        ]
    ]
}

fn continent_row(
    continent: &q_continents::QContinentsContinents,
    selected: &Option<Code>,
) -> Node<Msg> {
    li![a![
        C![IF!(&continent.code == selected => "is-active")],
        &continent.name,
        continent
            .code
            .clone()
            .map(|code| ev(Ev::Click, move |_| Msg::ContinentClicked(code))),
    ],]
}

fn country_row(
    country: &q_continent::QContinentContinentCountries,
    selected: &Option<Code>,
) -> Node<Msg> {
    li![a![
        C![IF!(&country.code == selected => "is-active")],
        &country.name,
        country
            .code
            .clone()
            .map(|code| ev(Ev::Click, move |_| Msg::CountryClicked(code))),
    ],]
}

#[allow(clippy::cognitive_complexity)]
fn country_detail(country: &q_country::QCountryCountry) -> Node<Msg> {
    div![
        C!["content"],
        p![C!["title", "is-5"], &country.name],
        p![
            "Code: ",
            span![C!["has-text-weight-semibold"], &country.code]
        ],
        p![
            "Native name: ",
            span![C!["has-text-weight-semibold"], &country.native]
        ],
        p![
            "Currency: ",
            span![C!["has-text-weight-semibold"], &country.currency]
        ],
        p![
            "Phone prefix: ",
            span![C!["has-text-weight-semibold"], &country.phone]
        ],
        country
            .languages
            .as_ref()
            .map(|languages| { p!["Languages: ", view_languages(languages)] }),
        country.states.as_ref().and_then(|states| {
            IF!(not(states.is_empty()) => p!["States: ", view_states(states)])
        })
    ]
}

fn view_languages(languages: &[Option<q_country::QCountryCountryLanguages>]) -> Node<Msg> {
    ul![languages.iter().filter_map(Option::as_ref).map(|lang| {
        li![
            &lang.name,
            IF!(matches!(lang.rtl, Some(rtl) if rtl == 1) => " (RTL)"),
        ]
    })]
}

fn view_states(states: &[Option<q_country::QCountryCountryStates>]) -> Node<Msg> {
    ul![states
        .iter()
        .filter_map(Option::as_ref)
        .map(|state| li![&state.name])]
}

// ------ ------
//     Start
// ------ ------

#[wasm_bindgen(start)]
pub fn start() {
    App::start("app", init, update, view);
}
