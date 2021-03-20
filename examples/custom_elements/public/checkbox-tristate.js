import { LitElement, html, css } from 'https://unpkg.com/lit-element/lit-element.js?module';

class CheckboxTristateElement extends LitElement {
    static get properties() {
        return {
            name: { type: String },
            label: { type: String },
            state: { type: String },
        };
    }

    constructor() {
        super();
        this.name = null;
        this.label = "";
        this.state = "unchecked";
    }

    render() {
        return html`
            <div>
                <input type="checkbox" name="${this.name}"></input>
                <label for="${this.name}">${this.label}</label>
            </div>`;
    }

    updated(changedProperties) {
        if (changedProperties.has("state")) {
            const checkbox = this.getElementsByTagName("input")[0];
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
