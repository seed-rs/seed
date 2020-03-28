import { LitElement, html, css } from 'https://unpkg.com/lit-element/lit-element.js?module';

class CheckboxTristateElement extends LitElement {
    static get properties() {
        return {
            state: { type: String },
        };
    }

    constructor() {
        super();
        this.state = "unchecked";
    }

    render() {
        return html`<input type="checkbox"></input>`;
    }

    updated(changedProperties) {
        if (changedProperties.has("state")) {
            const checkbox = this.firstElementChild;
            switch (this.state) {
                case "unchecked":
                    checkbox.checked = false;
                    checkbox.indeterminate = false;
                    break

                case "indeterminate":
                    checkbox.checked = false;
                    checkbox.indeterminate = true;
                    break

                case "checked":
                    checkbox.checked = true;
                    checkbox.indeterminate = false;
                    break
            }
        }
    }

    createRenderRoot() {
        return this;
    }
}

customElements.define('checkbox-tristate', CheckboxTristateElement);
