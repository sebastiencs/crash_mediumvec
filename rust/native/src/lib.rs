#[macro_use]
extern crate neon;
extern crate mediumvec;

use neon::vm::{Call, JsResult};
use neon::js::JsString;

fn start_rust(call: Call) -> JsResult<JsString> {
    let scope = call.scope;

    let mut vec = mediumvec::Vec32::new();
    vec.push(10);
    vec.push(10); // Crash here

    Ok(JsString::new(scope, "Hello from Neon!").unwrap())
}

register_module!(m, {
    m.export("start_rust", start_rust)
});
