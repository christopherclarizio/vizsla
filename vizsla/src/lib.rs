use std::vec;

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

// state: fn(input) -> state
// normal_mode()

fn buffer_input(buffered_input: vec<char>, event: web_sys::KeyboardEvent) -> Vec<char> {

}

extract_command(buffered_input: vec<char>, current_state: State) -> Option<Command> {

}

apply_command(command: Command, current_state: State) {

}

(":wq", InsertMode) -> fn write_and_quit(state: State) -> State { }

fn handle_keydown_event(event: web_sys::KeyboardEvent) {
    let buffered_input = buffer_input(buffered_input, event);

    match extract_command(buffered_input, current_state) {
        Some(command) => apply_command(current_state, command),
        None => _
    }
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

    let closure = Closure::wrap(Box::new(move |event: web_sys::KeyboardEvent| {
        // Your Rust function logic goes here
        log(&format!("Event triggered: {:?}", event.key()));
    }) as Box<dyn FnMut(_)>); 

    let _ = document.add_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref()).expect("unable to register event listener");

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