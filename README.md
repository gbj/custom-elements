A framework-agnostic CustomElement trait to create Rust/WASM Web Components/Custom Elements easily without writing any JavaScript.

# Overview

The [Web Components](https://github.com/WICG/webcomponents) standard creates a browser feature that allows you to create reusable components, called Custom Elements. (People often use the label “Web Components” when they mean Custom Elements in particular; the Web Components standard also includes Shadow DOM and HTML Templates.)

While `web_sys` exposes the browser’s [CustomElementRegistry](https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.CustomElementRegistry.html) interface, it can be hard to use. Creating a Custom Element requires calling `customElements.define()` and passing it an ES2015 class that `extends HTMLElement`, which is not currently possible to do directly from Rust.

This crate provides a `CustomElement` trait that, when implemented, allows you to encapsulate any Rust structure as a reusable web component without writing any JavaScript. In theory it should be usable with any Rust front-end framework; the `examples` directory contains examples for Yew and for a vanilla Rust/WASM component.

```
impl CustomElement for MyWebComponent {
    fn to_node(&mut self) -> Node {
        self.view()
    }

    fn observed_attributes() -> Vec<&'static str> {
        vec!["name"]
    }

    fn attribute_changed_callback(
        &self,
    ) -> Box<dyn FnMut(web_sys::HtmlElement, String, Option<String>, Option<String>)> {
        let node = self.name_node.clone();
        Box::new(move |_this, _name, _old_value, new_value| {
            node.set_data(&new_value.unwrap_or("friend".to_string()));
        })
    }
}

#[wasm_bindgen]
pub fn define_custom_elements() {
    MyWebComponent::define("my-component");
}
```

# Running the Examples

The examples use `wasm-pack` and a simple Python server. If you have the right tools installed, you should be able to run them with a simple

`./build.sh && ./runserver`