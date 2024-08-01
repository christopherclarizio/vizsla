// the relative path from the WebExtension manifest.json file to the module
VIZSLA_REL_PATH = 'vizsla/pkg/vizsla.js';

const import_and_init_wasm_wrapper_module = async (module_url) => {
    const {default: init} = await import(module_url);

    return init();
}

const main = async () => {
    console.debug("[vizsla] content_scripts/main.js started");

    const vizsla_module_url = browser.runtime.getURL(VIZSLA_REL_PATH);
    vizsla = await import_and_init_wasm_wrapper_module(vizsla_module_url);
    console.debug('[vizsla] wasm wrapper module loaded and initialized');

    console.debug(`[vizsla] running Wasm module: vizsla.add(1, 2) = ${vizsla.add(1, 2)}`);
}

main();