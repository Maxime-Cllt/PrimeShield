use std::ops::{MulAssign};
use num_bigint::BigInt;

/// Exponentiation rapide : base^exponent avec base et exponent en u32
pub fn fast_exponentiation(base: u128, exponent: u16) -> BigInt {
    let mut result: BigInt = BigInt::from(1u64);
    let mut base : BigInt = BigInt::from(base);
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