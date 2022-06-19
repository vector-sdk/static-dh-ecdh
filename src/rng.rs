#![allow(missing_docs)]

use num_bigint_dig::BigUint;
use rand;
use rand::Rng;
use rand::SeedableRng;
use rand::rngs::SmallRng;
use rand::RngCore;
use lazy_static::lazy_static;
use spin::Mutex;

extern crate alloc;
use alloc::vec::Vec;

// TODO:
// Currently using PRNG SmallRng with const seed with no_std environment.
// Replace with something better.

lazy_static! {
    static ref SEED: Mutex<[u8; 32]> = Mutex::new([
        0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,
        17,18,19,20,21,22,23,24,25,26,27,28,29,30,31]);
}

pub fn no_std_rng_gen_biguint(bitsize: usize) -> BigUint {
    let mut small_rng = SmallRng::from_seed(*SEED.lock());
    let mut digits = Vec::new();
    let size = bitsize / 32;
    for _i in 0..size {
        let val: u32 = small_rng.next_u32();
        digits.push(val);
    }
    match small_rng.try_fill(&mut SEED.lock()[..]) {
        Ok(_) => (),
        Err(e) => panic!("RNG error: {}", e)
    }
    BigUint::new(digits)
}

