use rand::Rng;

pub fn prime_gen(min: u64, max: u64) -> u64 {
    if min > max {
        panic!("min should be less than max");
    }

    let mut tests_vec = vec![];

    // on génère un nombre aléatoire entre min et max
    loop {

        let mut num = rand::thread_rng().gen_range(min..max);

        // on vérifie si le nombre generé a déjà été testé ou si on a testé tous les nombres entre min et max
        while tests_vec.contains(&num) && tests_vec.len() < (max - min) as usize {
            num = rand::thread_rng().gen_range(min..max);
        }

        if is_prime(num) {
            return num;
        } else {
            tests_vec.push(num);
            continue;
        };

    };
}


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
    return true;
}