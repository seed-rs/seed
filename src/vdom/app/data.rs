use crate::vdom::{
    alias::*,
    render_timestamp_delta::{RenderTimestamp, RenderTimestampDelta},
};
use crate::{
    dom_types::node::el::El,
    events::{Event, Listener},
    util,
};
use std::cell::{Cell, RefCell};
use wasm_bindgen::closure::Closure;

// TODO: Examine what needs to be ref cells, rcs etc

type StoredPopstate = RefCell<Option<Closure<dyn FnMut(Event)>>>;

/// Used as part of an interior-mutability pattern, ie Rc<RefCell<>>
#[allow(clippy::type_complexity, clippy::module_name_repetitions)]
pub struct AppData<Ms: 'static, Mdl> {
    // Model is in a RefCell here so we can modify it in self.update().
    pub model: RefCell<Option<Mdl>>,
    pub main_el_vdom: RefCell<Option<El<Ms>>>,
    pub popstate_closure: StoredPopstate,
    pub hashchange_closure: StoredPopstate,
    pub routes: RefCell<Option<RoutesFn<Ms>>>,
    pub window_listeners: RefCell<Vec<Listener<Ms>>>,
    pub msg_listeners: RefCell<MsgListeners<Ms>>,
    pub scheduled_render_handle: RefCell<Option<util::RequestAnimationFrameHandle>>,
    pub after_next_render_callbacks:
        RefCell<Vec<Box<dyn FnOnce(Option<RenderTimestampDelta>) -> Ms>>>,
    pub render_timestamp: Cell<Option<RenderTimestamp>>,
}
