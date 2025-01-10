use num_format::{Locale, ToFormattedString};
use crate::exponential_fast::*;
use crate::inverse_modular::*;
use crate::prime_gen::*;
use crate::utils::*;

#[test]
fn test_pgcd() {
    assert_eq!(pgcd(12, 18), 6);
    assert_eq!(pgcd(15, 8), 1);
    assert_eq!(pgcd(18, 12), 6);
    assert_eq!(pgcd(0, 18), 18);
}

#[test]
fn test_are_co_prime() {
    assert!(!are_coprime(12, 18));
    assert!(are_coprime(12, 35));
    assert!(are_coprime(12, 37));
    assert!(!are_coprime(12, 39));
}

#[test]
fn test_exponential_fast() {
    assert_eq!(exponential_fast(2, 0), 1);
    assert_eq!(exponential_fast(2, 1), 2);
    assert_eq!(exponential_fast(2, 2), 4);
    assert_eq!(exponential_fast(2, 3), 8);
    assert_eq!(exponential_fast(2, 4), 16);
}

#[test]
fn test_exponential_fast_mod() {
    assert_eq!(exponential_fast_mod(2, 0, 3), 1);
    assert_eq!(exponential_fast_mod(2, 1, 3), 2);
    assert_eq!(exponential_fast_mod(2, 2, 3), 1);
    assert_eq!(exponential_fast_mod(2, 3, 3), 2);
    assert_eq!(exponential_fast_mod(2, 4, 3), 1);
}

#[test]
fn test_is_probably_prime() {
    const PRIME_TO_TEST: [u64; 11] = [3, 5, 7, 11, 13, 19, 241, 599, 1319, 3671, 7879];
    const NORMAL_TO_TEST: [u64; 7] = [10, 20, 30, 50, 1000, 55555, 45678];

    for &n in &PRIME_TO_TEST {
        assert!(is_probably_prime(n));
    }

    for &n in &NORMAL_TO_TEST {
        assert!(!is_probably_prime(n));
    }
}

#[test]
fn test_prime_gen() {
    const MIN: u64 = 1000;
    const MAX: u64 = 10000;

    assert_eq!(is_prime(3), true);
    assert_eq!(is_prime(5), true);
    assert_eq!(is_prime(1243093), true); //1243093 est un nombre premier

    let prime: u128 = prime_gen(MIN, MAX) as u128;
    assert_eq!(prime >= MIN as u128 && prime <= MAX as u128, true);
    assert_eq!(is_prime(prime as u64), true);
}

#[test]
fn test_e_is_prime_with() {
    assert!(e_is_prime_with(27, 5, 11));
    assert!(!e_is_prime_with(26, 5, 11));
    assert!(e_is_prime_with(3, 5, 11));
    assert!(!e_is_prime_with(2, 5, 11));
}

#[test]
fn test_inverse_modular() {
    const E: u128 = 27u128;
    const P: u128 = 5u128;
    const Q: u128 = 11u128;
    let start = std::time::Instant::now();
    let  d: Option<u64> = inverse_modular(E as u64, P as u64, Q as u64);
    let duration = start.elapsed();
    println!("Time elapsed in inverse_modular() is: {:?}", duration);
    assert_eq!(d.is_some(), true);

    let d: u128 = d.unwrap() as u128;
    println!("d: {}", d.to_formatted_string(&Locale::fr));

    let ed: u128 = E * d;
    let modulo: u128 = (P - 1) * (Q - 1);
    let res: u128 = ed % modulo;
    assert_eq!(res, 1);
}
