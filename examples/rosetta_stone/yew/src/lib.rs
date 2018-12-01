#[macro_use]
extern crate yew;
use yew::prelude::*;

struct Model {clicks: i8}

impl Default for Model {
    fn default() -> Self {
        Self {clicks: 0}
    }
}

enum Msg {
    Increment,
    Decrement,
}


impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_:Self::Properties, _: ComponentLink<Self>) -> Self {
        Model::default()
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Increment => {
                self.clicks += 1
            },
            Msg::Decrement => {
                self.clicks += 1
            }
        }
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        html! {
            <div>
                <button onclick=|_| Msg::Increment,>{ "Click me!"}</button>
                <h3>{self.clicks}</h3>
                </div>
        }
    }
}

fn main() {
    yew::initialize();
    App::<Model>::new().mount_to_body();
    yew::run_loop();
}