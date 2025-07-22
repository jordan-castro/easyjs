globalThis.console = {
    /**
     * Print to the console.
     * @param  {...any} v the arguments to print.
     */
    log(...v) {
        globalThis.___print(`${v.join(" ")}`);
    },

    /**
     * Panic with a message. 
     * @param  {...any} v the arguments to show when panicing
     */
    error(...v) {
        globalThis.___error(`${v.join(" ")}`);
    },
}