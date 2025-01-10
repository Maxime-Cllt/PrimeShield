use crate::exponential_fast::exponential_fast;
use crate::utils::{are_coprime, is_probably_prime, pgcd};

#[test]
fn test_pgcd() {
    assert_eq!(pgcd(12, 18), 6);
    assert_eq!(pgcd(15, 8), 1);
    assert_eq!(pgcd(18, 12), 6);
    assert_eq!(pgcd(0, 18), 18);
}

#[test]
fn test_are_co_prime() {
    assert_eq!(are_coprime(12, 18), false);
    assert_eq!(are_coprime(12, 35), true);
    assert_eq!(are_coprime(12, 37), true);
    assert_eq!(are_coprime(12, 39), false);
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
fn test_is_probably_prime() {
    const PRIME_TO_TEST: [u64; 11] = [3, 5, 7, 11, 13, 19, 241, 599, 1319, 3671, 7879];
    const NORMAL_TO_TEST: [u64; 7] = [10, 20, 30, 50, 1000, 55555, 45678];

    for &n in &PRIME_TO_TEST {
        assert_eq!(is_probably_prime(n), true);
    }

    for &n in &NORMAL_TO_TEST {
        assert_eq!(is_probably_prime(n), false);
    }
}
