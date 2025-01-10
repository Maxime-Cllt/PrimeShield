pub fn exponential_fast(g: u64, x: u64) -> u64 {
    let mut aux: u64 = g;
    let mut output: u64 = 1;
    let mut x: u64 = x;
    while x != 0 {
        if x & 1 == 1 {
            output *= aux;
        }
        x >>= 1;
        aux = aux * aux;
    }
    output
}
