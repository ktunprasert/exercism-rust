#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    SpanTooLong,
    InvalidDigit(char),
}

pub fn lsp(string_digits: &str, span: usize) -> Result<u64, Error> {
    if span == 0 {
        return Ok(1);
    }

    string_digits
        .chars()
        .map(|c| {
            c.to_digit(10)
                .ok_or(Error::InvalidDigit(c))
                .map(|d| d as u64)
        })
        .collect::<Result<Vec<u64>, _>>()?
        .windows(span)
        .map(|w| w.iter().product::<u64>())
        .max()
        .ok_or(Error::SpanTooLong)
}
