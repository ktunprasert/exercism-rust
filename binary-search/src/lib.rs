use std::cmp::Ordering;

fn search<U: AsRef<[T]>, T: Ord>(array: U, key: T, idx: usize) -> Option<usize> {
    let array = array.as_ref();

    if array.is_empty() {
        return None;
    }

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

pub fn find<U: AsRef<[T]>, T: Ord>(array: U, key: T) -> Option<usize> {
    search(array, key, 0)
}
