use super::types::MsgListeners;
use super::{RenderInfo, SubManager};
use crate::browser::util;
use crate::virtual_dom::{El, EventHandlerManager};
use std::cell::{Cell, RefCell};
use wasm_bindgen::closure::Closure;

type StoredPopstate = RefCell<Option<Closure<dyn FnMut(web_sys::Event)>>>;

// Used as part of an interior-mutability pattern, ie Rc<RefCell<>>
#[allow(clippy::type_complexity)]
pub(crate) struct AppData<Ms: 'static, Mdl> {
    // Model is in a RefCell here so we can modify it in self.update().
    pub model: RefCell<Option<Mdl>>,
    // @TODO: Look into removing/renaming main_el_vdom
    pub(crate) main_el_vdom: RefCell<Option<El<Ms>>>,
    pub popstate_closure: StoredPopstate,
    pub hashchange_closure: StoredPopstate,
    pub window_event_handler_manager: RefCell<EventHandlerManager<Ms>>,
    pub sub_manager: RefCell<SubManager<Ms>>,
    pub msg_listeners: RefCell<MsgListeners<Ms>>,
    pub scheduled_render_handle: RefCell<Option<util::RequestAnimationFrameHandle>>,
    pub after_next_render_callbacks: RefCell<Vec<Box<dyn FnOnce(RenderInfo) -> Option<Ms>>>>,
    pub render_info: Cell<Option<RenderInfo>>,
}
