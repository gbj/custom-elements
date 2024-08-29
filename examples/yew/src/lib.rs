mod component;

use component::Model;
use component::Msg;
use custom_elements::{inject_stylesheet, CustomElement, GenericCustomElement};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{window, HtmlElement};
use yew::html::Scope;
use yew::prelude::*;

struct ComponentWrapper {
    scope: Option<Scope<Model>>,
}

impl ComponentWrapper {
    fn new() -> Self {
        Self { scope: None }
    }
}

impl GenericCustomElement for ComponentWrapper {
    fn inject_children(&mut self, this: &HtmlElement) {
        yew::initialize();
        let app = App::<Model>::new();
        let scope = app.mount(this.clone().unchecked_into());
        self.scope = Some(scope);
        yew::run_loop();

        inject_stylesheet(&this, "/component_style.css");
    }
    fn attribute_changed_callback(
        &mut self,
        _this: &HtmlElement,
        name: String,
        _old_value: Option<String>,
        new_value: Option<String>,
    ) {
        match name.as_str() {
            "value" => {
                if let Some(value) = new_value {
                    if let Ok(value) = value.parse::<i64>() {
                        if let Some(scope) = &self.scope {
                            scope.send_message(Msg::Set(value));
                        }
                    }
                }
            }
            _ => (),
        };
    }
}

impl CustomElement for ComponentWrapper {
    fn observed_attributes() -> &'static [&'static str] {
        &["value"]
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
}
