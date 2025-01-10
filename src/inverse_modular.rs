use crate::prime_gen::prime_gen;
use crate::utils::are_coprime;
use rayon::prelude::*;
use std::sync::{Arc, Mutex, MutexGuard};

pub fn inverse_modular(e: u64, p: u64, q: u64) -> Option<u64> {
    let modulo: u64 = (p - 1) * (q - 1);
    let always_tested: Arc<Mutex<Vec<u64>>> = Arc::new(Mutex::new(Vec::new()));

    let result: Option<u64> = rayon::iter::repeat(()).find_map_any(|()| {
        let prime: u64 = prime_gen(2, u64::MAX);

        let mut tested: MutexGuard<Vec<u64>> = always_tested.lock().unwrap();
        if tested.contains(&prime) {
            return None;
        }

        if are_coprime(prime, modulo)
            && ((u128::from(prime)) * (u128::from(e))) % u128::from(modulo) == 1
        {
            return Some(prime);
        }

        tested.push(prime);
        None
    });

    result
}
