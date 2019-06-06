#[macro_use]
extern crate seed;
#[macro_use]
extern crate serde_derive;
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
        thread_rng().gen_range(400.0, 800.0)
    }

    fn generate_color() -> CarColor {
        let hue = thread_rng().gen_range(0, 360);
        format!("hsl({},100%,50%)", hue)
    }
}

impl Default for Car {
    fn default() -> Self {
        let car_width = 120.0;
        Self {
            x: -car_width,
            y: 100.0,
            speed: Self::generate_speed(),
            color: Self::generate_color(),
            width: car_width,
            height: 60.0,
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

#[derive(Clone, Copy, Serialize, Deserialize)]
enum Msg {
    Init,
    SetViewportWidth,
    NextAnimationStep,
    OnAnimationFrame(RequestAnimationFrameTime),
}

fn update(msg: Msg, model: &mut Model, orders: &mut Orders<Msg>) {
    match msg {
        Msg::Init => {
            orders
                .send_msg(Msg::SetViewportWidth)
                .send_msg(Msg::NextAnimationStep)
                .skip();
        }
        Msg::SetViewportWidth => {
            model.viewport_width = f64::from(seed::body().client_width());
            orders.skip();
        }
        Msg::NextAnimationStep => {
            let cb = Closure::wrap(Box::new(|time| {
                seed::update(Msg::OnAnimationFrame(time));
            }) as Box<FnMut(RequestAnimationFrameTime)>);

            model.request_animation_frame_handle = Some(request_animation_frame(cb));
            orders.skip();
        }
        Msg::OnAnimationFrame(time) => {
            let delta = match model.previous_time {
                Some(previous_time) => time - previous_time,
                None => 0.0,
            };
            model.previous_time = Some(time);

            if delta > 0.0 {
                // move car at least 1px to the right
                model.car.x += f64::max(1.0, delta / 1000.0 * model.car.speed);

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

fn px(number: impl ToString + Copy) -> String {
    let mut value = number.to_string();
    value.push_str("px");
    value
}

fn view(model: &Model) -> El<Msg> {
    // scene container + sky
    div![
        style! {
          "overflow" => "hidden";
          "width" => "100%";
          "position" => "relative";
          "height" => "170px";
          "background-color" => "deepskyblue";
        },
        // road
        div![style! {
            "width" => "100%";
            "height" => "20px";
            "bottom" => "0";
            "background-color" => "darkgray";
            "position" => "absolute";
        }],
        view_car(&model.car)
    ]
}

fn view_car(car: &Car) -> El<Msg> {
    div![
        // car container
        style! {
            "width" => px(car.width);
            "height" => px(car.height);
            "top" => px(car.y);
            "left" => px(car.x);
            "position" => "absolute";
        },
        // windows
        div![style! {
            "background-color" => "rgb(255,255,255,0.5)";
            "left" => px(car.width * 0.25);
            "width" => px(car.width * 0.50);
            "height" => px(car.height * 0.60);
            "border-radius" => "9999px";
            "position" => "absolute";
        }],
        // body
        div![style! {
            "top" => px(car.height * 0.35);
            "background-color" => car.color;
            "width" => px(car.width);
            "height" => px(car.height * 0.50);
            "border-radius" => "9999px";
            "position" => "absolute";
        }],
        view_wheel(car.width * 0.15, car),
        view_wheel(car.width * 0.60, car)
    ]
}

fn view_wheel(wheel_x: f64, car: &Car) -> El<Msg> {
    let wheel_radius = car.height * 0.40;
    div![style! {
        "top" => px(car.height * 0.55);
        "left" => px(wheel_x);
        "background-color" => "black";
        "width" => px(wheel_radius);
        "height" => px(wheel_radius);
        "border-radius" => "9999px";
        "position" => "absolute";
    }]
}

#[wasm_bindgen]
pub fn render() {
    let app = seed::App::build(Model::default(), update, view)
        .window_events(|_| {
            vec![
                // we want to use `seed::update(...)`
                trigger_update_handler(),
                simple_ev(Ev::Resize, Msg::SetViewportWidth),
            ]
        })
        .finish()
        .run();

    app.update(Msg::Init);
}
