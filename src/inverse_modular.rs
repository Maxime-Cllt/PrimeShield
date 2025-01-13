use rayon::prelude::*;
use std::u128;

pub fn inverse_modular(e: u64, phi_n: u128) -> u128 {
    // Nous allons diviser l'espace de recherche en plusieurs parties
    let num_threads = rayon::current_num_threads() as u128;
    let chunk_size = u128::from(u64::MAX) / num_threads;

    // Utilisation de `par_iter` pour parcourir les chunks en parallèle
    (0..num_threads)
        .into_par_iter()
        .find_map_any(|i| {
            let start = i * chunk_size + 2; // Commencer à partir de 2
            let end = if i == num_threads - 1 {
                phi_n // Le dernier chunk peut être plus grand
            } else {
                (i + 1) * chunk_size
            };

            // Recherche de l'inverse modulaire dans le chunk courant
            for d in start..=end {
                if (d * e as u128) % phi_n == 1 {
                    return Some(d);
                }
            }
            None
        })
        .unwrap_or_else(|| panic!("No modular inverse found"))
}
