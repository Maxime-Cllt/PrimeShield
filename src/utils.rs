/// Calcule le PGCD de deux nombres
/// # Arguments
/// * `a` - Le premier nombre
/// * `b` - Le deuxième nombre
pub fn pgcd(a: u64, b: u64) -> u64 {
    let mut a: u64 = a;
    let mut b: u64 = b;
    while b != 0 {
        let r: u64 = a % b;
        a = b;
        b = r;
    }
    a
}

/// Vérifie si deux nombres sont premiers entre eux
/// # Arguments
/// * `a` - Le premier nombre
/// * `b` - Le deuxième nombre
/// # Returns
/// * `true` si les deux nombres sont premiers entre eux, `false` sinon
pub fn are_coprime(a: u64, b: u64) -> bool {
    pgcd(a, b) == 1
}
