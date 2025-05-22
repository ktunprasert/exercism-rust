fn collatz_re(n: u64, times: u64) -> u64 {
    match true {
        _ if n == 1 => times,
        _ if (n & 1 == 0) => collatz_re(n / 2, times + 1),
        _ => collatz_re(3 * n + 1, times + 1),
    }
}

pub fn collatz(n: u64) -> Option<u64> {
    if n == 0 {
        return None;
    }

    Some(collatz_re(n, 0))
}
