use rand::prelude::*;
use seed::{prelude::*, *};

type CarColor = String;

// ------ ------
//     Init
// ------ ------

fn init(_: Url, orders: &mut impl Orders<Msg>) -> Model {
    orders
        .send_msg(Msg::SetViewportWidth)
        .stream(streams::window_event(Ev::Resize, |_| Msg::SetViewportWidth))
        .after_next_render(Msg::Rendered);

    Model::default()
}

// ------ ------
//     Model
// ------ ------

#[derive(Default)]
struct Model {
    viewport_width: f64,
    car: Car,
}

// ------ Car ------

#[derive(Debug)]
struct Car {
    x: f64,
    y: f64,
    speed: f64,
    color: CarColor,
    width: f64,
    height: f64,
}

impl Car {
    /// Pixels per second.
    /// _Note_:
    /// Optional feature "wasm-bindgen" has to be enabled for crate `rand` (otherwise it panics).
    fn generate_speed() -> f64 {
        thread_rng().gen_range(400., 800.)
    }

    fn generate_color() -> CarColor {
        let hue = thread_rng().gen_range(0, 360);
        format!("hsl({}, 80%, 50%)", hue)
    }
}

impl Default for Car {
    fn default() -> Self {
        let car_width = 120.;
        Self {
            x: -car_width,
            y: 100.,
            speed: Self::generate_speed(),
            color: Self::generate_color(),
            width: car_width,
            height: 60.,
        }
    }
}

// ------ ------
//    Update
// ------ ------

#[derive(Copy, Clone)]
enum Msg {
    Rendered(RenderInfo),
    SetViewportWidth,
}

fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::Rendered(render_info) => {
            let delta = render_info.timestamp_delta.unwrap_or_default();
            if delta > 0. {
                // Move car at least 1px to the right.
                model.car.x += f64::max(1., delta / 1000. * model.car.speed);

                // We don't see car anymore => back to start + generate new color and speed.
                if model.car.x > model.viewport_width {
                    model.car = Car::default();
                }
            }
            orders.after_next_render(Msg::Rendered);
        }
        Msg::SetViewportWidth => {
            model.viewport_width = f64::from(body().client_width());
            orders.skip();
        }
    }
}

// ------ ------
//     View
// ------ ------

fn view(model: &Model) -> Node<Msg> {
    // Scene container, also represents sky.
    div![
        id!("animation"),
        style! {
            St::Overflow => "hidden";
            St::Width => unit!(100, %);
            St::Position => "relative";
            St::Height => unit!(170, px);
            St::BackgroundColor => "deepskyblue";
        },
        // Road.
        div![style! {
            St::Width => unit!(100, %);
            St::Height => unit!(20, px);
            St::Bottom => 0;
            St::BackgroundColor => "darkgray";
            St::Position => "absolute";
        }],
        view_car(&model.car)
    ]
}

fn view_car(car: &Car) -> Node<Msg> {
    div![
        // Car container.
        style! {
            St::Width => unit!(car.width, px);
            St::Height => unit!(car.height, px);
            St::Top => unit!(car.y, px);
            St::Left => unit!(car.x, px);
            St::Position => "absolute";
        },
        // Windows.
        div![style! {
            St::BackgroundColor => "rgb(255, 255, 255, 0.5)";
            St::Left => unit!(car.width * 0.25, px);
            St::Width => unit!(car.width * 0.5, px);
            St::Height => unit!(car.height * 0.6, px);
            St::BorderRadius => unit!(9999, px);
            St::Position => "absolute";
        }],
        // Body.
        div![style! {
            St::Top => unit!(car.height * 0.35, px);
            St::BackgroundColor => car.color;
            St::Width => unit!(car.width, px);
            St::Height => unit!(car.height * 0.5, px);
            St::BorderRadius => unit!(9999, px);
            St::Position => "absolute";
        }],
        view_wheel(car.width * 0.15, car),
        view_wheel(car.width * 0.6, car)
    ]
}

fn view_wheel(wheel_x: f64, car: &Car) -> Node<Msg> {
    let wheel_radius = car.height * 0.4;
    div![style! {
        St::Top => unit!(car.height * 0.55, px);
        St::Left => unit!(wheel_x, px);
        St::BackgroundColor => "black";
        St::Width => unit!(wheel_radius, px);
        St::Height => unit!(wheel_radius, px);
        St::BorderRadius => unit!(9999, px);
        St::Position => "absolute";
    }]
}

fn main() {
    App::start("app", init, update, view);
}
