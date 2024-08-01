console.log("[vizsla] content_scripts/main.js loaded");

document.body.style.border = "5px solid green";

const WASM_MOD_URL = chrome.runtime.getURL('vizsla/pkg/vizsla.js');

// Import Wasm module binding using dynamic import
// "init" may fail if the current site CSP restricts the use of Wasm (e.g. any github.com page)
// In this case instantiate module in the background worker (see background.js) and use message passing
const loadWasmModule = async () => {
    const { default: init, add } = await import(WASM_MOD_URL);

    return init().catch(() => null);
};


(async () => {
    const vizsla = await loadWasmModule();

    // If the module is successfully initialized,
    // import entities from the module
    if (vizsla) {
        console.log("[vizsla] Wasm module loaded");

        console.log(`[vizsla] Running Wasm module: vizsla.add(1, 2) = ${vizsla.add(1, 2)}`);
    }
})();