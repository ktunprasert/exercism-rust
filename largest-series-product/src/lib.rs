#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    SpanTooLong,
    InvalidDigit(char),
    NoProduct,
}

pub fn lsp(string_digits: &str, span: usize) -> Result<u64, Error> {
    if span > string_digits.len() {
        return Err(Error::SpanTooLong);
    } else if span == 0 {
        return Ok(1);
    }

    let mut digits = vec![];

    for c in string_digits.chars() {
        if !c.is_digit(10) {
            return Err(Error::InvalidDigit(c));
        }

        digits.push(c.to_digit(10).unwrap() as u64)
    }

    if digits.len() == span {
        return Ok(digits.into_iter().product());
    }

    digits
        .windows(span)
        .map(|x| x.iter().product::<u64>())
        .max()
        .ok_or(Error::NoProduct)
}
