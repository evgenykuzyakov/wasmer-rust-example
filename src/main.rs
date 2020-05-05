extern crate wasmer_runtime;

use wasmer_runtime::{error, instantiate, imports};

// Make sure that the compiled wasm-sample-app is accessible at this path.
static WASM: &'static [u8] = include_bytes!("../math-bug/res/math_bug.wasm");

fn main() -> error::Result<()> {
    let import_object = imports! {};

    // Compile our webassembly into an `Instance`.
    let instance = instantiate(WASM, &import_object)?;

    // Call our exported function!
    println!("Running multiply_8");
    instance.call("multiply_8", &[])?;
    println!("multiply_8 is OK");

    println!("Running multiply_4");
    instance.call("multiply_4", &[])?;
    println!("multiply_4 is OK");

    Ok(())
}
