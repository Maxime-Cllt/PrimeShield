use crate::prime_gen::{PrimeGen};
use crate::utils::are_coprime;
use rayon::prelude::*;
use std::sync::{Arc, Mutex};

/// Calcule l'inverse modulaire de `e` modulo `(p-1)*(q-1)`
/// # Arguments
/// * `e` - Le nombre à inverser
/// * `p` - Le premier nombre
/// * `q` - Le deuxième nombre
/// # Returns
/// * `Some(u64)` si l'inverse modulaire existe, `None` sinon
pub fn inverse_modular(e: u64, p: u64, q: u64) -> u128 {
    let modulo: u128 = (u128::from(p) - 1) * (u128::from(q) - 1);
    let val: Arc<Mutex<u128>> = Arc::new(Mutex::new(2u128));

    rayon::iter::repeat(()).find_map_any(|()| {
        let test = val.lock().unwrap().clone();
        *val.lock().unwrap() += 1;

        if are_coprime(u128::from(test), u128::from(modulo))
            && ((u128::from(test)) * (u128::from(e))) % u128::from(modulo) == 1
        {
            return Some(test);
        }

        None
    }).unwrap_or_else(|| panic!("No prime number found in range"))
}