import { LitElement, html, css } from 'https://unpkg.com/lit-element/lit-element.js?module';
import { unsafeHTML } from 'https://unpkg.com/lit-html/directives/unsafe-html.js?module';

class MathTexElement extends LitElement {

    connectedCallback() {
        super.connectedCallback();
        this.style.display = "block";
    }

    render() {
        const tex = this.innerHTML;
        const html_tex = window.MathJax.tex2mml(tex);

        return html`${unsafeHTML(html_tex)}`;
    }

    createRenderRoot() {
        return this;
    }
}

customElements.define('math-tex', MathTexElement);
