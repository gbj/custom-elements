export function make_custom_element(
  tag_name,
  shadow,
  buildElement,
  observedAttributes,
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

        const el = buildElement(this);

        if (shadow) {
          this.shadowRoot.appendChild(el);
        } else {
          this.appendChild(el);
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