/// Calcule l'exponentiation rapide de g^x mod n
/// # Arguments
/// * `g` - La base
/// * `x` - L'exposant
/// * `modu` - Le modulo
/// # Returns
/// * Le résultat de l'exponentiation rapide
pub fn exponential_fast_mod(g: u128, x: u128, modu: u128) -> u128 {
    let mut aux: u128 = g % modu; // Base initiale mod n
    let mut output: u128 = 1u128;
    let mut x: u128 = x;

    while x != 0 {
        if x & 1 == 1 {
            output = (output * aux) % modu;
        }
        x >>= 1; // Opération de division par 2
        aux = (aux * aux) % modu; // Mise à jour de la base
    }
    output
}
