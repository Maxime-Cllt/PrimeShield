use rayon::prelude::*;
use std::u128;


// algorithme de recherche de l'inverse modulaire par force brute
#[allow(dead_code)]

pub fn inverse_modular(e: u64, phi_n: u128) -> u128 {
    // Nous allons diviser l'espace de recherche en plusieurs parties
    let num_threads = rayon::current_num_threads() as u128;
    let chunk_size = u128::from(u64::MAX) / num_threads;
    let e = e as u128;

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
                if (d * e) % phi_n == 1 {
                    return Some(d);
                }
            }
            None
        })
        .unwrap_or_else(|| panic!("No modular inverse found"))
}


#[allow(dead_code)]
fn extended_gcd(a: i128, b: i128) -> (i128, i128, i128) {

    if a == 0 {
        (b, 0, 1)
    } else {
        let (gcd, x1, y1) = extended_gcd(b % a, a);
        let x = y1 - (b / a) * x1;
        let y = x1;
        (gcd, x, y)
    }
}

// algorithme de recherche de l'inverse modulaire par l'algorithme d'Euclide étendu
pub fn inverse_modular_fast(a: u128, m: u128) -> Option<u128> {

    let a = a as i128;
    let m = m as i128;

    let (gcd, x, _) = extended_gcd(a, m);
    if gcd == 1 {
        Some(((x % m + m) % m) as u128) // Pour s'assurer que le résultat est positif
    } else {
        None // L'inverse n'existe pas si a et m ne sont pas premiers entre eux
    }
}
