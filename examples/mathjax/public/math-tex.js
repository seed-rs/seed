/*
Typesets math written in (La)TeX, using [MathJax](http://mathjax.org).

##### Example

    <math-tex>c = \sqrt{a^2 + b^2}</math-tex>

##### Example

    <math-tex mode="display">\sum_{k=1}^n k = \frac{n (n + 1)}{2}</math-tex>

@element math-tex
@version 0.3.2
@homepage http://github.com/janmarthedal/math-tex/
*/
(function(global) {
    'use strict';

    const TAG_NAME = 'math-tex',
        CONTROLLER_TAG_NAME = 'math-tex-controller',
        mutation_config = {childList: true, characterData: true, attributes: true, subtree: true};
    let handler;

    function check_handler() {
        if (handler) return;
        handler = document.querySelector(CONTROLLER_TAG_NAME) || document.createElement(CONTROLLER_TAG_NAME);
        if (!handler || typeof handler.typeset !== 'function') {
            console.warn('no %s element defined; %s element will not work', CONTROLLER_TAG_NAME, TAG_NAME);
            handler = undefined;
        } else if (!document.contains(handler))
            document.head.appendChild(handler);
    }

    function update(elem) {
        const sdom = elem.shadowRoot,
            math = elem.textContent.trim(),
            isBlock = elem.getAttribute('mode') === 'display',
            check = (isBlock ? 'D' : 'I') + math;
        if (check !== elem._private.check) {
            while (sdom.firstChild)
                sdom.removeChild(sdom.firstChild);
            elem._private.check = check;
            if (math.length) {
                handler.typeset(math, isBlock, function(melem, styleNode) {
                    sdom.appendChild(styleNode.cloneNode(true));
                    sdom.appendChild(melem);
                });
            }
        }
    }

    class MathTex extends HTMLElement {

        constructor() {
            super();
            this.attachShadow({mode: 'open'});
            check_handler();
        }

        connectedCallback() {
            const elem = this;
            global.requestAnimationFrame(function() {
                elem._private = {
                    check: '',
                    observer: new MutationObserver(function () {
                        update(elem);
                    })
                };
                update(elem);
                elem._private.observer.observe(elem, mutation_config);
            });
        }

        disconnectedCallback() {
            if (this._private) {
                this._private.observer.disconnect();
                delete this._private;
            }
        }

    }

    global.customElements.define(TAG_NAME, MathTex);

})(window);
