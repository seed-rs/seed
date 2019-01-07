use web_sys;

/// Convenience function to avoid repeating expect logic.
pub fn window() -> web_sys::Window {
    web_sys::window().expect("Can't find the global Window")
}


/// Convenience function to access the web_sys DOM document.
pub fn document() -> web_sys::Document {
    window().document().expect("Can't find document")
}