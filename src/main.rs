extern crate wasmer_runtime;

use std::ffi::c_void;

mod runtime;
use crate::runtime::{Runtime, imports};

use wasmer_runtime::{
    instantiate,
    error,
    Value
};

// Make sure that the compiled wasm-sample-app is accessible at this path.
static WASM: &'static [u8] = include_bytes!("../wasm-sample-app/target/wasm32-unknown-unknown/release/wasm_sample_app.wasm");

fn run(n: u32) ->  error::Result<Vec<Value>> {
    let mut runtime = Runtime { a: 42, total_gas: 0, gas_limit: 1_000_000_000_000_000_000};

    let import_object = imports::build();

    // Compile our webassembly into an `Instance`.
    let mut instance = instantiate(WASM, &import_object)?;

    instance.context_mut().data = &mut runtime as *mut _ as *mut c_void;

    // Call our exported function!
    instance.call("hello_wasm", &[Value::I32(n as i32)]).map_err(Into::<error::Error>::into)
}

fn main() -> error::Result<()> {
    // Let's define the import object used to import our function
    // into our webassembly sample application.
    //
    // We've defined a macro that makes it super easy.
    //
    // The signature tells the runtime what the signature (the parameter
    // and return types) of the function we're defining here is.
    // The allowed types are `i32`, `u32`, `i64`, `u64`,
    // `f32`, and `f64`.
    //
    let num_iter = 1;
    let n: u32 = 1_000_000_000 / num_iter;

    for _ in 0..num_iter {
        run(n)?;
    }

    Ok(())
}