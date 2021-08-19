export function make_custom_element(
  tag_name,
  shadow,
  buildElement,
  observedAttributes,
  style,
  stylesheets
) {
  customElements.define(
    tag_name,
    class extends HTMLElement {
      static get observedAttributes() {
        return observedAttributes;
      }

      constructor() {
        super();

        if (shadow) this.attachShadow({ mode: "open" });

        this.element = this;

        const fragment = document.createDocumentFragment();

        if(style) {
          const style_el = document.createElement("style");
          style_el.textContent = style;
          fragment.appendChild(style_el);
        }

        for(const url of stylesheets) {
          const link = document.createElement("link");
          link.setAttribute("rel", "stylesheet");
          link.setAttribute("href", url);
          fragment.appendChild(link);
        }

        const el = buildElement(this);
        fragment.appendChild(el);

        if (shadow) {
          this.shadowRoot.appendChild(fragment);
        } else {
          this.appendChild(fragment);
        }
      }

      attributeChangedCallback(name, oldValue, newValue) {
        this._attributeChangedCallback(this, name, oldValue || "", newValue);
      }

      connectedCallback() {
        this._connectedCallback(this);
      }

      disconnectedCallback() {
        this._disconnectedCallback(this);
      }

      adoptedCallback() {
        this._adoptedCallback(this);
      }
    }
  );
}