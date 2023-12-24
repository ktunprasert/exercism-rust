pub fn factors(n: u64) -> Vec<u64> {
    if n < 2 {
        return vec![]
    }

    let mut n_copy = n;

    let mut prime_factors = vec![];
    let mut factor = 2;

    while n_copy != 1 {
        if n_copy % factor == 0 {
            prime_factors.push(factor);
            n_copy /= factor;
        } else {
            factor += 1;
        }
    }

    prime_factors
}
