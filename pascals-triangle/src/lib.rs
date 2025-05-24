use std::{collections::HashMap, iter::once};

pub struct PascalsTriangle {
    data: Vec<Vec<u32>>,
    memo: HashMap<u32, Vec<u32>>,
}

impl PascalsTriangle {
    pub fn new(count: u32) -> Self {
        let mut s = Self {
            data: Vec::new(),
            memo: HashMap::new(),
        };

        s.data = (1..=count).map(|i| s.make_triangle(i)).collect();
        s
    }

    pub fn make_triangle(&mut self, at: u32) -> Vec<u32> {
        if let Some(row) = self.memo.get(&at) {
            return row.clone();
        }

        let row = match at {
            1 => vec![1],
            _ => once(1)
                .chain(
                    self.make_triangle(at - 1)
                        .windows(2)
                        .map(|pair| pair[0] + pair[1]),
                )
                .chain(once(1))
                .collect(),
        };

        self.memo.insert(at, row.clone());
        row
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        self.data.clone()
    }
}
