// Define a function that is imported into the module.
// By default, the "env" namespace is used.
extern "C" {
    fn print_str(ptr: *const u8, len: usize, a: i32);
    fn gas(a: u32);
}

// Define a string that is accessible within the wasm
// linear memory.
static HELLO: &'static str = "Hello, World!";

// Export a function named "hello_wasm". This can be called
// from the embedder!
#[no_mangle]
pub unsafe extern fn hello_wasm(num: i32) {
    // Call the function we just imported and pass in
    // the offset of our string and its length as parameters.
    let mut a: i32 = 1;
    for i in 0..num {
        if (i & 1) == 1 {
            a += i;
        } else {
            a -= i;
        }
        gas((if a < 0 {-a} else {a}) as u32);
    }
    print_str(HELLO.as_ptr(), HELLO.len(), a);
}