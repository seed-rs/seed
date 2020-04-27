use wasm_bindgen::{JsCast, JsValue};
use web_sys::{Document, HtmlElement};

pub struct Hud {
    num_bunnies: HtmlElement,
    fps: HtmlElement,
}

impl Hud {
    pub fn new(document: &Document, body: &HtmlElement) -> Result<Self, JsValue> {
        let container: HtmlElement = document.create_element("div")?.dyn_into()?;
        container.set_class_name("info");
        body.append_child(&container)?;

        let num_bunnies: HtmlElement = document.create_element("div")?.dyn_into()?;
        num_bunnies.set_class_name("info-num_bunnies");
        num_bunnies.set_text_content(Some(""));
        container.append_child(&num_bunnies)?;

        let fps: HtmlElement = document.create_element("div")?.dyn_into()?;
        fps.set_class_name("info-fps");
        fps.set_text_content(Some(""));
        container.append_child(&fps)?;

        Ok(Self { num_bunnies, fps })
    }

    pub fn update(&self, len: usize, fps: u32) {
        let s = format!("bunnies: {}", len);
        self.num_bunnies.set_text_content(Some(&s));
        let s = format!("fps: {}", fps);
        self.fps.set_text_content(Some(&s));
    }
}
