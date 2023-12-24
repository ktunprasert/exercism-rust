use std::{time::Duration, ops::Add};

use time::PrimitiveDateTime as DateTime;

const GIGA_SECONDS: Duration = Duration::from_secs(1_000_000_000);

// Returns a DateTime one billion seconds after start.
pub fn after(start: DateTime) -> DateTime {
    start.add(GIGA_SECONDS)
}
