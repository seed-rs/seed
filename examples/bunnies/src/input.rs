use crate::components::*;

use gloo_events::EventListener;
use shipyard::*;
use std::rc::Rc;
use web_sys::HtmlCanvasElement;

pub fn start(world: Rc<World>, canvas: &HtmlCanvasElement) {
    EventListener::new(canvas, "pointerdown", {
        let world = Rc::clone(&world);
        move |_| {
            *world.borrow::<UniqueViewMut<Controller>>() = Controller::Adding;
        }
    })
    .forget();

    EventListener::new(canvas, "pointerup", move |_| {
        *world.borrow::<UniqueViewMut<Controller>>() = Controller::Waiting;
    })
    .forget();
}
