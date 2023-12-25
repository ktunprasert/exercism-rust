use std::collections::HashSet;

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    factors
        .iter()
        .filter(|&&x| x != 0)
        .fold(HashSet::new(), |mut acc: HashSet<u32>, &x| {
            (x..limit).step_by(x as usize).into_iter().for_each(|x| {
                acc.insert(x);
            });

            acc
        })
        .iter()
        .sum()
}
