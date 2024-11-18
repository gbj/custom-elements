export function make_custom_element(
    superclass,
    tag_name,
    shadow,
    constructor,
    observedAttributes,
    superclassTag
) {

    customElements.define(
        tag_name,
        class extends HTMLElement {
            static get observedAttributes() {
                return observedAttributes;
            }


            constructor() {
                super();
                this.current_assigned_attributes = new Map();
                //this.initRustComponent();
            }

            initRustComponent() {
                // run whatever custom constructor we've been given, and other setup as necessary
                constructor(this);
                this._constructor(this);
                this._constructor = null;

                if (shadow) {
                    this.attachShadow({mode: "open"});
                    this._injectChildren(this.shadowRoot);
                    this._injectChildren = null;
                }
                this.current_assigned_attributes.forEach((value, key) => {
                    this._attributeChangedCallback(this, key, "", value);
                })
            }

            attributeChangedCallback(name, oldValue, newValue) {
                this.current_assigned_attributes.set(name, newValue);
                if (!this._attributeChangedCallback) {
                    return;
                }
                this._attributeChangedCallback(this, name, oldValue || "", newValue);
            }

            connectedCallback() {
                if (this._disconnected && this._disconnectedCallback) {
                    this._disconnected = false;
                    return;
                }
                this._disconnected = false;
                if (!this._connectedCallback) {
                    this.adoptedCallback();
                }
                // on first connection, add children
                if (!this.hasSetup) {
                    this.hasSetup = true;

                    if (!shadow) {
                        this._injectChildren(this);
                        this._injectChildren = null;
                    }
                }

                // otherwise, and also the first time, just run the callback
                this._connectedCallback(this);
                this._connectedCallback = null;
            }

            disconnectedCallback() {
                this._disconnected = true;
                if (this._disconnectTimeout) {
                    clearTimeout(this._disconnectTimeout);
                }
                this._disconnectTimeout = setTimeout(() => {
                    if (!this._disconnected) {
                        return;
                    }
                    if (!this._disconnectedCallback) {
                        return;
                    }
                    this._disconnectedCallback(this);
                    this._attributeChangedCallback = null;
                    this._disconnectedCallback = null;
                }, 1000);
            }

            adoptedCallback() {
                if (!this._adoptedCallback) {
                    this.initRustComponent();
                }

                this._adoptedCallback(this);
                this._adoptedCallback = null;
            }
        },
        superclassTag ? {extends: superclassTag} : undefined
    );
}
