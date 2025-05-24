use std::cmp::Ordering;

fn search(array: &[i32], key: i32, idx: usize) -> Option<usize> {
    if array.len() == 1 && key != array[0] {
        return None;
    }

    let n = array.len() / 2;
    match key.cmp(&array[n]) {
        Ordering::Equal => Some(n + idx),
        Ordering::Less => search(&array[..n], key, idx.min(n)),
        Ordering::Greater => search(&array[n..], key, idx + n),
    }
}

pub fn find(array: &[i32], key: i32) -> Option<usize> {
    if array.is_empty() || array.len() == 0 {
        return None;
    }

    search(array, key, 0)
}
