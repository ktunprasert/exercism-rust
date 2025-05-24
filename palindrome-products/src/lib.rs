use std::collections::{BTreeMap, HashSet};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Palindrome {
    // prod: (u64, u64),
    // prod: HashSet<(u64, u64)>,
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
    // Search for min palindrome (start from smallest product)
    let mut min_palindrome = None;
    for prod in (min * min)..=(max * max) {
        if is_palindrome(prod) {
            let pairs = factor_pairs(prod, min, max);
            if !pairs.is_empty() {
                min_palindrome = Some(Palindrome {
                    value: prod,
                    prod: pairs,
                });
                break;
            }
        }
    }

    // Search for max palindrome (start from largest product)
    let mut max_palindrome = None;
    for prod in (min * min..=max * max).rev() {
        if is_palindrome(prod) {
            let pairs = factor_pairs(prod, min, max);
            if !pairs.is_empty() {
                max_palindrome = Some(Palindrome {
                    value: prod,
                    prod: pairs,
                });
                break;
            }
        }
    }

    min_palindrome.zip(max_palindrome)
}

fn is_palindrome(n: u64) -> bool {
    if n.lt(&10) {
        return true;
    }

    let s = n.to_string();
    s.chars().eq(s.chars().rev())
}
