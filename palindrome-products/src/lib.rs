use std::collections::{BTreeSet, HashSet};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Palindrome {
    // prod: (u64, u64),
    prod:HashSet<(u64, u64)>,
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
    let tree: BTreeSet<_> = (min..max).map(|x| (x, x)).fold(
        BTreeSet::new() as BTreeSet<(u64, (u64, u64))>,
        |mut acc, (x, y)| {
            acc.insert((x * y, (x, y)));
            acc
        },
    );

    // find first palindrome from left and from right
    let min = tree
        .iter()
        .find(|(x, _)| is_palindrome(x))
        .map(|(value, prod)| Palindrome { value: *value, prod: *prod });

    let mut it = tree.iter();
    let max = std::iter::from_fn(|| it.next_back())
        .find(|(x, _)| is_palindrome(x))
        .map(|(value, prod)| Palindrome { value: *value, prod: *prod });

    println!("min: {:?}", min);
    println!("max: {:?}", max);


    min.zip(max)
}

fn is_palindrome(n: &u64) -> bool {
    if n.lt(&10) {
        return true;
    }

    let s = n.to_string();
    s.chars().eq(s.chars().rev())
}
