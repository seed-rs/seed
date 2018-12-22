(function() {
    var wasm;
    const __exports = {};
    /**
    * @returns {void}
    */
    __exports.render = function() {
        return wasm.render();
    };

    let cachedTextDecoder = new TextDecoder('utf-8');

    let cachegetUint8Memory = null;
    function getUint8Memory() {
        if (cachegetUint8Memory === null || cachegetUint8Memory.buffer !== wasm.memory.buffer) {
            cachegetUint8Memory = new Uint8Array(wasm.memory.buffer);
        }
        return cachegetUint8Memory;
    }

    function getStringFromWasm(ptr, len) {
        return cachedTextDecoder.decode(getUint8Memory().subarray(ptr, ptr + len));
    }

    __exports.__wbg_error_cc95a3d302735ca3 = function(arg0, arg1) {
        let varg0 = getStringFromWasm(arg0, arg1);

        varg0 = varg0.slice();
        wasm.__wbindgen_free(arg0, arg1 * 1);

        console.error(varg0);
    };

    const heap = new Array(32);

    heap.fill(undefined);

    heap.push(undefined, null, true, false);

function getObject(idx) { return heap[idx]; }

let heap_next = heap.length;

function addHeapObject(obj) {
    if (heap_next === heap.length) heap.push(heap.length + 1);
    const idx = heap_next;
    heap_next = heap[idx];

    heap[idx] = obj;
    return idx;
}

const __widl_f_create_element_Document_target = typeof Document === 'undefined' ? null : Document.prototype.createElement || function() {
    throw new Error(`wasm-bindgen: Document.createElement does not exist`);
};

let cachegetUint32Memory = null;
function getUint32Memory() {
    if (cachegetUint32Memory === null || cachegetUint32Memory.buffer !== wasm.memory.buffer) {
        cachegetUint32Memory = new Uint32Array(wasm.memory.buffer);
    }
    return cachegetUint32Memory;
}

__exports.__widl_f_create_element_Document = function(arg0, arg1, arg2, exnptr) {
    let varg1 = getStringFromWasm(arg1, arg2);
    try {
        return addHeapObject(__widl_f_create_element_Document_target.call(getObject(arg0), varg1));
    } catch (e) {
        const view = getUint32Memory();
        view[exnptr / 4] = 1;
        view[exnptr / 4 + 1] = addHeapObject(e);

    }
};

const __widl_f_create_element_ns_Document_target = typeof Document === 'undefined' ? null : Document.prototype.createElementNS || function() {
    throw new Error(`wasm-bindgen: Document.createElementNS does not exist`);
};

__exports.__widl_f_create_element_ns_Document = function(arg0, arg1, arg2, arg3, arg4, exnptr) {
    let varg1 = arg1 == 0 ? undefined : getStringFromWasm(arg1, arg2);
    let varg3 = getStringFromWasm(arg3, arg4);
    try {
        return addHeapObject(__widl_f_create_element_ns_Document_target.call(getObject(arg0), varg1, varg3));
    } catch (e) {
        const view = getUint32Memory();
        view[exnptr / 4] = 1;
        view[exnptr / 4 + 1] = addHeapObject(e);

    }
};

const __widl_f_create_text_node_Document_target = typeof Document === 'undefined' ? null : Document.prototype.createTextNode || function() {
    throw new Error(`wasm-bindgen: Document.createTextNode does not exist`);
};

__exports.__widl_f_create_text_node_Document = function(arg0, arg1, arg2) {
    let varg1 = getStringFromWasm(arg1, arg2);
    return addHeapObject(__widl_f_create_text_node_Document_target.call(getObject(arg0), varg1));
};

function isLikeNone(x) {
    return x === undefined || x === null;
}

const __widl_f_get_element_by_id_Document_target = typeof Document === 'undefined' ? null : Document.prototype.getElementById || function() {
    throw new Error(`wasm-bindgen: Document.getElementById does not exist`);
};

__exports.__widl_f_get_element_by_id_Document = function(arg0, arg1, arg2) {
    let varg1 = getStringFromWasm(arg1, arg2);

    const val = __widl_f_get_element_by_id_Document_target.call(getObject(arg0), varg1);
    return isLikeNone(val) ? 0 : addHeapObject(val);

};

const __widl_f_remove_attribute_Element_target = typeof Element === 'undefined' ? null : Element.prototype.removeAttribute || function() {
    throw new Error(`wasm-bindgen: Element.removeAttribute does not exist`);
};

__exports.__widl_f_remove_attribute_Element = function(arg0, arg1, arg2, exnptr) {
    let varg1 = getStringFromWasm(arg1, arg2);
    try {
        __widl_f_remove_attribute_Element_target.call(getObject(arg0), varg1);
    } catch (e) {
        const view = getUint32Memory();
        view[exnptr / 4] = 1;
        view[exnptr / 4 + 1] = addHeapObject(e);

    }
};

const __widl_f_set_attribute_Element_target = typeof Element === 'undefined' ? null : Element.prototype.setAttribute || function() {
    throw new Error(`wasm-bindgen: Element.setAttribute does not exist`);
};

__exports.__widl_f_set_attribute_Element = function(arg0, arg1, arg2, arg3, arg4, exnptr) {
    let varg1 = getStringFromWasm(arg1, arg2);
    let varg3 = getStringFromWasm(arg3, arg4);
    try {
        __widl_f_set_attribute_Element_target.call(getObject(arg0), varg1, varg3);
    } catch (e) {
        const view = getUint32Memory();
        view[exnptr / 4] = 1;
        view[exnptr / 4 + 1] = addHeapObject(e);

    }
};

function GetOwnOrInheritedPropertyDescriptor(obj, id) {
    while (obj) {
        let desc = Object.getOwnPropertyDescriptor(obj, id);
        if (desc) return desc;
        obj = Object.getPrototypeOf(obj);
    }
return {}
}

const __widl_f_set_inner_html_Element_target = GetOwnOrInheritedPropertyDescriptor(typeof Element === 'undefined' ? null : Element.prototype, 'innerHTML').set || function() {
    throw new Error(`wasm-bindgen: Element.innerHTML does not exist`);
};

__exports.__widl_f_set_inner_html_Element = function(arg0, arg1, arg2) {
    let varg1 = getStringFromWasm(arg1, arg2);
    __widl_f_set_inner_html_Element_target.call(getObject(arg0), varg1);
};

const __widl_f_prevent_default_Event_target = typeof Event === 'undefined' ? null : Event.prototype.preventDefault || function() {
    throw new Error(`wasm-bindgen: Event.preventDefault does not exist`);
};

__exports.__widl_f_prevent_default_Event = function(arg0) {
    __widl_f_prevent_default_Event_target.call(getObject(arg0));
};

const __widl_f_target_Event_target = GetOwnOrInheritedPropertyDescriptor(typeof Event === 'undefined' ? null : Event.prototype, 'target').get || function() {
    throw new Error(`wasm-bindgen: Event.target does not exist`);
};

__exports.__widl_f_target_Event = function(arg0) {

    const val = __widl_f_target_Event_target.call(getObject(arg0));
    return isLikeNone(val) ? 0 : addHeapObject(val);

};

const __widl_f_add_event_listener_with_callback_EventTarget_target = typeof EventTarget === 'undefined' ? null : EventTarget.prototype.addEventListener || function() {
    throw new Error(`wasm-bindgen: EventTarget.addEventListener does not exist`);
};

__exports.__widl_f_add_event_listener_with_callback_EventTarget = function(arg0, arg1, arg2, arg3, exnptr) {
    let varg1 = getStringFromWasm(arg1, arg2);
    try {
        __widl_f_add_event_listener_with_callback_EventTarget_target.call(getObject(arg0), varg1, getObject(arg3));
    } catch (e) {
        const view = getUint32Memory();
        view[exnptr / 4] = 1;
        view[exnptr / 4 + 1] = addHeapObject(e);

    }
};

const __widl_f_remove_event_listener_with_callback_EventTarget_target = typeof EventTarget === 'undefined' ? null : EventTarget.prototype.removeEventListener || function() {
    throw new Error(`wasm-bindgen: EventTarget.removeEventListener does not exist`);
};

__exports.__widl_f_remove_event_listener_with_callback_EventTarget = function(arg0, arg1, arg2, arg3, exnptr) {
    let varg1 = getStringFromWasm(arg1, arg2);
    try {
        __widl_f_remove_event_listener_with_callback_EventTarget_target.call(getObject(arg0), varg1, getObject(arg3));
    } catch (e) {
        const view = getUint32Memory();
        view[exnptr / 4] = 1;
        view[exnptr / 4 + 1] = addHeapObject(e);

    }
};

__exports.__widl_instanceof_HTMLButtonElement = function(idx) {
    return getObject(idx) instanceof HTMLButtonElement ? 1 : 0;
};

const __widl_f_set_autofocus_HTMLButtonElement_target = GetOwnOrInheritedPropertyDescriptor(typeof HTMLButtonElement === 'undefined' ? null : HTMLButtonElement.prototype, 'autofocus').set || function() {
    throw new Error(`wasm-bindgen: HTMLButtonElement.autofocus does not exist`);
};

__exports.__widl_f_set_autofocus_HTMLButtonElement = function(arg0, arg1) {
    __widl_f_set_autofocus_HTMLButtonElement_target.call(getObject(arg0), arg1 !== 0);
};

__exports.__widl_instanceof_HTMLInputElement = function(idx) {
    return getObject(idx) instanceof HTMLInputElement ? 1 : 0;
};

const __widl_f_set_autofocus_HTMLInputElement_target = GetOwnOrInheritedPropertyDescriptor(typeof HTMLInputElement === 'undefined' ? null : HTMLInputElement.prototype, 'autofocus').set || function() {
    throw new Error(`wasm-bindgen: HTMLInputElement.autofocus does not exist`);
};

__exports.__widl_f_set_autofocus_HTMLInputElement = function(arg0, arg1) {
    __widl_f_set_autofocus_HTMLInputElement_target.call(getObject(arg0), arg1 !== 0);
};

const __widl_f_set_checked_HTMLInputElement_target = GetOwnOrInheritedPropertyDescriptor(typeof HTMLInputElement === 'undefined' ? null : HTMLInputElement.prototype, 'checked').set || function() {
    throw new Error(`wasm-bindgen: HTMLInputElement.checked does not exist`);
};

__exports.__widl_f_set_checked_HTMLInputElement = function(arg0, arg1) {
    __widl_f_set_checked_HTMLInputElement_target.call(getObject(arg0), arg1 !== 0);
};

let cachedTextEncoder = new TextEncoder('utf-8');

let WASM_VECTOR_LEN = 0;

function passStringToWasm(arg) {

    const buf = cachedTextEncoder.encode(arg);
    const ptr = wasm.__wbindgen_malloc(buf.length);
    getUint8Memory().set(buf, ptr);
    WASM_VECTOR_LEN = buf.length;
    return ptr;
}

const __widl_f_value_HTMLInputElement_target = GetOwnOrInheritedPropertyDescriptor(typeof HTMLInputElement === 'undefined' ? null : HTMLInputElement.prototype, 'value').get || function() {
    throw new Error(`wasm-bindgen: HTMLInputElement.value does not exist`);
};

__exports.__widl_f_value_HTMLInputElement = function(ret, arg0) {

    const retptr = passStringToWasm(__widl_f_value_HTMLInputElement_target.call(getObject(arg0)));
    const retlen = WASM_VECTOR_LEN;
    const mem = getUint32Memory();
    mem[ret / 4] = retptr;
    mem[ret / 4 + 1] = retlen;

};

const __widl_f_set_value_HTMLInputElement_target = GetOwnOrInheritedPropertyDescriptor(typeof HTMLInputElement === 'undefined' ? null : HTMLInputElement.prototype, 'value').set || function() {
    throw new Error(`wasm-bindgen: HTMLInputElement.value does not exist`);
};

__exports.__widl_f_set_value_HTMLInputElement = function(arg0, arg1, arg2) {
    let varg1 = getStringFromWasm(arg1, arg2);
    __widl_f_set_value_HTMLInputElement_target.call(getObject(arg0), varg1);
};

__exports.__widl_instanceof_HTMLSelectElement = function(idx) {
    return getObject(idx) instanceof HTMLSelectElement ? 1 : 0;
};

const __widl_f_set_autofocus_HTMLSelectElement_target = GetOwnOrInheritedPropertyDescriptor(typeof HTMLSelectElement === 'undefined' ? null : HTMLSelectElement.prototype, 'autofocus').set || function() {
    throw new Error(`wasm-bindgen: HTMLSelectElement.autofocus does not exist`);
};

__exports.__widl_f_set_autofocus_HTMLSelectElement = function(arg0, arg1) {
    __widl_f_set_autofocus_HTMLSelectElement_target.call(getObject(arg0), arg1 !== 0);
};

const __widl_f_value_HTMLSelectElement_target = GetOwnOrInheritedPropertyDescriptor(typeof HTMLSelectElement === 'undefined' ? null : HTMLSelectElement.prototype, 'value').get || function() {
    throw new Error(`wasm-bindgen: HTMLSelectElement.value does not exist`);
};

__exports.__widl_f_value_HTMLSelectElement = function(ret, arg0) {

    const retptr = passStringToWasm(__widl_f_value_HTMLSelectElement_target.call(getObject(arg0)));
    const retlen = WASM_VECTOR_LEN;
    const mem = getUint32Memory();
    mem[ret / 4] = retptr;
    mem[ret / 4 + 1] = retlen;

};

__exports.__widl_instanceof_HTMLTextAreaElement = function(idx) {
    return getObject(idx) instanceof HTMLTextAreaElement ? 1 : 0;
};

const __widl_f_set_autofocus_HTMLTextAreaElement_target = GetOwnOrInheritedPropertyDescriptor(typeof HTMLTextAreaElement === 'undefined' ? null : HTMLTextAreaElement.prototype, 'autofocus').set || function() {
    throw new Error(`wasm-bindgen: HTMLTextAreaElement.autofocus does not exist`);
};

__exports.__widl_f_set_autofocus_HTMLTextAreaElement = function(arg0, arg1) {
    __widl_f_set_autofocus_HTMLTextAreaElement_target.call(getObject(arg0), arg1 !== 0);
};

const __widl_f_value_HTMLTextAreaElement_target = GetOwnOrInheritedPropertyDescriptor(typeof HTMLTextAreaElement === 'undefined' ? null : HTMLTextAreaElement.prototype, 'value').get || function() {
    throw new Error(`wasm-bindgen: HTMLTextAreaElement.value does not exist`);
};

__exports.__widl_f_value_HTMLTextAreaElement = function(ret, arg0) {

    const retptr = passStringToWasm(__widl_f_value_HTMLTextAreaElement_target.call(getObject(arg0)));
    const retlen = WASM_VECTOR_LEN;
    const mem = getUint32Memory();
    mem[ret / 4] = retptr;
    mem[ret / 4 + 1] = retlen;

};

const __widl_f_push_state_with_url_History_target = typeof History === 'undefined' ? null : History.prototype.pushState || function() {
    throw new Error(`wasm-bindgen: History.pushState does not exist`);
};

__exports.__widl_f_push_state_with_url_History = function(arg0, arg1, arg2, arg3, arg4, arg5, exnptr) {
    let varg2 = getStringFromWasm(arg2, arg3);
    let varg4 = arg4 == 0 ? undefined : getStringFromWasm(arg4, arg5);
    try {
        __widl_f_push_state_with_url_History_target.call(getObject(arg0), getObject(arg1), varg2, varg4);
    } catch (e) {
        const view = getUint32Memory();
        view[exnptr / 4] = 1;
        view[exnptr / 4 + 1] = addHeapObject(e);

    }
};

__exports.__widl_instanceof_KeyboardEvent = function(idx) {
    return getObject(idx) instanceof KeyboardEvent ? 1 : 0;
};

const __widl_f_key_code_KeyboardEvent_target = GetOwnOrInheritedPropertyDescriptor(typeof KeyboardEvent === 'undefined' ? null : KeyboardEvent.prototype, 'keyCode').get || function() {
    throw new Error(`wasm-bindgen: KeyboardEvent.keyCode does not exist`);
};

__exports.__widl_f_key_code_KeyboardEvent = function(arg0) {
    return __widl_f_key_code_KeyboardEvent_target.call(getObject(arg0));
};

__exports.__widl_f_pathname_Location = function(ret, arg0, exnptr) {
    try {

        const retptr = passStringToWasm(getObject(arg0).pathname);
        const retlen = WASM_VECTOR_LEN;
        const mem = getUint32Memory();
        mem[ret / 4] = retptr;
        mem[ret / 4 + 1] = retlen;

    } catch (e) {
        const view = getUint32Memory();
        view[exnptr / 4] = 1;
        view[exnptr / 4 + 1] = addHeapObject(e);

    }
};

const __widl_f_append_child_Node_target = typeof Node === 'undefined' ? null : Node.prototype.appendChild || function() {
    throw new Error(`wasm-bindgen: Node.appendChild does not exist`);
};

__exports.__widl_f_append_child_Node = function(arg0, arg1, exnptr) {
    try {
        return addHeapObject(__widl_f_append_child_Node_target.call(getObject(arg0), getObject(arg1)));
    } catch (e) {
        const view = getUint32Memory();
        view[exnptr / 4] = 1;
        view[exnptr / 4 + 1] = addHeapObject(e);

    }
};

const __widl_f_remove_child_Node_target = typeof Node === 'undefined' ? null : Node.prototype.removeChild || function() {
    throw new Error(`wasm-bindgen: Node.removeChild does not exist`);
};

__exports.__widl_f_remove_child_Node = function(arg0, arg1, exnptr) {
    try {
        return addHeapObject(__widl_f_remove_child_Node_target.call(getObject(arg0), getObject(arg1)));
    } catch (e) {
        const view = getUint32Memory();
        view[exnptr / 4] = 1;
        view[exnptr / 4 + 1] = addHeapObject(e);

    }
};

const __widl_f_node_type_Node_target = GetOwnOrInheritedPropertyDescriptor(typeof Node === 'undefined' ? null : Node.prototype, 'nodeType').get || function() {
    throw new Error(`wasm-bindgen: Node.nodeType does not exist`);
};

__exports.__widl_f_node_type_Node = function(arg0) {
    return __widl_f_node_type_Node_target.call(getObject(arg0));
};

const __widl_f_child_nodes_Node_target = GetOwnOrInheritedPropertyDescriptor(typeof Node === 'undefined' ? null : Node.prototype, 'childNodes').get || function() {
    throw new Error(`wasm-bindgen: Node.childNodes does not exist`);
};

__exports.__widl_f_child_nodes_Node = function(arg0) {
    return addHeapObject(__widl_f_child_nodes_Node_target.call(getObject(arg0)));
};

const __widl_f_set_text_content_Node_target = GetOwnOrInheritedPropertyDescriptor(typeof Node === 'undefined' ? null : Node.prototype, 'textContent').set || function() {
    throw new Error(`wasm-bindgen: Node.textContent does not exist`);
};

__exports.__widl_f_set_text_content_Node = function(arg0, arg1, arg2) {
    let varg1 = arg1 == 0 ? undefined : getStringFromWasm(arg1, arg2);
    __widl_f_set_text_content_Node_target.call(getObject(arg0), varg1);
};

const __widl_f_item_NodeList_target = typeof NodeList === 'undefined' ? null : NodeList.prototype.item || function() {
    throw new Error(`wasm-bindgen: NodeList.item does not exist`);
};

__exports.__widl_f_item_NodeList = function(arg0, arg1) {

    const val = __widl_f_item_NodeList_target.call(getObject(arg0), arg1);
    return isLikeNone(val) ? 0 : addHeapObject(val);

};

const __widl_f_length_NodeList_target = GetOwnOrInheritedPropertyDescriptor(typeof NodeList === 'undefined' ? null : NodeList.prototype, 'length').get || function() {
    throw new Error(`wasm-bindgen: NodeList.length does not exist`);
};

__exports.__widl_f_length_NodeList = function(arg0) {
    return __widl_f_length_NodeList_target.call(getObject(arg0));
};

const __widl_f_set_item_Storage_target = typeof Storage === 'undefined' ? null : Storage.prototype.setItem || function() {
    throw new Error(`wasm-bindgen: Storage.setItem does not exist`);
};

__exports.__widl_f_set_item_Storage = function(arg0, arg1, arg2, arg3, arg4, exnptr) {
    let varg1 = getStringFromWasm(arg1, arg2);
    let varg3 = getStringFromWasm(arg3, arg4);
    try {
        __widl_f_set_item_Storage_target.call(getObject(arg0), varg1, varg3);
    } catch (e) {
        const view = getUint32Memory();
        view[exnptr / 4] = 1;
        view[exnptr / 4 + 1] = addHeapObject(e);

    }
};

__exports.__widl_instanceof_Window = function(idx) {
    return getObject(idx) instanceof Window ? 1 : 0;
};

__exports.__widl_f_document_Window = function(arg0) {

    const val = getObject(arg0).document;
    return isLikeNone(val) ? 0 : addHeapObject(val);

};

__exports.__widl_f_location_Window = function(arg0) {
    return addHeapObject(getObject(arg0).location);
};

__exports.__widl_f_history_Window = function(arg0, exnptr) {
    try {
        return addHeapObject(getObject(arg0).history);
    } catch (e) {
        const view = getUint32Memory();
        view[exnptr / 4] = 1;
        view[exnptr / 4 + 1] = addHeapObject(e);

    }
};

__exports.__widl_f_local_storage_Window = function(arg0, exnptr) {
    try {

        const val = getObject(arg0).localStorage;
        return isLikeNone(val) ? 0 : addHeapObject(val);

    } catch (e) {
        const view = getUint32Memory();
        view[exnptr / 4] = 1;
        view[exnptr / 4 + 1] = addHeapObject(e);

    }
};

const __widl_f_log_1__target = console.log;

__exports.__widl_f_log_1_ = function(arg0) {
    __widl_f_log_1__target(getObject(arg0));
};

__exports.__wbg_newnoargs_6a80f84471205fc8 = function(arg0, arg1) {
    let varg0 = getStringFromWasm(arg0, arg1);
    return addHeapObject(new Function(varg0));
};

__exports.__wbg_call_582b20dfcad7fee4 = function(arg0, arg1, exnptr) {
    try {
        return addHeapObject(getObject(arg0).call(getObject(arg1)));
    } catch (e) {
        const view = getUint32Memory();
        view[exnptr / 4] = 1;
        view[exnptr / 4 + 1] = addHeapObject(e);

    }
};

__exports.__wbindgen_object_clone_ref = function(idx) {
    return addHeapObject(getObject(idx));
};

function dropObject(idx) {
    if (idx < 36) return;
    heap[idx] = heap_next;
    heap_next = idx;
}

__exports.__wbindgen_object_drop_ref = function(i) { dropObject(i); };

__exports.__wbindgen_string_new = function(p, l) {
    return addHeapObject(getStringFromWasm(p, l));
};

__exports.__wbindgen_number_get = function(n, invalid) {
    let obj = getObject(n);
    if (typeof(obj) === 'number') return obj;
    getUint8Memory()[invalid] = 1;
    return 0;
};

__exports.__wbindgen_is_null = function(idx) {
    return getObject(idx) === null ? 1 : 0;
};

__exports.__wbindgen_is_undefined = function(idx) {
    return getObject(idx) === undefined ? 1 : 0;
};

__exports.__wbindgen_boolean_get = function(i) {
    let v = getObject(i);
    if (typeof(v) === 'boolean') {
        return v ? 1 : 0;
    } else {
        return 2;
    }
};

__exports.__wbindgen_is_symbol = function(i) {
    return typeof(getObject(i)) === 'symbol' ? 1 : 0;
};

__exports.__wbindgen_string_get = function(i, len_ptr) {
    let obj = getObject(i);
    if (typeof(obj) !== 'string') return 0;
    const ptr = passStringToWasm(obj);
    getUint32Memory()[len_ptr / 4] = WASM_VECTOR_LEN;
    return ptr;
};

__exports.__wbindgen_cb_drop = function(i) {
    const obj = getObject(i).original;
    dropObject(i);
    if (obj.cnt-- == 1) {
        obj.a = 0;
        return 1;
    }
    return 0;
};

__exports.__wbindgen_closure_wrapper1026 = function(a, b, _ignored) {
    const f = wasm.__wbg_function_table.get(2);
    const d = wasm.__wbg_function_table.get(3);
    const cb = function(arg0) {
        this.cnt++;
        let a = this.a;
        this.a = 0;
        try {
            return f(a, b, addHeapObject(arg0));

        } finally {
            this.a = a;
            if (this.cnt-- == 1) d(this.a, b);

        }

    };
    cb.a = a;
    cb.cnt = 1;
    let real = cb.bind(cb);
    real.original = cb;
    return addHeapObject(real);
};

__exports.__wbindgen_throw = function(ptr, len) {
    throw new Error(getStringFromWasm(ptr, len));
};

function init(path_or_module) {
    let instantiation;
    const imports = { './todomvc': __exports };
    if (path_or_module instanceof WebAssembly.Module) {
        instantiation = WebAssembly.instantiate(path_or_module, imports)
        .then(instance => {
        return { instance, module: path_or_module }
    });
} else {
    const data = fetch(path_or_module);
    if (typeof WebAssembly.instantiateStreaming === 'function') {
        instantiation = WebAssembly.instantiateStreaming(data, imports);
    } else {
        instantiation = data
        .then(response => response.arrayBuffer())
        .then(buffer => WebAssembly.instantiate(buffer, imports));
    }
}
return instantiation.then(({instance}) => {
    wasm = init.wasm = instance.exports;

});
};
self.wasm_bindgen = Object.assign(init, __exports);
})();
