pub fn egg_count(dv: u32) -> usize {
    match dv {
        0 => 0,
        _ if dv & 1 == 1 => 1 + egg_count(dv >> 1),
        _ => egg_count(dv >> 1),
    }
}
