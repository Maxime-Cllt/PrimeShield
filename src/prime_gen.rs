use rand::Rng;
use std::collections::HashMap;

/// Génère un nombre premier aléatoire entre min et max
/// # Arguments
/// * `min` - La borne inférieure
/// * `max` - La borne supérieure
/// # Returns
/// * Un nombre premier aléatoire entre min et max
pub fn prime_gen(min: u64, max: u64) -> u64 {
    assert!(
        (min <= max),
        "Le minimum doit être inférieur ou égal au maximum"
    );

    let mut map_no_prime: HashMap<u64, bool> = HashMap::new();

    // on génère un nombre aléatoire entre min et max
    loop {
        let mut num: u64 = rand::thread_rng().gen_range(min..max);

        // on vérifie si le nombre generé a déjà été testé ou si on a testé tous les nombres entre min et max
        while map_no_prime.contains_key(&num)
            && map_no_prime.len() < usize::try_from(max - min).unwrap()
        {
            num = rand::thread_rng().gen_range(min..max);
        }

        if is_prime(num) {
            return num;
        }

        map_no_prime.insert(num, true); // on ajoute le nombre à la map des nombres non premiers pour ne pas le retester
    }
}

/// Vérifie si un nombre est premier
/// # Arguments
/// * `n` - Le nombre à tester
/// # Returns
/// * `true` si `n` est premier, `false` sinon
pub fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    if n == 2 {
        return true;
    }
    if n % 2 == 0 {
        return false;
    }
    let mut i: u64 = 3;
    while i * i <= n {
        if n % i == 0 {
            return false;
        }
        i += 2;
    }
    return true;
}
