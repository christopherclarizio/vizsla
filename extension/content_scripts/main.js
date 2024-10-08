// the relative path from the WebExtension manifest.json file to the module
VIZSLA_REL_PATH = 'wasm/vizsla.js';

const import_and_init_wasm_wrapper_module = async (module_url) => {
    console.debug('[vizsla] attempting to load wasm wrapper module');
    return import(module_url).then(({default: init}) => init());
}

const main = async () => {
    console.debug("[vizsla] started main content script");

    const vizsla_module_url = browser.runtime.getURL(VIZSLA_REL_PATH);
    import_and_init_wasm_wrapper_module(vizsla_module_url)
        .then(vizsla => {
            console.debug('[vizsla] loaded and initialized wasm wrapper module');        

            console.debug(`[vizsla] ran wasm: vizsla.add(1, 2) = ${vizsla.add(1, 2)}`);
        })
        .catch(error => {
            console.error(`[vizsla] failed to load and initialize wasm wrapper module: ${error}`);
        })
}

main();