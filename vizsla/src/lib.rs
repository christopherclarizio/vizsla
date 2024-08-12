use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
//     #[wasm_bindgen]
//     fn alert(s: &str);

    #[wasm_bindgen(js_namespace=console)]
    fn log(s: &str);

//     #[wasm_bindgen(js_namespace=console)]
//     fn debug(s: &str);

//     #[wasm_bindgen(js_namespace=console)]
//     fn error(s: &str);
}

// Called when the wasm module is instantiated
#[wasm_bindgen(start)]
fn main() -> Result<(), JsValue> {
    // Use `web_sys`'s global `window` function to get a handle on the global
    // window object.
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    // let body = document.body().expect("document should have a body");

    // Manufacture the element we're gonna append
    // let val = document.create_element("p")?;
    // val.set_inner_html("Hello from Rust!");

    
    let _ = document.body().unwrap().style().set_property("border", "5px solid blue");

    let closure = Closure::wrap(Box::new(move |event: web_sys::Event| {
        // Your Rust function logic goes here
        log(&format!("Event triggered: {:?}", event));
    }) as Box<dyn FnMut(_)>); 

    let _ = document.add_event_listener_with_callback("keypress", closure.as_ref().unchecked_ref()).expect("unable to register event listener");

    log("[vizsla] wasm module starting");

    closure.forget();

    // document.body.style.border = "5px solid green";

    // body.append_child(&val)?;

    Ok(())
}

#[wasm_bindgen]
pub fn add(a: u32, b: u32) -> u32 {
    a + b
}