(function (global) {
    'use strict';

    const document = global.document,
        states = {start: 1, loading: 2, ready: 3, typesetting: 4, error: 5};
    let mathjaxHub,
        typesets = [],
        state = states.start,
        styleNode,
        src = 'https://cdnjs.cloudflare.com/ajax/libs/mathjax/2.7.5/latest.js';

    function getStyleNode() {
        const styleNodes = document.querySelectorAll('style');
        const sn = Array.prototype.filter.call(styleNodes, function(n) {
        return n.sheet && n.sheet.cssRules.length > 100
            && n.sheet.cssRules[0].selectorText === '.mjx-chtml';
        });
        styleNode = sn[0];
    }

    // precondition: state === states.ready
    function flush_typesets() {
        if (!typesets.length) return;
        const jaxs = [], items = [];
        typesets.forEach(function(item) {
            const script = document.createElement('script'),
                div = document.createElement('div');
            script.type = item[1] ? 'math/tex; mode=display' : 'math/tex';
            script.text = item[0];
            div.style.position = 'fixed';
            div.style.top = 0;
            div.style.left = '99999px';
            div.appendChild(script);
            document.body.appendChild(div);
            jaxs.push(script);
            items.push([div, item[2]]);
        });
        typesets = [];
        state = states.typesetting;
        mathjaxHub.Queue(['Typeset', mathjaxHub, jaxs]);
        mathjaxHub.Queue(function() {
            if (!styleNode)
                getStyleNode();
            items.forEach(function(item) {
                const div = item[0];
                const result = div.firstElementChild.tagName === 'SPAN' ? div.firstElementChild : null;
                item[1](result, styleNode);
                document.body.removeChild(div);
            });
            state = states.ready;
            flush_typesets();
        });
    }

    function load_library() {
        state = states.loading;
        global.MathJax = {
            skipStartupTypeset: true,
            showMathMenu: false,
            jax: ['input/TeX', 'output/CommonHTML'],
            TeX: {
                extensions: ['AMSmath.js', 'AMSsymbols.js', 'noErrors.js', 'noUndefined.js']
            },
            AuthorInit: function () {
                mathjaxHub = global.MathJax.Hub;
                mathjaxHub.Register.StartupHook('End', function() {
                    state = states.ready;
                    flush_typesets();
                });
            }
        };
        var script = document.createElement('script');
        script.type = 'text/javascript';
        script.src = src;
        script.async = true;
        script.onerror = function () {
            console.warn('Error loading MathJax library ' + src);
            state = states.error;
            typesets = [];
        };
        document.head.appendChild(script);
    }

    class MathTexController extends HTMLElement {

        connectedCallback() {
            if (this.hasAttribute('src'))
                src = this.getAttribute('src');
            if (!this.hasAttribute('lazy'))
                load_library();
        }

        typeset(math, displayMode, cb) {
            if (state === states.error)
                return;
            typesets.push([math, displayMode, cb]);
            if (state === states.start)
                load_library();
            else if (state === states.ready)
                flush_typesets();
        }

    }

    global.customElements.define('math-tex-controller', MathTexController);

})(window);
