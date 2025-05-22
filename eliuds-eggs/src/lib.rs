pub fn egg_count(dv: u32) -> usize {
    match dv {
        0 => 0,
        _ => (dv & 1) as usize + egg_count(dv >> 1),
    }
}
