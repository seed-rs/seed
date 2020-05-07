import { LitElement, html, css } from 'https://unpkg.com/lit-element/lit-element.js?module';
import { unsafeHTML } from 'https://unpkg.com/lit-html/directives/unsafe-html.js?module';

class FeatherIconElement extends LitElement {
    static get properties() { return {
        icon: { type: String },
        width: { type: Number },
        height: { type: Number },
    };}

    constructor() {
        super();
        this.width = 24;
        this.height = 24;
      }

    render() {
        const svg = feather.icons[this.icon].toSvg({
            width: this.width,
            height: this.height,
        });
        return html`${unsafeHTML(svg)}`;
    }

    createRenderRoot() {
        return this;
    }
}

customElements.define('feather-icon', FeatherIconElement);
