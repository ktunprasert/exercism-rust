use std::collections::{BTreeMap, HashSet};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Palindrome {
    // prod: (u64, u64),
    prod: HashSet<(u64, u64)>,
    value: u64,
}

impl Palindrome {
    pub fn value(&self) -> u64 {
        self.value
    }

    pub fn into_factors(self) -> HashSet<(u64, u64)> {
        self.prod
    }
}

pub fn palindrome_products(min: u64, max: u64) -> Option<(Palindrome, Palindrome)> {
    let mut prods: BTreeMap<u64, HashSet<(u64, u64)>> = BTreeMap::new();

    for x in min..=max {
        for y in min..=max {
            prods
                .entry(x * y)
                .or_insert_with(HashSet::new)
                .insert((x.min(y), x.max(y)));
        }
    }

    let min = prods
        .iter()
        .find(|(x, _)| is_palindrome(x))
        .map(|(&value, prod)| Palindrome {
            value,
            prod: prod.clone(),
        });

    let max = prods
        .iter()
        .rev()
        .find(|(x, _)| is_palindrome(x))
        .map(|(&value, prod)| Palindrome {
            value,
            prod: prod.clone(),
        });

    min.zip(max)
}

fn is_palindrome(n: &u64) -> bool {
    if n.lt(&10) {
        return true;
    }

    let s = n.to_string();
    s.chars().eq(s.chars().rev())
}
