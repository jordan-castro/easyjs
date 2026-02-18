globalThis.console = {
    log: (...args) => {
        ___console_log(...args);
    },
    error: (...args) => {
        ___console_log(...args);
    },
    warn: (...args) => {
        ___console_log(...args);
    }
};