use std::iter::once;

pub struct PascalsTriangle {
    count: usize,
}

impl PascalsTriangle {
    pub fn new(count: usize) -> Self {
        Self { count }
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        let mut out: Vec<Vec<u32>> = Vec::with_capacity(self.count);
        let mut prev = vec![];

        if self.count >= 1 {
            out.push(vec![1]);
            prev = vec![1];
        }

        (1..self.count)
            .fold((out, prev), |(mut acc, prev), _| {
                let current: Vec<u32> = once(1)
                    .chain(prev.windows(2).map(|pair| pair[0] + pair[1]))
                    .chain(once(1))
                    .collect();

                acc.push(current.clone());

                (acc, current)
            })
            .0
    }
}
