A framework-agnostic CustomElement trait to create Rust/WASM Web Components/Custom Elements easily without writing any JavaScript.

# Overview

The [Web Components](https://github.com/WICG/webcomponents) standard creates a browser feature that allows you to create reusable components, called Custom Elements. (People often use the label “Web Components” when they mean Custom Elements in particular; the Web Components standard also includes Shadow DOM and HTML Templates.)

While `web_sys` exposes the browser’s [CustomElementRegistry](https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.CustomElementRegistry.html) interface, it can be hard to use. Creating a Custom Element requires calling `customElements.define()` and passing it an ES2015 class that `extends HTMLElement`, which is not currently possible to do directly from Rust.

This crate provides a `CustomElement` trait that, when implemented, allows you to encapsulate any Rust structure as a reusable web component without writing any JavaScript. In theory it should be usable with any Rust front-end framework; the `examples` directory contains examples for Yew and for a vanilla Rust/WASM component.

```rust
impl CustomElement for MyWebComponent {
    fn to_node(&mut self) -> Node {
        self.view()
    }

    fn observed_attributes() -> Vec<&'static str> {
        vec!["name"]
    }

    fn attribute_changed_callback(
        &self,
        this: &HtmlElement,
        name: String,
        old_value: Option<String>,
        new_value: Option<String>,
    ) {
        if name == "name" {
            // do something
        }
    }
}

#[wasm_bindgen]
pub fn define_custom_elements() {
    MyWebComponent::define("my-component");
}
```

## Shadow DOM

By default, these custom elements use the Shadow DOM (in “open” mode) to encapsulate the styles and content of the element. You can override that choice simply by implementing the `shadow` method and returning `false`:

```rust
fn shadow() -> bool {
    false
}
```

## Lifecycle Methods

You can implement each of the custom element’s lifecycle callbacks. Each of the callbacks is passed both the component for which the trait is being implemented, and the `HtmlElement` of the custom element.

```rust

fn connected_callback(&self, _this: &HtmlElement) {
    log("connected");
}

fn disconnected_callback(&self, _this: &HtmlElement) {
    log("disconnected");
}

fn adopted_callback(&self, _this: &HtmlElement) {
    log("adopted");
}

fn attribute_changed_callback(
    &self,
    this: &HtmlElement,
    name: String,
    old_value: Option<String>,
    new_value: Option<String>,
) {
    if name == "name" {
        // do something
    }
}
```

## Using Rust Frameworks

The minimum needed to implement `CustomElement` is some way to get a `web_sys::Node` from your component. It’s also generally helpful to have it respond to changes in its attributes via the `attribute_changed_callback`. Depending on the framework, these may be more or less difficult to accomplish; in particular, for Elm-inspired frameworks you may need to create a wrapper that owns some way of updating the app’s state.

See the Yew example for an example of how to work with a framework’s API.

# Resources

This is a fairly minimal wrapper for the Custom Elements API. The following MDN sources should give you more than enough information to start creating custom elements:
- [Web Components](https://developer.mozilla.org/en-US/docs/Web/Web_Components)
- [Using custom elements](https://developer.mozilla.org/en-US/docs/Web/Web_Components/Using_custom_elements)
- [Using the lifecycle callbacks](https://developer.mozilla.org/en-US/docs/Web/Web_Components/Using_custom_elements#using_the_lifecycle_callbacks)

# Running the Examples

The examples use `wasm-pack` and a simple Python server. You should be able to run them with a simple

`./build.sh && ./runserver`
