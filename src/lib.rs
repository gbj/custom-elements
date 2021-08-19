use wasm_bindgen::{prelude::*, JsCast};
use web_sys::{HtmlElement, Node};

/// A custom DOM element that can be reused via the Web Components/Custom Elements standard.
pub trait CustomElement: Default {
    /// Returns a root [Node][web_sys::Node] that will be appended to the custom element.
    /// Depending on your component, this will probably do some kind of initialization or rendering.
    fn to_node(&mut self) -> Node;

    /// Whether a [Shadow DOM](https://developer.mozilla.org/en-US/docs/Web/Web_Components/Using_shadow_DOM)
    /// should be attached to the element or not. Shadow DOM encapsulates styles, but makes some DOM manipulation more difficult.
    ///
    /// Defaults to `true`.
    fn shadow() -> bool {
        true
    }

    /// The names of the attributes whose changes should be observed. If an attribute name is in this list,
    /// [attribute_changed_callback](CustomElement::attribute_changed_callback) will be invoked when it changes.
    /// If it is not, nothing will happen when the DOM attribute changes.
    ///
    /// ```
    /// fn observed_attributes() -> Vec<&'static str> {
    ///    vec!["name"]
    /// }
    /// ```
    fn observed_attributes() -> Vec<&'static str> {
        Vec::new()
    }

    /// Invoked each time the custom element is appended into a document-connected element.
    /// This will happen each time the node is moved, and may happen before the element's contents have been fully parsed.
    ///
    /// The argument is the [HtmlElement](web_sys::HtmlElement) for the custom element itself.
    ///
    /// ```
    /// fn connected_callback(&self) -> Box<dyn FnMut(HtmlElement)> {
    ///		Box::new(|_this: HtmlElement| log("connected"))
    /// }
    /// ```
    fn connected_callback(&self) -> Box<dyn FnMut(HtmlElement)> {
        Box::new(|_| ())
    }

    /// Invoked each time the custom element is disconnected from the document's DOM.
    ///
    /// The argument is the [HtmlElement](web_sys::HtmlElement) for the custom element itself.
    ///
    /// ```
    /// fn disconnected_callback(&self) -> Box<dyn FnMut(HtmlElement)> {
    ///		Box::new(|_this: HtmlElement| log("disconnected"))
    /// }
    /// ```
    fn disconnected_callback(&self) -> Box<dyn FnMut(HtmlElement)> {
        Box::new(|_| ())
    }

    /// Invoked each time the custom element is moved to a new document.
    ///
    /// The argument is the [HtmlElement](web_sys::HtmlElement) for the custom element itself.
    ///
    /// ```
    /// fn adopted_callback(&self) -> Box<dyn FnMut(HtmlElement)> {
    ///		Box::new(|_this: HtmlElement| log("adopted"))
    /// }
    /// ```
    fn adopted_callback(&self) -> Box<dyn FnMut(HtmlElement)> {
        Box::new(|_| ())
    }

    /// Invoked each time one of the custom element's attributes is added, removed, or changed.
    /// Which attributes to notice change for is specified in [observed_attributes](CustomElement::observed_attributes).
    ///
    /// The first argument is the [HtmlElement](web_sys::HtmlElement) for the custom element itself.
    ///
    /// The second, third, and fourth arguments are the name of the attribute that has been changed, its old value, and its new value.
    ///
    /// ```
    /// fn attribute_changed_callback(
    ///    &self,
    ///) -> Box<dyn FnMut(HtmlElement, String, Option<String>, Option<String>)> {
    ///    let node = self.name_node.clone();
    ///    Box::new(move |_this, name, _old_value, new_value| {
    ///        if name == "name" {
    ///            node.set_data(&new_value.unwrap_or_else(|| "friend".to_string()));
    ///        }
    ///    })
    /// }
    /// ```
    fn attribute_changed_callback(
        &self,
    ) -> Box<dyn FnMut(HtmlElement, String, Option<String>, Option<String>)> {
        Box::new(|_, _, _, _| ())
    }

    ///
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
