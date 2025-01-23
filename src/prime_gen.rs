use rand::Rng;
use rayon::iter::ParallelIterator;
use std::collections::HashMap;
use std::sync::{Arc, Mutex, MutexGuard};
use crate::utils::{are_coprime, is_probably_prime};

/// Génère un nombre premier aléatoire entre min et max
/// # Arguments
/// * `min` - Le minimum de la plage de recherche
/// * `max` - Le maximum de la plage de recherche
/// # Returns
/// * `u64` - Un nombre premier aléatoire entre min et max
pub fn prime_gen(min: u64, max: u64) -> u64 {
    assert!(
        min <= max,
        "Le minimum doit être inférieur ou égal au maximum"
    );

    let tested_numbers: Arc<Mutex<HashMap<u64, bool>>> = Arc::new(Mutex::new(HashMap::new()));

    rayon::iter::repeat(())
        .find_map_any(|()| {
            // on génère un nombre aléatoire entre min et max
            let num: u64 = rand::thread_rng().gen_range(min..max);

            let mut tested: MutexGuard<HashMap<u64, bool>> = tested_numbers.lock().unwrap();

            // on vérifie si le nombre generé a déjà été testé ou si on a testé tous les nombres entre min et max
            if tested.contains_key(&num) || tested.len() >= usize::try_from(max - min).unwrap() {
                return None;
            }

            if is_prime(num) {
                return Some(num);
            }

            tested.insert(num, false);
            None
        })
        .unwrap_or_else(|| panic!("No prime number found in range"))
}

pub fn prime_gen_probably_and_coprime(min : u64, max : u64, nb : u128) -> u128{
    assert!(
        min <= max,
        "Le minimum doit être inférieur ou égal au maximum"
    );

    let tested_numbers: Arc<Mutex<HashMap<u64, bool>>> = Arc::new(Mutex::new(HashMap::new()));

    rayon::iter::repeat(())
        .find_map_any(|()| {
            // on génère un nombre aléatoire entre min et max
            let num: u64 = rand::thread_rng().gen_range(min..max);

            let mut tested: MutexGuard<HashMap<u64, bool>> = tested_numbers.lock().unwrap();

            // on vérifie si le nombre generé a déjà été testé ou si on a testé tous les nombres entre min et max
            if tested.contains_key(&num) || tested.len() >= usize::try_from(max - min).unwrap() {
                return None;
            }

            if is_probably_prime(u128::from(num)) && are_coprime(u128::from(num), nb) {
                return Some(num);
            }

            tested.insert(num, false);
            None
        })
        .unwrap_or_else(|| panic!("No prime number found in range")) as u128
}

/// Vérifie si un nombre u64 est premier
/// # Arguments
/// * `n` - Le nombre à vérifier
/// # Returns
/// * `bool` - true si le nombre est premier, sinon false
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

pub trait PrimeGen {
    #[allow(dead_code)]
    fn prime_gen(min: Self, max: Self) -> Self;
    #[allow(dead_code)]
    fn is_prime(n: u128) -> bool;
}
impl PrimeGen for u128 {
    fn prime_gen(min: u128, max: u128) -> u128 {
        assert!(min <= max, "min should be less than max");

        let tested_numbers: Arc<Mutex<HashMap<u128, bool>>> = Arc::new(Mutex::new(HashMap::new()));

        rayon::iter::repeat(())
            .find_map_any(|()| {
                // on génère un nombre aléatoire entre min et max
                let num: u128 = rand::thread_rng().gen_range(min..max);

                let mut tested: MutexGuard<HashMap<u128, bool>> = tested_numbers
                    .lock()
                    .unwrap_or_else(|_| panic!("MutexGuard error"));

                // on vérifie si le nombre generé a déjà été testé ou si on a testé tous les nombres entre min et max
                if tested.contains_key(&num) || tested.len() >= usize::try_from(max - min).unwrap()
                {
                    return None;
                }

                if u128::is_prime(num) {
                    return Some(num);
                }

                tested.insert(num, false);
                None
            })
            .unwrap_or_else(|| panic!("No prime number found in range"))
    }

    fn is_prime(n: u128) -> bool {
        if n < 2 {
            return false;
        }
        if n == 2 {
            return true;
        }
        if n % 2 == 0 {
            return false;
        }
        let mut i: u128 = 3;
        while i * i <= n {
            if n % i == 0 {
                return false;
            }
            i += 2;
        }
        true
    }
}
