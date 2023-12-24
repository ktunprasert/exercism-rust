pub fn nth(n: u32) -> u32 {
    let mut i = 3;
    let mut primes = vec![2];

    while primes.len() <= n as usize {
        if primes.iter().all(|&x| i % x != 0) {
            primes.push(i);
        }

        i += 2;
    }

    primes.last().unwrap().clone()
}
