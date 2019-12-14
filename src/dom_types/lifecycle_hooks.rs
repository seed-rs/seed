// @TODO remove

use crate::{dom_types::MessageMapper, events};
use std::fmt;

type _HookFn = Box<dyn FnMut(&web_sys::Node)>; // todo

pub struct LifecycleHooks<Ms> {
    pub did_mount: Option<DidMount<Ms>>,
    pub did_update: Option<DidUpdate<Ms>>,
    pub will_unmount: Option<WillUnmount<Ms>>,
}

impl<Ms> LifecycleHooks<Ms> {
    pub const fn new() -> Self {
        Self {
            did_mount: None,
            did_update: None,
            will_unmount: None,
        }
    }
}

impl<Ms> fmt::Debug for LifecycleHooks<Ms> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "LifecycleHooks {{ did_mount:{:?}, did_update:{:?}, will_unmount:{} }}",
            events::fmt_hook_fn(&self.did_mount),
            events::fmt_hook_fn(&self.did_update),
            events::fmt_hook_fn(&self.will_unmount)
        )
    }
}

impl<Ms: 'static, OtherMs: 'static> MessageMapper<Ms, OtherMs> for LifecycleHooks<Ms> {
    type SelfWithOtherMs = LifecycleHooks<OtherMs>;
    fn map_msg(self, f: impl FnOnce(Ms) -> OtherMs + 'static + Clone) -> Self::SelfWithOtherMs {
        LifecycleHooks {
            did_mount: self.did_mount.map(|d| DidMount {
                actions: d.actions,
                message: d.message.map(f.clone()),
            }),
            did_update: self.did_update.map(|d| DidUpdate {
                actions: d.actions,
                message: d.message.map(f.clone()),
            }),
            will_unmount: self.will_unmount.map(|d| WillUnmount {
                actions: d.actions,
                message: d.message.map(f),
            }),
        }
    }
}

pub struct DidMount<Ms> {
    pub actions: Box<dyn FnMut(&web_sys::Node)>,
    pub message: Option<Ms>,
}

pub struct DidUpdate<Ms> {
    pub actions: Box<dyn FnMut(&web_sys::Node)>,
    pub message: Option<Ms>,
}

pub struct WillUnmount<Ms> {
    pub actions: Box<dyn FnMut(&web_sys::Node)>,
    pub message: Option<Ms>,
}

/// A constructor for `DidMount`, to be used in the API
pub fn did_mount<Ms>(mut actions: impl FnMut(&web_sys::Node) + 'static) -> DidMount<Ms> {
    let closure = move |el: &web_sys::Node| actions(el);
    DidMount {
        actions: Box::new(closure),
        message: None,
    }
}

/// A constructor for `DidUpdate`, to be used in the API
pub fn did_update<Ms>(mut actions: impl FnMut(&web_sys::Node) + 'static) -> DidUpdate<Ms> {
    let closure = move |el: &web_sys::Node| actions(el);
    DidUpdate {
        actions: Box::new(closure),
        message: None,
    }
}

/// A constructor for `WillUnmount`, to be used in the API
pub fn will_unmount<Ms>(mut actions: impl FnMut(&web_sys::Node) + 'static) -> WillUnmount<Ms> {
    let closure = move |el: &web_sys::Node| actions(el);
    WillUnmount {
        actions: Box::new(closure),
        message: None,
    }
}
