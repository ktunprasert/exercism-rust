use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Palindrome {
    prod: Vec<(u64, u64)>,
    value: u64,
}

impl Palindrome {
    pub fn value(&self) -> u64 {
        self.value
    }

    pub fn into_factors(self) -> HashSet<(u64, u64)> {
        self.prod.into_iter().collect()
    }
}

// Helper to collect all distinct factor pairs (x, y) for a palindrome value in range
fn factor_pairs(n: u64, min: u64, max: u64) -> Vec<(u64, u64)> {
    let mut pairs = Vec::new();
    let sqrt = (n as f64).sqrt() as u64;
    for x in min..=max {
        if x > sqrt {
            break;
        }

        if n % x == 0 {
            let y = n / x;
            if y >= min && y <= max {
                pairs.push((x, y));
            }
        }
    }

    pairs
}

pub fn palindrome_products(min: u64, max: u64) -> Option<(Palindrome, Palindrome)> {
    let min_palindrome = (min * min..=(max * max))
        .into_iter()
        .filter(|prod| is_palindrome(*prod))
        .find_map(|prod| {
            let pairs = factor_pairs(prod, min, max);
            (!pairs.is_empty()).then_some(Palindrome {
                value: prod,
                prod: pairs,
            })
        });

    let max_palindrome = (min * min..=(max * max))
        .rev()
        .into_iter()
        .filter(|prod| is_palindrome(*prod))
        .find_map(|prod| {
            let pairs = factor_pairs(prod, min, max);
            (!pairs.is_empty()).then_some(Palindrome {
                value: prod,
                prod: pairs,
            })
        });

    min_palindrome.zip(max_palindrome)
}

fn is_palindrome(n: u64) -> bool {
    if n.lt(&10) {
        return true;
    }

    let s = n.to_string();
    s.chars().eq(s.chars().rev())
}
