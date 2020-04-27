// [Original code](https://github.com/leudz/shipyard/tree/23f2998296f690aee78972f9cfe06dfd73b7971c/demo)
// triggers many Clippy lints:
#![allow(
    clippy::cast_precision_loss,
    clippy::default_trait_access,
    clippy::cast_sign_loss,
    clippy::cast_possible_truncation,
    clippy::needless_pass_by_value,
    clippy::cast_lossless,
    clippy::too_many_arguments,
    clippy::needless_borrow,
    clippy::missing_const_for_fn
)]

use awsm_web::errors::Error;
use awsm_web::loaders;
use awsm_web::webgl::{
    get_texture_size, get_webgl_context_1, ResizeStrategy, WebGl1Renderer, WebGlContextOptions,
    WebGlTextureSource,
};
use awsm_web::window::get_window_size;
use seed::web_sys::{HtmlCanvasElement, HtmlImageElement};
use seed::{prelude::*, *};
use shipyard::{NonSendSync, UniqueView, UniqueViewMut, World};

mod components;
mod config;
mod fps_counter;
mod geometry;
mod hud;
mod init_world;
mod scene_renderer;
mod systems;

use components::{Controller, StageArea, Timestamp};
use config::get_media_href;
use geometry::Area;
use hud::Hud;
use init_world::init_world;
use scene_renderer::SceneRenderer;
use systems::TICK;

// ------ ------
//     Init
// ------ ------

fn init(_: Url, orders: &mut impl Orders<Msg>) -> Model {
    orders
        .stream(streams::window_event(Ev::Resize, |_| Msg::OnResize))
        .perform_cmd(async {
            Msg::ResourcesLoaded(
                async {
                    Ok(Resources {
                        img: loaders::fetch::image(&get_media_href("bunny.png")).await?,
                        vertex: loaders::fetch::text(&get_media_href("vertex.glsl")).await?,
                        fragment: loaders::fetch::text(&get_media_href("fragment.glsl")).await?,
                    })
                }
                .await,
            )
        });

    let (stage_width, stage_height) = get_window_size(&window()).unwrap();

    Model {
        resources: None,
        canvas: ElRef::default(),
        stage_width,
        stage_height,
        world: None,
        num_bunnies: 0,
        fps: 0,
    }
}

// ------ ------
//     Model
// ------ ------

struct Model {
    resources: Option<Resources>,
    canvas: ElRef<HtmlCanvasElement>,
    stage_width: u32,
    stage_height: u32,
    world: Option<World>,
    num_bunnies: usize,
    fps: u32,
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
    OnResize,
    OnTick(RenderInfo),
    PointerDown,
    PointerUp,
}

fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::ResourcesLoaded(Ok(resources)) => {
            log!("resources loaded");
            model.resources = Some(resources);
            orders.after_next_render(|_| Msg::CanvasReady);
        }
        Msg::ResourcesLoaded(Err(error)) => {
            log!("resources loading failed:", error);
        }
        Msg::CanvasReady => {
            log!("canvas ready");
            model.world = Some(create_world(model));
            log!("world created");
            log!("starting game loop...");
            orders.after_next_render(Msg::OnTick);
        }
        Msg::OnResize => {
            let (stage_width, stage_height) = get_window_size(&window()).unwrap();
            model.stage_width = stage_width;
            model.stage_height = stage_height;

            if let Some(world) = &model.world {
                world
                    .borrow::<NonSendSync<UniqueViewMut<SceneRenderer>>>()
                    .renderer
                    .resize(ResizeStrategy::All(stage_width, stage_height));
                let mut stage_area = world.borrow::<UniqueViewMut<StageArea>>();
                stage_area.0.width = stage_width;
                stage_area.0.height = stage_height;
            }
        }
        Msg::OnTick(render_info) => {
            if let Some(world) = &model.world {
                world.borrow::<UniqueViewMut<Timestamp>>().0 = render_info.timestamp;
                world.run_workload(TICK);
                let hud = world.borrow::<UniqueView<Hud>>();
                model.fps = hud.fps();
                model.num_bunnies = hud.num_bunnies();
            }
            orders.after_next_render(Msg::OnTick);
        }
        Msg::PointerDown => {
            if let Some(world) = &model.world {
                *world.borrow::<UniqueViewMut<Controller>>() = Controller::Adding;
            }
        }
        Msg::PointerUp => {
            if let Some(world) = &model.world {
                *world.borrow::<UniqueViewMut<Controller>>() = Controller::Waiting;
            }
        }
    }
}

fn create_world(model: &mut Model) -> World {
    let res = model
        .resources
        .as_ref()
        .expect("get resources to start world");
    let canvas = model.canvas.get().expect("get canvas element");

    let (img_width, img_height, _) = get_texture_size(&WebGlTextureSource::ImageElement(&res.img));

    let gl = get_webgl_context_1(
        &canvas,
        Some(&WebGlContextOptions {
            alpha: false,
            ..Default::default()
        }),
    )
    .expect("get_webgl_context_1");

    let renderer = WebGl1Renderer::new(gl).expect("create renderer");

    let scene_renderer = SceneRenderer::new(renderer, &res.vertex, &res.fragment, &res.img)
        .expect("create scene renderer");

    let world = init_world(
        Area {
            width: img_width,
            height: img_height,
        },
        Area {
            width: model.stage_width,
            height: model.stage_height,
        },
        Hud::default(),
        scene_renderer,
    );

    systems::register_workloads(&world);

    world
}

// ------ ------
//     View
// ------ ------

fn view(model: &Model) -> Vec<Node<Msg>> {
    if model.resources.is_none() {
        vec![view_loading()]
    } else {
        vec![
            view_canvas(&model.canvas, model.stage_width, model.stage_height),
            view_hud(model.num_bunnies, model.fps),
        ]
    }
}

fn view_loading() -> Node<Msg> {
    div![C!["loading"], "Loading..."]
}

fn view_canvas(
    canvas: &ElRef<HtmlCanvasElement>,
    stage_width: u32,
    stage_height: u32,
) -> Node<Msg> {
    canvas![
        el_ref(canvas),
        attrs! {
            At::Width => stage_width,
            At::Height => stage_height,
        },
        ev(Ev::PointerDown, |_| Msg::PointerDown),
        ev(Ev::PointerUp, |_| Msg::PointerUp),
    ]
}

fn view_hud(num_bunnies: usize, fps: u32) -> Node<Msg> {
    div![
        C!["info"],
        div![C!["info-num__bunnies"], "bunnies: ", num_bunnies,],
        div![C!["info-fps"], "fps: ", fps,],
    ]
}

// ------ ------
//     Start
// ------ ------

#[wasm_bindgen(start)]
pub fn start() {
    App::start("app", init, update, view);
}
