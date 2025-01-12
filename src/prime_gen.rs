use std::sync::{Arc, Mutex, MutexGuard};
use rand::Rng;
use rayon::iter::ParallelIterator;

// on génère un nombre aléatoire entre min et max
pub fn prime_gen(min: u64, max: u64) -> u64 {
    assert!(
        (min <= max),
        "Le minimum doit être inférieur ou égal au maximum"
    );

    let tested_numbers: Arc<Mutex<Vec<u64>>> = Arc::new(Mutex::new(Vec::new()));

    rayon::iter::repeat(())
        .find_map_any(|()| {

            // on génère un nombre aléatoire entre min et max
            let num: u64 = rand::thread_rng().gen_range(min..max);

            let mut tested: MutexGuard<Vec<u64>> = tested_numbers.lock().unwrap();

            // on vérifie si le nombre generé a déjà été testé ou si on a testé tous les nombres entre min et max
            if tested.contains(&num) || tested.len() >= usize::try_from(max - min).unwrap() {
                return None;
            }

            if is_prime(num) {
                return Some(num);
            }

            tested.push(num);
            None
        })
        .unwrap_or_else(|| panic!("No prime number found in range"))
}


pub trait PrimeGen {
    fn prime_gen(min: Self, max: Self) -> Self;
    fn is_prime(n: u128) -> bool;
}
impl PrimeGen for u128 {
    fn prime_gen(min: u128, max: u128) -> u128 {
        assert!((min <= max), "min should be less than max");

        let tested_numbers: Arc<Mutex<Vec<u128>>> = Arc::new(Mutex::new(Vec::new()));

        rayon::iter::repeat(())
            .find_map_any(|()| {

                // on génère un nombre aléatoire entre min et max
                let num: u128 = rand::thread_rng().gen_range(min..max);

                let mut tested: MutexGuard<Vec<u128>> = tested_numbers.lock().unwrap_or_else(|_| panic!("MutexGuard error"));

                // on vérifie si le nombre generé a déjà été testé ou si on a testé tous les nombres entre min et max
                if tested.contains(&num) || tested.len() >= usize::try_from(max - min).unwrap() {
                    return None;
                }

                if u128::is_prime(num) {
                    return Some(num);
                }

                tested.push(num);
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
        let mut i = 3;
        while i * i <= n {
            if n % i == 0 {
                return false;
            }
            i += 2;
        }
        true
    }

}

// on vérifie si un nombre est premier
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
    let mut i = 3;
    while i * i <= n {
        if n % i == 0 {
            return false;
        }
        i += 2;
    }
    true
}
