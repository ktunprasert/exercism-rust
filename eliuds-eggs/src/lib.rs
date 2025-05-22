pub fn egg_count(display_value: u32) -> usize {
    let mut eggs: u32 = 0;
    let mut dp = display_value;

    while dp > 0 {
        eggs += dp & 1;
        dp >>= 1;
    }

    eggs as usize
}
