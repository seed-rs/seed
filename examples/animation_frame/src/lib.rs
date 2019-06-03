#[macro_use]
extern crate seed;
#[macro_use]
extern crate serde_derive;
use seed::prelude::*;
use rand::prelude::*;

// Model

type CarColor = String;

#[derive(Debug)]
struct Car {
    x: i32,
    y: i32,
    speed: f64,
    color: CarColor,
    width: i32,
    height: i32,
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
        let car_width = 120;
        Self {
            x: -car_width,
            y: 100,
            speed: Self::generate_speed(),
            color: Self::generate_color(),
            width: car_width,
            height: 60,
        }
    }
}

#[derive(Default)]
struct Model {
    request_animation_frame_handle: Option<RequestAnimationFrameHandle>,
    previous_time: Option<RequestAnimationFrameTime>,
    viewport_width: i32,
    car: Car
}

// Update

#[derive(Clone, Serialize, Deserialize)]
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
            model.viewport_width = seed::body().client_width();
            orders.skip();
        }
        Msg::NextAnimationStep =>  {
            let cb = Closure::wrap(Box::new(|time| {
                seed::update(Msg::OnAnimationFrame(time));
            }) as Box<FnMut(RequestAnimationFrameTime)>);

            model.request_animation_frame_handle = Some(request_animation_frame(cb));
            orders.skip();
        },
        Msg::OnAnimationFrame(time) => {
            let delta =  match model.previous_time{
                Some(previous_time) => time - previous_time,
                None => 0.0
            };
            model.previous_time = Some(time);

            if delta > 0.0 {
                // move car at least 1px to the right
                model.car.x += i32::max(1,(delta / 1000.0 * model.car.speed) as i32);

                // we don't see car anymore => back to start + generate new color and speed
                if model.car.x > model.viewport_width {
                    model.car = Default::default();
                }
            }
            orders.send_msg(Msg::NextAnimationStep);
        }
    }
}

// View

fn view(model: &Model) -> El<Msg> {
    // scene container + sky
    div![
        style!{
          "overflow" => "hidden";
          "width" => "100%";
          "position" => "relative";
          "height" => 170;
          "background-color" => "deepskyblue";
        },
        // road
        div![
            style!{
                "width" => "100%";
                "height" => 20;
                "bottom" => 0;
                "background-color" => "darkgray";
                "position" => "absolute";
            }
        ],
        view_car(&model.car)
    ]
}

fn view_car(car: &Car) -> El<Msg> {
    div![
        // car container
        // @TODO [BUG?]: Check that Seed doesn't change order of properties between renders
        style!{
            "width" => car.width;
            "height" => car.height;
            "top" => car.y;
            "left" => car.x;
            "position" => "absolute";
        },
        // windows
        div![
            style!{
                "background-color" => "rgb(255,255,255,0.5)";
                "left" => car.width as f32 * 0.25;
                "width" => car.width as f32 * 0.50;
                "height" => car.height as f32 * 0.60;
                "border-radius" => 9999;
                "position" => "absolute";
            }
        ],
        // body
        div![
            style!{
                "top" => car.height as f32 * 0.35;
                "background-color" => car.color;
                "width" => car.width;
                "height" => car.height as f32 * 0.50;
                "border-radius" => 9999;
                "position" => "absolute";
            }
        ],
        view_wheel((car.width as f32 * 0.15) as i32, car),
        view_wheel((car.width as f32 * 0.60) as i32, car)
    ]
}

fn view_wheel(wheel_x: i32, car: &Car) -> El<Msg> {
    let wheel_radius = car.height as f32 * 0.40;
    div![
        style!{
            "top" => car.height as f32 * 0.55;
            "left" => wheel_x;
            "background-color" => "black";
            "width" => wheel_radius;
            "height" => wheel_radius;
            "border-radius" => 9999;
            "position" => "absolute";
        }
    ]
}

#[wasm_bindgen]
pub fn render() {
    let app = seed::App::build(Model::default(), update, view)
        .window_events(|_| {
            vec![
                // we want to use `seed::update(...)`
                trigger_update_handler(),
                simple_ev(Ev::Resize, Msg::SetViewportWidth)
            ]
        })
        .finish()
        .run();

    app.update(Msg::Init);
}
