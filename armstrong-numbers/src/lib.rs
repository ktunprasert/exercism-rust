pub fn is_armstrong_number(num: u64) -> bool {
    let pow = num.to_string().len() as u32;

    num == num
        .to_string()
        .chars()
        .fold(0, |sum, c| (c.to_digit(10).unwrap() as u64).pow(pow) + sum)
}
