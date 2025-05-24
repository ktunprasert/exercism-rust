use std::{collections::HashMap, iter};

pub struct PascalsTriangle {
    data: Vec<Vec<u32>>,
    memo: HashMap<u32, Vec<u32>>,
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        let mut s = Self {
            data: Vec::new(),
            memo: HashMap::new(),
        };

        s.init(row_count);
        s
    }

    fn init(&mut self, count: u32) {
        if count == 0 {
            self.data = vec![];
            return;
        }

        self.data = (1..=count)
            .map(|i| {
                let next = self.make_triangle(i);
                self.memo.insert(i, next.clone());

                next
            })
            .collect()
    }

    pub fn make_triangle(&self, at: u32) -> Vec<u32> {
        if let Some(row) = self.memo.get(&at) {
            return row.clone();
        }

        match at {
            1 => vec![1],
            _ => iter::once(1)
                .chain(
                    self.make_triangle(at - 1)
                        .windows(2)
                        .map(|pair| pair[0] + pair[1]),
                )
                .chain(iter::once(1))
                .collect(),
        }
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        self.data.clone()
    }
}
