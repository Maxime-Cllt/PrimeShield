use crate::prime_gen::prime_gen;
use crate::utils::are_coprime;
use rayon::prelude::*;
use std::sync::{Arc, Mutex};

pub fn inverse_modular(e: u64, p: u64, q: u64) -> Option<u64> {
    let modulo = (p - 1) * (q - 1);
    let always_tested = Arc::new(Mutex::new(Vec::new()));

    // Utilisation de rayon pour paralléliser la boucle
    let result = rayon::iter::repeat(())
        .find_map_any(|_| {
            let prime = prime_gen(2, u64::MAX);

            // Vérifier si ce nombre premier a déjà été testé
            let mut tested = always_tested.lock().unwrap();
            if tested.contains(&prime) {
                return None;
            }

            // Vérifier les conditions
            if are_coprime(prime, modulo) {
                if ((prime as u128) * (e as u128)) % modulo as u128 == 1 {
                    return Some(prime);
                }
            }

            // Ajouter ce nombre premier à la liste des nombres déjà testés
            tested.push(prime);
            None
        });

    result
}