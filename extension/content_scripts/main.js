const { greet } = wasm_bindgen;

async function run() {
    console.log("[vizsla] loading WebExtension");

    document.body.style.border = "5px solid green";

    console.log(browser.runtime.getURL('vizsla.js'))

    await wasm_bindgen();

    greet("hello from WebExtension");
}

run();