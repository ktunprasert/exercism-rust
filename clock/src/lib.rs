use std::fmt::{Debug, Display};

pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Debug for Clock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Clock {{ hours: {}, minutes: {} }}",
            self.hours, self.minutes
        )
    }
}

impl PartialEq for Clock {
    fn eq(&self, other: &Self) -> bool {
        self.hours == other.hours && self.minutes == other.minutes
    }
}

impl Display for Clock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let (h, m) = Self::calc_time(hours, minutes);
        Clock {
            hours: h,
            minutes: m,
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let (h, m) = Self::calc_time(self.hours, self.minutes + minutes);
        Clock {
            hours: h,
            minutes: m,
        }
    }

    fn calc_time(hours: i32, minutes: i32) -> (i32, i32) {
        let total = 1440 + ((hours * 60 + minutes) % 1440);
        (total / 60 % 24, total % 60)
    }
}
