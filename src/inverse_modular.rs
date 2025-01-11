use crate::prime_gen::{prime_gen, PrimeGen};
use crate::utils::are_coprime;
use rayon::prelude::*;
use std::sync::{Arc, Mutex, MutexGuard};

pub fn inverse_modular(e: u64, p: u64, q: u64) -> u128 {
    let modulo: u128 = (u128::from(p) - 1) * (u128::from(q) - 1);
    let always_tested: Arc<Mutex<Vec<u128>>> = Arc::new(Mutex::new(Vec::new()));

    rayon::iter::repeat(()).find_map_any(|()| {
        let prime: u128 = u128::prime_gen(2, u64::MAX as u128);

        let mut tested: MutexGuard<Vec<u128>> = always_tested.lock().unwrap();
        if tested.contains(&prime) {
            return None;
        }

        if are_coprime(u128::from(prime), u128::from(modulo))
            && ((u128::from(prime)) * (u128::from(e))) % u128::from(modulo) == 1
        {
            return Some(prime);
        }

        tested.push(prime);
        None
    }).unwrap_or_else(|| panic!("No prime number found in range"))
}
