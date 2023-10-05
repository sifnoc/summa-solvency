use halo2_proofs::halo2curves::bn256::Fr as Fp;
use wasm_bindgen::prelude::*;

use summa_solvency::merkle_sum_tree::utils::{big_int_to_fp, big_intify_username, poseidon_entry};

pub fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    let user_name = big_intify_username("dxGaEAii");
    let balances = [Fp::from(11888), Fp::from(41163)];

    let result = poseidon_entry::<2>(big_int_to_fp(&user_name), balances);
    alert(&format!("hash output is : {:?}", result));
}

fn main() {
    greet();
}
