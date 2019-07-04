//! This module contains code related to patching the VDOM. It can be considered
//! a subset of the `vdom` module.

use crate::{dom_types::El, websys_bridge};

/// Remove a node from the vdom and web_sys DOM.
pub(crate) fn remove_node<Ms>(node: &web_sys::Node, parent: &web_sys::Node, el_vdom: &mut El<Ms>) {
    websys_bridge::remove_node(node, parent);

    if let Some(unmount_actions) = &mut el_vdom.hooks.will_unmount {
        (unmount_actions.actions)(node);
        //                if let Some(message) = unmount_actions.message.clone() {
        //                    app.update(message);
        //                }
    }
}
