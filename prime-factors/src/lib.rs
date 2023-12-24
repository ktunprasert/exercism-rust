pub fn factors(mut n: u64) -> Vec<u64> {
    if n < 2 {
        return vec![]
    }

    let mut prime_factors = vec![];
    let mut factor = 2;

    while n != 1 {
        if n % factor == 0 {
            prime_factors.push(factor);
            n /= factor;
        } else {
            factor += 1;
        }
    }

    prime_factors
}
