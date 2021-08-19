mod component;

use std::convert::TryInto;

use component::Model;
use component::Msg;
use custom_elements::CustomElement;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{window, HtmlElement};
use yew::html::Scope;
use yew::prelude::*;

struct ComponentWrapper {
    node: web_sys::Node,
    scope: Scope<Model>,
}

impl ComponentWrapper {
    fn new() -> Self {
        yew::initialize();
        let document = window().unwrap().document().unwrap();
        let fragment = document.create_document_fragment();
        let app = App::<Model>::new();
        let scope = app.mount(fragment.clone().unchecked_into());
        yew::run_loop();
        let node = fragment.unchecked_into();

        Self { node, scope }
    }
}

impl CustomElement for ComponentWrapper {
    fn to_node(&mut self) -> web_sys::Node {
        self.node.clone()
    }

    fn observed_attributes() -> Vec<&'static str> {
        vec!["value"]
    }

    fn attribute_changed_callback(
        &self,
        _this: &HtmlElement,
        name: String,
        _old_value: Option<String>,
        new_value: Option<String>,
    ) {
        match name.as_str() {
            "value" => {
                if let Some(value) = new_value {
                    if let Ok(value) = value.parse::<i64>() {
                        self.scope.send_message(Msg::Set(value));
                    }
                }
            }
            _ => (),
        };
    }
}

impl Default for ComponentWrapper {
    fn default() -> Self {
        Self::new()
    }
}

#[wasm_bindgen]
pub fn run() {
    ComponentWrapper::define("ce-yew");

    let document = window().unwrap().document().unwrap();
    let body = document.query_selector("body").unwrap().unwrap();
    let el_1 = document.create_element("ce-yew").unwrap();
    let el_2 = document.create_element("ce-yew").unwrap();
    body.append_child(&el_1).unwrap();
    body.append_child(&el_2).unwrap();
}
