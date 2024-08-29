use custom_elements::{inject_style, CustomElement, GenericCustomElement};
use wasm_bindgen::prelude::*;
use web_sys::{window, HtmlElement};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = HTMLParagraphElement, js_namespace = window)]
    pub static HtmlParagraphElementConstructor: js_sys::Function;
}

struct PurpleParagraph {}

impl PurpleParagraph {
    fn new() -> Self {
        Self {}
    }
}

impl Default for PurpleParagraph {
    fn default() -> Self {
        Self::new()
    }
}

// Here's the interesting part: configuring the Custom Element
impl GenericCustomElement for PurpleParagraph {
    fn inject_children(&mut self, this: &HtmlElement) {
        inject_style(&this, "* { color: purple; }");
        let slot = window()
            .unwrap_throw()
            .document()
            .unwrap_throw()
            .create_element("slot")
            .unwrap_throw();
        this.append_child(&slot).unwrap_throw();
    }
}
impl CustomElement for PurpleParagraph {
    fn superclass() -> (Option<&'static str>, &'static js_sys::Function) {
        (Some("p"), &HtmlParagraphElementConstructor)
    }
}

// wasm_bindgen entry point defines the Custom Element, then creates a few of them
#[wasm_bindgen]
pub fn run() -> Result<(), JsValue> {
    // define the Custom Element
    PurpleParagraph::define("purple-paragraph");

    Ok(())
}
