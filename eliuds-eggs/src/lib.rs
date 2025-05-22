pub fn egg_count(dv: u32) -> usize {
    match dv {
        0 => 0,
        _ => 1 + egg_count(dv & (dv - 1)),
    }
}
