function observeElementSize(element, send_msg_resized) {

    const resizeObserver = new ResizeObserver(entries => {
        const entry = entries[0];

        let size = 0;
        // Browsers use different structures to store the size. Don't ask me why.. 
        if (entry.borderBoxSize instanceof ResizeObserverSize) {
            size = entry.borderBoxSize;
        } else if (entry.borderBoxSize[0] instanceof ResizeObserverSize) {
            size = entry.borderBoxSize[0];
        } else {
            console.error("Cannot get borderBoxSize from ResizeObserver entry!");
        }

        const height = size.blockSize;
        const width = size.inlineSize;
        send_msg_resized(width, height);
    });

    resizeObserver.observe(element);
}
