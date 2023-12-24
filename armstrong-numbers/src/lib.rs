pub fn is_armstrong_number(num: u64) -> bool {
    let pow = if num == 0 { 1 } else { num.ilog10() + 1 };
    let mut sum: u64 = 0;
    let mut n = num;

    while n > 0 {
        let digit = n % 10;
        sum += digit.pow(pow);
        n /= 10;
    }

    sum == num
}
