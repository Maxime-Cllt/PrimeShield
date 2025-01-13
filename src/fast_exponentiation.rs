use num_bigint::BigInt;
use rayon::prelude::*;
use std::ops::MulAssign;

/// Exponentiation rapide : base^exponent avec base et exponent en u32
pub fn fast_exponentiation(base: u128, exponent: u32) -> BigInt {
    let mut result: BigInt = BigInt::from(1u64);
    let mut base: BigInt = BigInt::from(base);
    let mut exponent: BigInt = BigInt::from(exponent);

    while !exponent.eq(&BigInt::from(0u64)) {
        // Si l'exposant est impair, multiplier le résultat par la base
        if (&exponent & BigInt::from(1u64)).eq(&BigInt::from(1u64)) {
            result.mul_assign(base.clone());
        }
        // Élever la base au carré
        base.mul_assign(base.clone());
        // Diviser exponent par 2 (décalage à droite)
        exponent >>= 1;
    }

    result
}

/// Exponentiation rapide parallélisée
pub fn parallel_fast_exponentiation(base: u128, exponent: u32) -> BigInt {
    let num_threads = rayon::current_num_threads() as u128;
    let chunk_size = u128::from(exponent) / num_threads;
    let remainder = u128::from(exponent) % num_threads;

    // Calculer les résultats partiels en parallèle
    let partial_results: Vec<BigInt> = (0..num_threads)
        .into_par_iter()
        .map(|i| {
            let start = i * chunk_size;
            let end = if i == num_threads - 1 {
                start + chunk_size + remainder
            } else {
                start + chunk_size
            };
            fast_exponentiation(base, (end - start) as u32)
        })
        .collect();

    // Combiner les résultats partiels
    partial_results.into_iter().fold(BigInt::from(1u64), |acc, x| acc * x)
}