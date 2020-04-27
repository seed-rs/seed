use crate::components::*;
use crate::config::get_media_href;
use crate::geometry::*;
use crate::hud::Hud;
use crate::input;
use crate::renderer::SceneRenderer;
use crate::systems::{self, TICK};
use crate::world::init_world;

use awsm_web::loaders::fetch;
use awsm_web::webgl::{
    get_texture_size, get_webgl_context_1, ResizeStrategy, WebGl1Renderer, WebGlContextOptions,
    WebGlTextureSource,
};
use awsm_web::window::get_window_size;
use gloo_events::EventListener;
use shipyard::*;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::future_to_promise;
use web_sys::{HtmlCanvasElement, HtmlElement};

pub fn start() -> Result<js_sys::Promise, JsValue> {
    let window = web_sys::window().ok_or("should have a Window")?;
    let document = window.document().ok_or("should have a Document")?;
    let body = document.body().ok_or("should have a Body")?;

    let loading: HtmlElement = document.create_element("div")?.dyn_into()?;
    loading.set_class_name("loading");
    loading.set_text_content(Some("Loading..."));
    body.append_child(&loading)?;

    let future = async move {
        let img = fetch::image(&get_media_href("bunny.png")).await?;
        let vertex = fetch::text(&get_media_href("vertex.glsl")).await?;
        let fragment = fetch::text(&get_media_href("fragment.glsl")).await?;

        let (stage_width, stage_height) = get_window_size(&window).unwrap();
        let (img_width, img_height, _) = get_texture_size(&WebGlTextureSource::ImageElement(&img));

        body.remove_child(&loading)?;
        let canvas: HtmlCanvasElement = document.create_element("canvas")?.dyn_into()?;
        body.append_child(&canvas)?;

        let hud = Hud::new(&document, &body)?;

        //not using any webgl2 features so might as well stick with v1
        let gl = get_webgl_context_1(
            &canvas,
            Some(&WebGlContextOptions {
                alpha: false,
                ..Default::default()
            }),
        )?;

        let renderer = WebGl1Renderer::new(gl)?;

        let scene_renderer = SceneRenderer::new(renderer, &vertex, &fragment, &img)?;

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
        let on_resize = {
            let window = window.clone();
            let world = Rc::clone(&world);
            move |_: &web_sys::Event| {
                let (width, height) = get_window_size(&window).unwrap();
                world
                    .borrow::<NonSendSync<UniqueViewMut<SceneRenderer>>>()
                    .renderer
                    .resize(ResizeStrategy::All(width, height));
                let mut stage_area = world.borrow::<UniqueViewMut<StageArea>>();
                stage_area.0.width = width;
                stage_area.0.height = height;
            }
        };

        on_resize(&web_sys::Event::new("").unwrap());

        EventListener::new(&window, "resize", on_resize).forget();

        //start the game loop!
        let tick = Raf::new({
            let world = Rc::clone(&world);

            move |timestamp| {
                world.borrow::<UniqueViewMut<Timestamp>>().0 = timestamp;
                world.run_workload(TICK);
            }
        });

        input::start(world, &canvas);

        Box::leak(Box::new(tick));
        Ok(JsValue::null())
    };

    Ok(future_to_promise(future))
}

/// Until Raf is availble in gloo...
struct Raf {
    state: Rc<RefCell<Option<RafState>>>,
}

struct RafState {
    id: i32,
    closure: Closure<dyn FnMut(f64)>,
}

impl Raf {
    fn new<F>(mut callback: F) -> Self
    where
        F: FnMut(f64) + 'static,
    {
        let state: Rc<RefCell<Option<RafState>>> = Rc::new(RefCell::new(None));

        fn schedule(callback: &Closure<dyn FnMut(f64)>) -> i32 {
            web_sys::window()
                .unwrap_throw()
                .request_animation_frame(callback.as_ref().unchecked_ref())
                .unwrap_throw()
        }

        let closure = {
            let state = state.clone();

            Closure::wrap(Box::new(move |time| {
                {
                    let mut state = state.borrow_mut();
                    let state = state.as_mut().unwrap_throw();
                    state.id = schedule(&state.closure);
                }

                callback(time);
            }) as Box<dyn FnMut(f64)>)
        };

        *state.borrow_mut() = Some(RafState {
            id: schedule(&closure),
            closure,
        });

        Self { state }
    }
}

impl Drop for Raf {
    fn drop(&mut self) {
        // The take is necessary in order to prevent an Rc leak
        let state = self.state.borrow_mut().take().unwrap_throw();

        web_sys::window()
            .unwrap_throw()
            .cancel_animation_frame(state.id)
            .unwrap_throw();
    }
}
