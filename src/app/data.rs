use super::{RenderInfo, SubManager};
use crate::browser::util;
use crate::virtual_dom::El;
use std::cell::{Cell, RefCell};
use wasm_bindgen::closure::Closure;

type StoredPopstate = RefCell<Option<Closure<dyn FnMut(web_sys::Event)>>>;

#[allow(clippy::type_complexity)]
pub(crate) struct AppData<Ms: 'static, Mdl> {
    pub model: RefCell<Option<Mdl>>,
    pub(crate) root_el: RefCell<Option<El<Ms>>>,
    pub popstate_closure: StoredPopstate,
    pub sub_manager: RefCell<SubManager<Ms>>,
    pub msg_listeners: RefCell<Vec<Box<dyn Fn(&Ms)>>>,
    pub scheduled_render_handle: RefCell<Option<util::RequestAnimationFrameHandle>>,
    pub after_next_render_callbacks: RefCell<Vec<Box<dyn FnOnce(RenderInfo) -> Option<Ms>>>>,
    pub render_info: Cell<Option<RenderInfo>>,
}
