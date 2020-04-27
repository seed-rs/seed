use seed::{prelude::*, *};
use std::rc::Rc;
use web_sys::{HtmlImageElement, HtmlCanvasElement};
use wasm_bindgen::JsCast;
use awsm_web::loaders;
use awsm_web::errors::Error;
use awsm_web::window::get_window_size;
use awsm_web::webgl::{
    get_texture_size, get_webgl_context_1, ResizeStrategy, WebGl1Renderer, WebGlContextOptions,
    WebGlTextureSource,
};

mod components;
mod config;
mod fps;
mod geometry;
mod hud;
mod init;
mod input;
mod renderer;
mod systems;
mod world;

use config::get_media_href;
use geometry::Area;
use hud::Hud;
use renderer::SceneRenderer;
use world::init_world;

// ------ ------
//     Init
// ------ ------

fn init(_: Url, orders: &mut impl Orders<Msg>) -> Model {
    orders.perform_cmd(async {
        Msg::ResourcesLoaded(async { Ok(Resources {
            img: loaders::fetch::image(&get_media_href("bunny.png")).await?,
            vertex: loaders::fetch::text(&get_media_href("vertex.glsl")).await?,
            fragment: loaders::fetch::text(&get_media_href("fragment.glsl")).await?,
        })}.await)
    });
    Model::default()
}

// ------ ------
//     Model
// ------ ------

#[derive(Default)]
struct Model {
    resources: Option<Resources>,
    canvas: ElRef<HtmlCanvasElement>,
}

struct Resources {
    img: HtmlImageElement,
    vertex: String,
    fragment: String,
}

// ------ ------
//    Update
// ------ ------

enum Msg {
    ResourcesLoaded(Result<Resources, Error>),
    CanvasReady,
}

fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::ResourcesLoaded(Ok(resources)) => {
            log!("resources loaded");
            model.resources = Some(resources);
            orders.after_next_render(|_| Msg::CanvasReady);
        },
        Msg::ResourcesLoaded(Err(error)) => {
            log!("Resources loading failed:", error);
        }
        Msg::CanvasReady => {
            log!("canvas ready");
            start_world(model)
        }
    }
}

fn start_world(model: &mut Model) {
    log!("starting world...");

    let res = model.resources.as_ref().expect("get resources to start world");
    let canvas = model.canvas.get().expect("get canvas element");

    let (stage_width, stage_height) = get_window_size(&window()).unwrap();
    log!("stage size:", stage_width, stage_height);
    let (img_width, img_height, _) = get_texture_size(&WebGlTextureSource::ImageElement(&res.img));
    log!("img size:", img_width, img_height);

    // @TODO rewrite
    let hud = Hud::new(&document(), &body()).expect("create HUD");

    let gl = get_webgl_context_1(
        &canvas,
        Some(&WebGlContextOptions {
            alpha: false,
            ..Default::default()
        }),
    ).expect("get_webgl_context_1");

    let renderer = WebGl1Renderer::new(gl).expect("create renderer");

    let scene_renderer = SceneRenderer::new(renderer, &res.vertex, &res.fragment, &res.img)
        .expect("create scene renderer");

    let world = Rc::new(init_world(
        Area {
            width: img_width,
            height: img_height,
        },
        Area {
            width: stage_width,
            height: stage_height,
        },
        hud,
        scene_renderer,
    ));

    systems::register_workloads(&world);

    // @TODO on_resize
    // @TODO input_start
}

// ------ ------
//     View
// ------ ------

fn view(model: &Model) -> Node<Msg> {
    if model.resources.is_none() {
        div![
            C!["loading"],
            "Loading..."
        ]
    } else {
        canvas![
            el_ref(&model.canvas),
        ]
    }
}

// ------ ------
//     Start
// ------ ------

#[wasm_bindgen(start)]
pub fn start() {
    App::start("app", init, update, view);
}
