use std::iter::once;

pub struct PascalsTriangle {
    count: usize,
}

impl PascalsTriangle {
    pub fn new(count: usize) -> Self {
        Self { count }
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        if self.count == 0 {
            return vec![];
        }

        (1..self.count).fold(vec![vec![1]], |mut acc, _| {
            acc.push(
                once(1)
                    .chain(acc.last().unwrap().windows(2).map(|pair| pair[0] + pair[1]))
                    .chain(once(1))
                    .collect::<Vec<u32>>(),
            );

            acc
        })
    }
}
