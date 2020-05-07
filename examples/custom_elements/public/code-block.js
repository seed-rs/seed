import { LitElement, html, css } from 'https://unpkg.com/lit-element/lit-element.js?module';
import { unsafeHTML } from 'https://unpkg.com/lit-html/directives/unsafe-html.js?module';

class CodeBlockElement extends LitElement {
    static get properties() { return {
        lang: "",
        code: "",
    };}

    render() {
        const highlightedCode = highlightCode(this.code, this.lang);
        return html`<pre><code>${unsafeHTML(highlightedCode)}</code></pre>`;
    }

    createRenderRoot() {
        return this;
    }
}
customElements.define('code-block', CodeBlockElement);

function highlightCode(code, lang) {
  // https://highlightjs.readthedocs.io/en/latest/api.html#highlightauto-value-languagesubset
  const highlightedCode =
     window
        .hljs
        .highlightAuto(code, lang ? [lang] : undefined)
        .value;
  return highlightedCode
}
