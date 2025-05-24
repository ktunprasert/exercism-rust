use std::iter;

pub struct PascalsTriangle {
    data: Vec<Vec<u32>>,
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        Self { data: Self::init(row_count) }
    }

    fn init(count: u32) -> Vec<Vec<u32>> {
        if count == 0 {
            return vec![];
        }

        (1..=count).map(|i| Self::make_triangle(i)).collect()
    }

    fn make_triangle(at: u32) -> Vec<u32> {
        match at {
            1 => vec![1],
            _ => {
                iter::once(1)
                    .chain(
                        Self::make_triangle(at - 1)
                            .windows(2)
                            .map(|pair| pair[0] + pair[1]),
                    )
                    .chain(iter::once(1))
                    .collect()
            }
        }
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        self.data.clone()
    }
}
