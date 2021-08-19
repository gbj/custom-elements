use wasm_bindgen::{prelude::*, JsCast};
use web_sys::{HtmlElement, Node};

pub trait CustomElement: Default {
    fn to_node(&mut self) -> Node;

    fn shadow() -> bool {
        true
    }

    fn observed_attributes() -> Vec<&'static str> {
        Vec::new()
    }

    fn attribute_changed_callback(
        &self,
    ) -> Box<dyn FnMut(HtmlElement, String, Option<String>, Option<String>)> {
        Box::new(|_, _, _, _| ())
    }

    fn connected_callback(&self) -> Box<dyn FnMut(HtmlElement)> {
        Box::new(|_| ())
    }

    fn disconnected_callback(&self) -> Box<dyn FnMut(HtmlElement)> {
        Box::new(|_| ())
    }

    fn adopted_callback(&self) -> Box<dyn FnMut(HtmlElement)> {
        Box::new(|_| ())
    }

    fn define(tag_name: &'static str) {
        // constructor function will be called for each new instance of the component
        let constructor = Closure::wrap(Box::new(|this: HtmlElement| {
            let mut component = Self::default();

            // connectedCallback
            let connected = Closure::wrap(component.connected_callback());
            js_sys::Reflect::set(
                &this,
                &JsValue::from_str("_connectedCallback"),
                &connected.as_ref().unchecked_ref(),
            )
            .unwrap();
            connected.forget();

            // disconnectedCallback
            let disconnected = Closure::wrap(component.disconnected_callback());
            js_sys::Reflect::set(
                &this,
                &JsValue::from_str("_disconnectedCallback"),
                &disconnected.as_ref().unchecked_ref(),
            )
            .unwrap();
            disconnected.forget();

            // adoptedCallback
            let adopted = Closure::wrap(component.adopted_callback());
            js_sys::Reflect::set(
                &this,
                &JsValue::from_str("_adoptedCallback"),
                &adopted.as_ref().unchecked_ref(),
            )
            .unwrap();
            adopted.forget();

            // attributeChangedCallback
            let attribute_changed = Closure::wrap(component.attribute_changed_callback());
            js_sys::Reflect::set(
                &this,
                &JsValue::from_str("_attributeChangedCallback"),
                &attribute_changed.as_ref().unchecked_ref(),
            )
            .unwrap();
            attribute_changed.forget();

            component.to_node()
        }) as Box<dyn FnMut(HtmlElement) -> Node>);

        // observedAttributes is static and needs to be known when the class is defined
        let attributes = Self::observed_attributes();
        let observed_attributes = JsValue::from(
            attributes
                .iter()
                .map(|attr| JsValue::from_str(attr))
                .collect::<js_sys::Array>(),
        );

        // call out to JS to define the Custom Element
        make_custom_element(tag_name, Self::shadow(), &constructor, observed_attributes);
        constructor.forget();
    }
}

// JavaScript shim
#[wasm_bindgen(module = "/src/make_custom_element.js")]
extern "C" {
    fn make_custom_element(
        tag_name: &str,
        shadow: bool,
        build_element: &Closure<dyn FnMut(HtmlElement) -> web_sys::Node>,
        observed_attributes: JsValue,
    );
}
