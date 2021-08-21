export function make_custom_element(
  tag_name,
  shadow,
  constructor,
  observedAttributes
) {
  customElements.define(
    tag_name,
    class extends HTMLElement {
      static get observedAttributes() {
        return observedAttributes;
      }

      constructor() {
        super();

        // run whatever custom constructor we've been given, and other setup as necessary
        constructor(this);
        this._constructor(this);

        if (shadow) {
          this.attachShadow({ mode: "open" });
          this._injectChildren(this.shadowRoot);
        }
      }

      attributeChangedCallback(name, oldValue, newValue) {
        this._attributeChangedCallback(this, name, oldValue || "", newValue);
      }

      connectedCallback() {
        // on first connection, add children
        if(!this.hasSetup) {
          this.hasSetup = true;
  
          if (!shadow) {
            this._injectChildren(this);
          }
        }

        // otherwise, and also the first time, just run the callback
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