mod component;

use crate::component::Model;
use custom_elements::{inject_stylesheet, GenericCustomElement, HTML_ELEMENT_CONSTRUCTOR};
use wasm_bindgen::prelude::*;
use web_sys::HtmlElement;
use yew::prelude::*;
use yew::Renderer;

pub struct GenericComponentWrapper<T: BaseComponent> {
    component: Option<AppHandle<T>>,
    props_builder: Box<dyn Fn(&HtmlElement) -> T::Properties>,
    attribute_changed_message_builder:
        Box<dyn Fn(String, Option<String>, Option<String>) -> Option<T::Message>>,
    stylesheet: Option<&'static str>,
    queue: Vec<T::Message>,
}

impl<T: BaseComponent> GenericComponentWrapper<T> {
    pub fn new(
        props_builder: Box<dyn Fn(&HtmlElement) -> T::Properties>,
        attribute_changed_message_builder: Box<
            dyn Fn(String, Option<String>, Option<String>) -> Option<T::Message>,
        >,
        stylesheet: Option<&'static str>,
    ) -> Self {
        Self {
            component: None,
            props_builder,
            attribute_changed_message_builder,
            stylesheet,
            queue: vec![],
        }
    }
}

impl<T: BaseComponent> GenericCustomElement for GenericComponentWrapper<T> {
    fn inject_children(&mut self, this: &HtmlElement) {
        let app =
            Renderer::<T>::with_root_and_props(this.clone().into(), (self.props_builder)(this));

        let component = app.render();
        while !self.queue.is_empty() {
            let next_entry = self.queue.remove(0);
            component.send_message(next_entry);
        }
        self.component = Some(component);
        if let Some(stylesheet) = self.stylesheet {
            inject_stylesheet(this, stylesheet);
        }
    }

    fn attribute_changed_callback(
        &mut self,
        _this: &HtmlElement,
        name: String,
        old_value: Option<String>,
        new_value: Option<String>,
    ) {
        if let Some(msg) = (self.attribute_changed_message_builder)(name, old_value, new_value) {
            if let Some(handle) = &self.component {
                handle.send_message(msg);
            } else {
                self.queue.push(msg);
            }
        }
    }
}

#[wasm_bindgen]
pub fn run() {
    custom_elements::define_custom_tag(
        "ce-yew",
        move || {
            GenericComponentWrapper::<Model>::new(Box::new(|_| ()), Box::new(|_, _, _| None), None)
        },
        || (None, &HTML_ELEMENT_CONSTRUCTOR),
        &["group"],
        false,
    );
}
