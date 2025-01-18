use crate::fast_exponentiation::exponential_fast_mod;

/// Calcule le PGCD de deux nombres
/// # Arguments
/// * `a` - Le premier nombre
/// * `b` - Le deuxième nombre
#[allow(dead_code)]
pub fn pgcd(a: u128, b: u128) -> u128 {
    let mut a: u128 = a;
    let mut b: u128 = b;
    while b != 0 {
        let r: u128 = a % b;
        a = b;
        b = r;
    }
    return a;
}

/// Vérifie si deux nombres sont premiers entre eux
/// # Arguments
/// * `a` - Le premier nombre
/// * `b` - Le deuxième nombre
/// # Returns
/// * `true` si les deux nombres sont premiers entre eux, `false` sinon
#[allow(dead_code)]
pub fn are_coprime(a: u128, b: u128) -> bool {
    return pgcd(a, b) == 1;
}

/// Vérifie si un nombre est probablement premier en utilisant les bases 2, 3, 5 et 7
/// # Arguments
/// * `n` - Le nombre à tester
/// # Returns
/// * `true` si `n` est probablement premier, `false` sinon
#[allow(dead_code)]
pub fn is_probably_prime(n: u64) -> bool {
    const BASES: [u128; 4] = [2, 3, 5, 7]; // Bases utilisées pour le test de primalité

    let n: u128 = u128::from(n);

    if n < 2 {
        return false;
    }

    // Vérifie si n est divisible par une des bases (autre qu'elle-même)
    for &base in &BASES {
        if n > base && n % base == 0 {
            return false;
        }
    }

    // Vérifie la condition (base^(n-1) % n == 1) pour chaque base
    for &base in &BASES {
        if base < n && exponential_fast_mod(base, u64::try_from(n - 1).unwrap(), n) != 1 {
            return false;
        }
    }

    return true;
}

/// Calcule si e premier avec (p − 1)(q − 1)
/// # Arguments
/// * `e` - Le nombre à tester
/// * `p` - Le premier nombre
/// * `q` - Le deuxième nombre
/// # Returns
/// * `true` si `e` est premier avec (p − 1)(q − 1), `false` sinon
#[allow(dead_code)]
pub fn e_is_prime_with(e: u128, p: u128, q: u128) -> bool {
    return are_coprime(e, (p - 1) * (q - 1));
}
