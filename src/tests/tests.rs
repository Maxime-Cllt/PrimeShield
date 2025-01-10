use crate::utils::{are_coprime, pgcd};

#[test]
fn test_pgcd() {
    assert_eq!(pgcd(12, 18), 6);
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
