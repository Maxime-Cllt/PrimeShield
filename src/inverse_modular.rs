use crate::prime_gen::prime_gen;
use crate::utils::are_coprime;
use rayon::prelude::*;
use std::collections::HashMap;
use std::sync::{Arc, Mutex, MutexGuard};

/// Calcule l'inverse modulaire de `e` modulo `(p-1)*(q-1)`
/// # Arguments
/// * `e` - Le nombre à inverser
/// * `p` - Le premier nombre
/// * `q` - Le deuxième nombre
/// # Returns
/// * `Some(u64)` si l'inverse modulaire existe, `None` sinon
pub fn inverse_modular(e: u64, p: u64, q: u64) -> Option<u64> {
    let modulo: u64 = (p - 1) * (q - 1);
    let always_tested: Arc<Mutex<HashMap<u64, bool>>> = Arc::new(Mutex::new(HashMap::new()));

    let result: Option<u64> = rayon::iter::repeat(()).find_map_any(|()| {
        let prime: u64 = prime_gen(2, u64::MAX);

        let mut tested: MutexGuard<HashMap<u64, bool>> = always_tested.lock().unwrap();
        if tested.contains_key(&prime) {
            return None;
        }

        if are_coprime(prime, modulo)
            && ((u128::from(prime)) * (u128::from(e))) % u128::from(modulo) == 1
        {
            return Some(prime);
        }

        tested.insert(prime, false);
        None
    });

    result
}
