use custom_elements::{CustomElement, ShadowDOM};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{window, HtmlElement, Node, Text};

// The boring part: a basic DOM component
struct MyWebComponent {
    name_node: Text,
}

impl MyWebComponent {
    fn new() -> Self {
        let window = window().unwrap();
        let document = window.document().unwrap();
        let name_node = document.create_text_node("friend");
        Self { name_node }
    }

    fn view(&self) -> Node {
        let window = window().unwrap();
        let document = window.document().unwrap();
        let el = document.create_element("p").unwrap();
        let t1 = document.create_text_node("Welcome to my web component, ");
        let t3 = document.create_text_node("!");
        el.append_child(&t1).unwrap();
        el.append_child(&self.name_node).unwrap();
        el.append_child(&t3).unwrap();

        el.unchecked_into()
    }
}

impl Default for MyWebComponent {
    fn default() -> Self {
        Self::new()
    }
}

// Here's the interesting part: configuring the Custom Element
impl CustomElement for MyWebComponent {
    fn to_node(&mut self) -> Node {
        self.view()
    }

    fn observed_attributes() -> Vec<&'static str> {
        vec!["name"]
    }

    fn attribute_changed_callback(
        &self,
        _this: &HtmlElement,
        name: String,
        _old_value: Option<String>,
        new_value: Option<String>,
    ) {
        if name == "name" {
            self.name_node
                .set_data(&new_value.unwrap_or_else(|| "friend".to_string()));
        }
    }

    fn connected_callback(&self, _this: &HtmlElement) {
        log("connected");
    }

    fn disconnected_callback(&self, _this: &HtmlElement) {
        log("disconnected");
    }

    fn adopted_callback(&self, _this: &HtmlElement) {
        log("adopted");
    }
}

// wasm_bindgen entry point defines the Custom Element, then creates a few of them
#[wasm_bindgen]
pub fn run() -> Result<(), JsValue> {
    // define the Custom Element
    MyWebComponent::define("ce-vanilla");

    let window = window().unwrap();
    let document = window.document().unwrap();

    let el_1 = document.create_element("ce-vanilla")?;

    let el_2 = document.create_element("ce-vanilla")?;
    el_2.set_attribute("name", "Alice")?;

    let el_3 = document.create_element("ce-vanilla")?;
    el_3.set_attribute("name", "Bob")?;

    let body = document.body().unwrap();
    body.append_child(&el_1)?;
    body.append_child(&el_2)?;
    body.append_child(&el_3)?;

    Ok(())
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}
