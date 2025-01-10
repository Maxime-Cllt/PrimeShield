use crate::prime_gen::prime_gen;
use crate::utils::are_coprime;

pub fn inverse_modular(e :u64, p : u64, q : u64) -> u64{
    let modulo = (p - 1) * (q - 1);

    loop {
        let prime = prime_gen(2, u64::MAX);

        if are_coprime(prime, modulo) {

            if((prime as u128) * (e as u128)) % modulo as u128 == 1 {
                return prime;
            }else { continue; }

        }else { continue; }
    }

}