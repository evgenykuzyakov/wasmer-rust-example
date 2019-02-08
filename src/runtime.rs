use std::str;

type StrResult<T> = Result<T, String>;

pub struct Runtime {
    pub a: u32,
    pub total_gas: u64,
    pub gas_limit: u64,
}

impl Runtime {
    fn print_str(&self, _ptr: u32, _len: u32, _aaa: i32) -> () {
        // Get a slice that maps to the memory currently used by the webassembly
        // instance.
        //
        // Webassembly only supports a single memory for now,
        // but in the near future, it'll support multiple.
        //
        // Therefore, we don't assume you always just want to access first
        // memory and force you to specify the first memory.
        // Get a subslice that corresponds to the memory used by the string.
        // let str_slice = &self.memory[ptr as usize..(ptr + len) as usize];
        let str_slice = b"hello";

        // Convert the subslice to a `&str`.
        let _string = str::from_utf8(str_slice).unwrap();

        // Print it!
        // println!("{}", string);
        // println!("self.a {}", self.a);
        // println!("a {}", aaa);
        // println!("self.total_gas {}", self.total_gas);
        // println!("self.gas_limit {}", self.gas_limit);
    }

    fn gas(&mut self, added_gas: u32) -> () {
        let prev = self.total_gas;
        let gas_amount = u64::from(added_gas);
        let res = match prev.checked_add(gas_amount) {
            // gas charge overflow protection
            None => false,
            Some(val) if val > self.gas_limit => false,
            Some(val) => {
                self.total_gas = val;
                true
            }
        };
        if !res {
            println!("NO gassss");
            // Err("NO gassss".to_string())
        } else {
            ()
//            Ok(())
        }
    }
}


pub(crate) mod imports {
    use super::{Runtime, StrResult};

    use wasmer_runtime::{
        imports,
        func,
        ImportObject,
        Ctx,
    };

    macro_rules! wrapped_imports {
        ( $( $import_name:expr => $func_name:ident < [ $( $arg_name:ident : $arg_type:ident ),* ] -> [ $( $returns:ident ),* ] >, )* ) => {
            $(
                fn $func_name( $( $arg_name: $arg_type, )* ctx: &mut Ctx) -> ($( $returns )*) {
                    let runtime: &mut Runtime = unsafe { &mut *(ctx.data as *mut Runtime) };
                    runtime.$func_name( $( $arg_name, )* )
                }
            )*

            pub(crate) fn build() -> ImportObject {
                imports! {
                    "env" => {
                        $(
                            $import_name => func!($func_name),
                        )*
                    },
                }
            }
        }
    }

    wrapped_imports! {
        "print_str" => print_str<[ptr: u32, len: u32, aaa: i32] -> []>,
        "gas" => gas<[added_gas: u32] -> []>,
    }

}
