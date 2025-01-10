use crate::exponential_fast::*;
use crate::inverse_modular::*;
use crate::utils::*;
use crate::prime_gen::*;

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

    assert_eq!(is_prime(3), true);
    assert_eq!(is_prime(5), true);
    assert_eq!(is_prime(1243093), true); //1243093 est un nombre premier

    let min = 1000;
    let max = 10000;
    let prime = prime_gen(min, max);
    assert_eq!(prime >= min && prime <= max, true);
    assert_eq!(is_prime(prime), true);
}


#[test]
fn test_inverse_modular() {
    let e = 27u128;
    let p = 5u128;
    let q = 11u128;
    let d = inverse_modular(e as u64, p as u64, q as  u64) as u128;

    let ed = e * d;
    let modulo = (p - 1) * (q - 1);
    let res = ed % modulo;
    assert_eq!(res, 1);
}