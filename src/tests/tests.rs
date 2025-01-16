use crate::fast_exponentiation::*;
use crate::inverse_modular::*;
use crate::prime_gen::*;
use crate::utils::*;
use num_format::{Locale, ToFormattedString};
use std::time::Instant;

#[tokio::test]
async fn test_pgcd() {
    assert_eq!(pgcd(12, 18), 6);
    assert_eq!(pgcd(15, 8), 1);
    assert_eq!(pgcd(18, 12), 6);
    assert_eq!(pgcd(0, 18), 18);
}

#[tokio::test]
async fn test_are_co_prime() {
    assert!(!are_coprime(12, 18));
    assert!(are_coprime(12, 35));
    assert!(are_coprime(12, 37));
    assert!(!are_coprime(12, 39));
}

#[tokio::test]
async fn test_exponential_fast_mod() {
    assert_eq!(exponential_fast_mod(2, 0, 3), 1);
    assert_eq!(exponential_fast_mod(2, 1, 3), 2);
    assert_eq!(exponential_fast_mod(2, 2, 3), 1);
    assert_eq!(exponential_fast_mod(2, 3, 3), 2);
    assert_eq!(exponential_fast_mod(2, 4, 3), 1);
}

#[tokio::test]
async fn test_is_probably_prime() {
    const PRIME_TO_TEST: [u64; 11] = [3, 5, 7, 11, 13, 19, 241, 599, 1319, 3671, 7879];
    const NORMAL_TO_TEST: [u64; 7] = [10, 20, 30, 50, 1000, 55555, 45678];

    for &n in &PRIME_TO_TEST {
        assert!(is_probably_prime(n));
    }

    for &n in &NORMAL_TO_TEST {
        assert!(!is_probably_prime(n));
    }
}

#[tokio::test]
async fn test_prime_gen() {
    const MIN: u64 = 1000;
    const MAX: u64 = 10000;

    assert!(is_prime(3));
    assert!(is_prime(5));
    assert!(is_prime(1_243_093)); //1243093 est un nombre premier

    let prime: u128 = u128::from(prime_gen(MIN, MAX));
    assert!(prime >= u128::from(MIN) && prime <= u128::from(MAX));
    assert!(is_prime(u64::try_from(prime).unwrap()));
}

#[tokio::test]
async fn test_e_is_prime_with() {
    assert!(e_is_prime_with(27, 5, 11));
    assert!(!e_is_prime_with(26, 5, 11));
    assert!(e_is_prime_with(3, 5, 11));
    assert!(!e_is_prime_with(2, 5, 11));
}

#[tokio::test]
async fn test_inverse_modular() {
    const E: u128 = 27u128;
    const P: u128 = 5u128;
    const Q: u128 = 11u128;

    let phi_n: u128 = (P - 1) * (Q - 1);

    let start: Instant = Instant::now();
    let d: u128 = inverse_modular(u64::try_from(E).unwrap(), phi_n);
    println!(
        "Temps écoulé en inverse_modular() est: {:?}",
        start.elapsed()
    );

    println!("d: {}", d.to_formatted_string(&Locale::fr));

    let ed: u128 = E * u128::from(d);
    let res: u128 = ed % phi_n;
    assert_eq!(res, 1);
}

#[tokio::test]
async fn test_mod_inverse() {
    const E: u128 = 27u128;
    const P: u128 = 5u128;
    const Q: u128 = 11u128;

    let start: Instant = Instant::now();
    let d: Option<u128> = inverse_modular_fast(
        E, (P - 1) * (Q - 1),
    );
    println!(
        "Temps écoulé en test_mod_inverse() est: {:?}",
        start.elapsed()
    );

    let d: u128 = u128::try_from(d.unwrap()).unwrap();
    println!("d: {}", d.to_formatted_string(&Locale::fr));

    let ed: u128 = E * d;
    let modulo: u128 = (P - 1) * (Q - 1);
    let res: u128 = ed % modulo;
    assert_eq!(res, 1);
}


#[tokio::test]
async fn fast_exponentiation_fn_test() {
    let base: u128 = 106_190;
    let exponent: u32 = 119_863;
    let modulo = 839_040;

    let mut  start: Instant = Instant::now();

    let res = exponential_fast_mod(base, exponent as u64, modulo);

    println!(
        "Temps écoulé en fast_exponentiation_fn_test() est: {:?} {:?}",
        start.elapsed(), res
    );
}