use rayon::prelude::*;

#[allow(dead_code)]
/// Calcule l'inverse modulaire de `e` modulo `phi_n`.
/// # Arguments
/// * `e` - La clé publique `e`.
/// * `phi_n` - La valeur de la fonction d'Euler de `n`.
/// # Returns
/// L'inverse modulaire de `e` modulo `phi_n`.
pub fn inverse_modular(e: u64, phi_n: u128) -> u128 {
    // Nous allons diviser l'espace de recherche en plusieurs parties
    let num_threads: u128 = rayon::current_num_threads() as u128;
    let chunk_size: u128 = u128::from(u64::MAX) / num_threads;
    let e: u128 = u128::from(e);

    // Utilisation de `par_iter` pour parcourir les chunks en parallèle
    (0..num_threads)
        .into_par_iter()
        .find_map_any(|i| {
            let start: u128 = i * chunk_size + 2; // Commencer à partir de 2
            let end: u128 = if i == num_threads - 1 {
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
        let x: i128 = y1 - (b / a) * x1;
        let y: i128 = x1;
        (gcd, x, y)
    }
}

// algorithme de recherche de l'inverse modulaire par l'algorithme d'Euclide étendu
pub fn inverse_modular_fast(a: u128, m: u128) -> Option<u128> {
    let a: i128 = i128::try_from(a).unwrap();
    let m: i128 = i128::try_from(m).unwrap();

    let (gcd, x, _) = extended_gcd(a, m);
    if gcd == 1 {
        Some(((x % m + m) % m) as u128) // Pour s'assurer que le résultat est positif
    } else {
        None // L'inverse n'existe pas si a et m ne sont pas premiers entre eux
    }
}
