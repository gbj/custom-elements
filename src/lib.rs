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

    fn connected_callback() -> Box<dyn FnMut(&Self, HtmlElement)> {
        Box::new(|_, _| ())
    }

    fn disconnected_callback() -> Box<dyn FnMut(&Self, HtmlElement)> {
        Box::new(|_, _| ())
    }

    fn adopted_callback() -> Box<dyn FnMut(&Self, HtmlElement)> {
        Box::new(|_, _| ())
    }

    fn define(tag_name: &'static str) {
        // constructor function will be called for each new instance of the component
        let constructor = Closure::wrap(Box::new(|this: HtmlElement| {
            let mut component = Self::default();

            // attributeChangedCallback
            let attribute_changed = Closure::wrap(component.attribute_changed_callback());
            js_sys::Reflect::set(
                &this,
                &JsValue::from_str("_attributeChanged"),
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
#[wasm_bindgen(
    inline_js = r#"export function make_custom_element(tag_name, shadow, buildElement, observedAttributes, attributeChangedCallback) {
  customElements.define(tag_name, class extends HTMLElement {
      static get observedAttributes() { return observedAttributes; }

      constructor() {
          super();
        
          if(shadow) this.attachShadow({ mode: 'open' });

          this.element = this;

          const el = buildElement(this);
          
          if(shadow) {
            this.shadowRoot.appendChild(el);
          } else {
            this.appendChild(el);
          }
      }

      attributeChangedCallback(name, oldValue, newValue) {
        this._attributeChanged(this, name, oldValue || "", newValue);
      }
  });
}"#
)]
extern "C" {
    fn make_custom_element(
        tag_name: &str,
        shadow: bool,
        build_element: &Closure<dyn FnMut(HtmlElement) -> web_sys::Node>,
        observed_attributes: JsValue,
    );
}
