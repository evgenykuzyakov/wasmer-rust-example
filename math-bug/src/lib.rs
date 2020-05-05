#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

use uint::construct_uint;

construct_uint! {
    /// 256-bit unsigned integer.
    pub struct U256(4);
}

construct_uint! {
    /// 512-bit unsigned integer.
    pub struct U512(8);
}

#[no_mangle]
pub extern "C" fn multiply_4() {
    let a = U256::from(std::u128::MAX);
    let b = U256::from(2u64);
    let c = a * b;
    assert_eq!(c.0, [18446744073709551614u64, 18446744073709551615u64, 1, 0]);
}


#[no_mangle]
pub extern "C" fn multiply_8() {
    let a = U512::from(std::u128::MAX);
    let b = U512::from(2u64);
    let c = a * b;
    assert_eq!(c.0, [18446744073709551614u64, 18446744073709551615u64, 1, 0, 0, 0, 0, 0]);
}

#[cfg(not(target_arch = "wasm32"))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_multiply_4() {
        multiply_4()
    }

    #[test]
    pub fn test_multiply_8() {
        multiply_8()
    }
}
