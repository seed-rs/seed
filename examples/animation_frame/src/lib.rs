#[macro_use]
extern crate seed;
use rand::prelude::*;
use seed::prelude::*;

// Model

type CarColor = String;

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
        format!("hsl({},100%,50%)", hue)
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

#[derive(Default)]
struct Model {
    request_animation_frame_handle: Option<RequestAnimationFrameHandle>,
    previous_time: Option<RequestAnimationFrameTime>,
    viewport_width: f64,
    car: Car,
}

// Update

#[derive(Clone, Copy)]
enum Msg {
    SetViewportWidth,
    NextAnimationStep,
    OnAnimationFrame(RequestAnimationFrameTime),
}

fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::SetViewportWidth => {
            model.viewport_width = f64::from(seed::body().client_width());
            orders.skip();
        }
        Msg::NextAnimationStep => {
            let (app, msg_mapper) = (orders.clone_app(), orders.msg_mapper());

            let cb = Closure::new(move |time| {
                app.update(msg_mapper(Msg::OnAnimationFrame(time)));
            });

            model.request_animation_frame_handle = Some(request_animation_frame(cb));
            orders.skip();
        }
        Msg::OnAnimationFrame(time) => {
            let delta = match model.previous_time {
                Some(previous_time) => time - previous_time,
                None => 0.,
            };
            model.previous_time = Some(time);

            if delta > 0. {
                // move car at least 1px to the right
                model.car.x += f64::max(1., delta / 1000. * model.car.speed);

                // we don't see car anymore => back to start + generate new color and speed
                if model.car.x > model.viewport_width {
                    model.car = Car::default();
                }
            }
            orders.send_msg(Msg::NextAnimationStep);
        }
    }
}

// View

fn view(model: &Model) -> Node<Msg> {
    // scene container + sky
    div![
        style! {
          "overflow" => "hidden";
          "width" => unit!(100, %);
          "position" => "relative";
          "height" => unit!(170, px);
          "background-color" => "deepskyblue";
        },
        // road
        div![style! {
            "width" => unit!(100, %);
            "height" => unit!(20, px);
            "bottom" => 0;
            "background-color" => "darkgray";
            "position" => "absolute";
        }],
        view_car(&model.car)
    ]
}

fn view_car(car: &Car) -> Node<Msg> {
    div![
        // car container
        style! {
            "width" => unit!(car.width, px);
            "height" => unit!(car.height, px);
            "top" => unit!(car.y, px);
            "left" => unit!(car.x, px);
            "position" => "absolute";
        },
        // windows
        div![style! {
            "background-color" => "rgb(255,255,255,0.5)";
            "left" => unit!(car.width * 0.25, px);
            "width" => unit!(car.width * 0.5, px);
            "height" => unit!(car.height * 0.6, px);
            "border-radius" => unit!(9999, px);
            "position" => "absolute";
        }],
        // body
        div![style! {
            "top" => unit!(car.height * 0.35, px);
            "background-color" => car.color;
            "width" => unit!(car.width, px);
            "height" => unit!(car.height * 0.5, px);
            "border-radius" => unit!(9999, px);
            "position" => "absolute";
        }],
        view_wheel(car.width * 0.15, car),
        view_wheel(car.width * 0.6, car)
    ]
}

fn view_wheel(wheel_x: f64, car: &Car) -> Node<Msg> {
    let wheel_radius = car.height * 0.4;
    div![style! {
        "top" => unit!(car.height * 0.55, px);
        "left" => unit!(wheel_x, px);
        "background-color" => "black";
        "width" => unit!(wheel_radius, px);
        "height" => unit!(wheel_radius, px);
        "border-radius" => unit!(9999, px);
        "position" => "absolute";
    }]
}

// Init

fn init(_: Url, orders: &mut impl Orders<Msg>) -> Model {
    orders
        .send_msg(Msg::SetViewportWidth)
        .send_msg(Msg::NextAnimationStep);
    Model::default()
}

#[wasm_bindgen(start)]
pub fn render() {
    seed::App::build(init, update, view)
        .window_events(|_| vec![simple_ev(Ev::Resize, Msg::SetViewportWidth)])
        .finish()
        .run();
}
