//! The Web Components standard creates a browser feature that allows you to create reusable components, called Custom Elements.
//!
//! While web_sys exposes the browser’s CustomElementRegistry interface, it can be hard to use. Creating a Custom Element requires calling customElements.define() and passing it an ES2015 class that extends HTMLElement, which is not currently possible to do directly from Rust.
//!
//! This crate provides a [CustomElement][CustomElement] trait that, when implemented, allows you to encapsulate any Rust structure as a reusable web component without writing any JavaScript. In theory it should be usable with any Rust front-end framework.
//! ```rust
//! impl CustomElement for MyWebComponent {
//!     fn to_node(&mut self) -> Node {
//!         self.view()
//!     }
//!
//!     fn observed_attributes() -> Vec<&'static str> {
//!         vec!["name"]
//!     }
//!
//!     fn attribute_changed_callback(
//!         &self,
//!         this: &HtmlElement,
//!         name: String,
//!         old_value: Option<String>,
//!         new_value: Option<String>,
//!     ) {
//!         if name == "name" {
//!             // do something
//!         }
//!     }
//! }
//!
//! #[wasm_bindgen]
//! pub fn define_custom_elements() {
//!     MyWebComponent::define("my-component");
//! }
//! ```

use std::sync::{Arc, Mutex};

use wasm_bindgen::prelude::*;
use wasm_bindgen::UnwrapThrowExt;
use web_sys::{HtmlElement, Node};

/// A custom DOM element that can be reused via the Web Components/Custom Elements standard.
///
/// Note that your component should implement [Default][std::default::Default], which allows the
/// browser to initialize a “default” blank component when a new custom element node is created.
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
    fn connected_callback(&mut self, _this: &HtmlElement) {}

    /// Invoked each time the custom element is disconnected from the document's DOM.
    fn disconnected_callback(&mut self, _this: &HtmlElement) {}

    /// Invoked each time the custom element is moved to a new document.
    fn adopted_callback(&mut self, _this: &HtmlElement) {}

    /// Invoked each time one of the custom element's attributes is added, removed, or changed.
    /// To observe an attribute, include it in [observed_attributes](CustomElement::observed_attributes).
    fn attribute_changed_callback(
        &mut self,
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

    /// URLs for CSS stylesheets to be attached to the element as `<link>` tags.
    fn style_urls() -> Vec<&'static str> {
        Vec::new()
    }

    /// Must be called somewhere to define the custom element and register it with the DOM Custom Elements Registry.
    ///
    /// Note that custom element names must contain a hyphen.
    ///
    /// ```rust
    /// impl CustomElement for MyCustomElement { /* ... */  */}
    /// #[wasm_bindgen]
    /// pub fn define_elements() {
    ///     MyCustomElement::define("my-component");
    /// }
    /// ```
    fn define(tag_name: &'static str) {
        // constructor function will be called for each new instance of the component
        let constructor = Closure::wrap(Box::new(move |this: HtmlElement| {
            let component = Arc::new(Mutex::new(Self::default()));

            // connectedCallback
            let cmp = component.clone();
            let connected = Closure::wrap(Box::new({
                move |el| {
                    let mut lock = cmp.lock().unwrap_throw();
                    lock.connected_callback(&el);
                }
            }) as Box<dyn FnMut(HtmlElement)>);
            js_sys::Reflect::set(
                &this,
                &JsValue::from_str("_connectedCallback"),
                &connected.into_js_value(),
            )
            .unwrap_throw();

            // disconnectedCallback
            let cmp = component.clone();
            let disconnected = Closure::wrap(Box::new(move |el| {
                let mut lock = cmp.lock().unwrap_throw();
                lock.disconnected_callback(&el);
            }) as Box<dyn FnMut(HtmlElement)>);
            js_sys::Reflect::set(
                &this,
                &JsValue::from_str("_disconnectedCallback"),
                &disconnected.into_js_value(),
            )
            .unwrap_throw();

            // adoptedCallback
            let cmp = component.clone();
            let adopted = Closure::wrap(Box::new(move |el| {
                let mut lock = cmp.lock().unwrap_throw();
                lock.adopted_callback(&el);
            }) as Box<dyn FnMut(HtmlElement)>);
            js_sys::Reflect::set(
                &this,
                &JsValue::from_str("_adoptedCallback"),
                &adopted.into_js_value(),
            )
            .unwrap_throw();

            // attributeChangedCallback
            let cmp = component.clone();
            let attribute_changed = Closure::wrap(Box::new(move |el, name, old_value, new_value| {
                let mut lock = cmp.lock().unwrap_throw();
                lock.attribute_changed_callback(&el, name, old_value, new_value);
            })
                as Box<dyn FnMut(HtmlElement, String, Option<String>, Option<String>)>);
            js_sys::Reflect::set(
                &this,
                &JsValue::from_str("_attributeChangedCallback"),
                &attribute_changed.into_js_value(),
            )
            .unwrap_throw();

            let mut lock = component.lock().unwrap_throw();
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
            constructor.into_js_value(),
            observed_attributes,
            Self::style(),
            stylesheets,
        );
    }
}

// JavaScript shim
#[wasm_bindgen(module = "/src/make_custom_element.js")]
extern "C" {
    fn make_custom_element(
        tag_name: &str,
        shadow: bool,
        build_element: JsValue,
        observed_attributes: JsValue,
        style: Option<&str>,
        stylesheets: JsValue,
    );
}
