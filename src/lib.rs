use std::sync::{Arc, Mutex};

use wasm_bindgen::{prelude::*, JsCast};
use web_sys::{HtmlElement, Node};

/// Specifies whether a shadow root should be attached to the element, and in which mode.
pub enum ShadowDOM {
    /// No shadow root will be attached; the component will simply be appended to the custom element as a child.
    None,
    /// A shadow root will be attached with the [Mode](https://developer.mozilla.org/en-US/docs/Web/API/ShadowRoot/mode) set to `open`.
    Open,
    /// A shadow root will be attached with the [Mode](https://developer.mozilla.org/en-US/docs/Web/API/ShadowRoot/mode) set to `closed`.
    Closed,
}

/// A custom DOM element that can be reused via the Web Components/Custom Elements standard.
pub trait CustomElement: Default + 'static {
    /// Returns a root [Node][web_sys::Node] that will be appended to the custom element.
    /// Depending on your component, this will probably do some kind of initialization or rendering.
    fn to_node(&mut self) -> Node;

    /// Whether a [Shadow root](https://developer.mozilla.org/en-US/docs/Web/Web_Components/Using_shadow_DOM)
    /// should be attached to the element or not. Shadow DOM encapsulates styles, but makes some DOM manipulation more difficult.
    ///
    /// Defaults to `true`.
    fn shadow() -> bool {
        true
    }

    /// The names of the attributes whose changes should be observed. If an attribute name is in this list,
    /// [attribute_changed_callback](CustomElement::attribute_changed_callback) will be invoked when it changes.
    /// If it is not, nothing will happen when the DOM attribute changes.
    fn observed_attributes() -> Vec<&'static str> {
        Vec::new()
    }

    /// Invoked each time the custom element is appended into a document-connected element.
    /// This will happen each time the node is moved, and may happen before the element's contents have been fully parsed.
    fn connected_callback(&self, _this: &HtmlElement) {}

    /// Invoked each time the custom element is disconnected from the document's DOM.
    fn disconnected_callback(&self, _this: &HtmlElement) {}

    /// Invoked each time the custom element is moved to a new document.
    fn adopted_callback(&self, _this: &HtmlElement) {}

    /// Invoked each time one of the custom element's attributes is added, removed, or changed.
    /// To observe an attribute, include it in [observed_attributes](CustomElement::observed_attributes).
    fn attribute_changed_callback(
        &self,
        _this: &HtmlElement,
        _name: String,
        _old_value: Option<String>,
        _new_value: Option<String>,
    ) {
    }

    /// CSS stylesheet to be attached to the element as a `<style>` tag.
    fn style() -> Option<&'static str> {
        None
    }

    /// URL for CSS stylesheets to be attached to the element as `<link>` tags.
    fn style_urls() -> Vec<&'static str> {
        Vec::new()
    }

    ///
    fn define(tag_name: &'static str) {
        // constructor function will be called for each new instance of the component
        let constructor = Closure::wrap(Box::new(move |this: HtmlElement| {
            let component = Arc::new(Mutex::new(Self::default()));

            // connectedCallback
            let cmp = component.clone();
            let connected = Closure::wrap(Box::new({
                move |el| {
                    let lock = cmp.lock().unwrap();
                    lock.connected_callback(&el);
                }
            }) as Box<dyn FnMut(HtmlElement)>);
            js_sys::Reflect::set(
                &this,
                &JsValue::from_str("_connectedCallback"),
                &connected.as_ref().unchecked_ref(),
            )
            .unwrap();
            connected.forget();

            // disconnectedCallback
            let cmp = component.clone();
            let disconnected = Closure::wrap(Box::new(move |el| {
                let lock = cmp.lock().unwrap();
                lock.disconnected_callback(&el);
            }) as Box<dyn FnMut(HtmlElement)>);
            js_sys::Reflect::set(
                &this,
                &JsValue::from_str("_disconnectedCallback"),
                &disconnected.as_ref().unchecked_ref(),
            )
            .unwrap();
            disconnected.forget();

            // adoptedCallback
            let cmp = component.clone();
            let adopted = Closure::wrap(Box::new(move |el| {
                let lock = cmp.lock().unwrap();
                lock.adopted_callback(&el);
            }) as Box<dyn FnMut(HtmlElement)>);
            js_sys::Reflect::set(
                &this,
                &JsValue::from_str("_adoptedCallback"),
                &adopted.as_ref().unchecked_ref(),
            )
            .unwrap();
            adopted.forget();

            // attributeChangedCallback
            let cmp = component.clone();
            let attribute_changed = Closure::wrap(Box::new(move |el, name, old_value, new_value| {
                let lock = cmp.lock().unwrap();
                lock.attribute_changed_callback(&el, name, old_value, new_value);
            })
                as Box<dyn FnMut(HtmlElement, String, Option<String>, Option<String>)>);
            js_sys::Reflect::set(
                &this,
                &JsValue::from_str("_attributeChangedCallback"),
                &attribute_changed.as_ref().unchecked_ref(),
            )
            .unwrap();
            attribute_changed.forget();

            let mut lock = component.lock().unwrap();
            lock.to_node()
        }) as Box<dyn FnMut(HtmlElement) -> Node>);

        // observedAttributes is static and needs to be known when the class is defined
        let attributes = Self::observed_attributes();
        let observed_attributes = JsValue::from(
            attributes
                .iter()
                .map(|attr| JsValue::from_str(attr))
                .collect::<js_sys::Array>(),
        );

        // styles
        let stylesheets = JsValue::from(
            Self::style_urls()
                .iter()
                .map(|attr| JsValue::from_str(attr))
                .collect::<js_sys::Array>(),
        );

        // call out to JS to define the Custom Element
        make_custom_element(
            tag_name,
            Self::shadow(),
            &constructor,
            observed_attributes,
            Self::style(),
            stylesheets,
        );
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
        style: Option<&str>,
        stylesheets: JsValue,
    );
}
