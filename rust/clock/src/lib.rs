use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Clock {
    minutes: i32,
}

const DAY: i32 = 60 * 24;

fn wrap_to_one_day(minutes: i32) -> i32 {
    (DAY + (minutes % DAY)) % DAY
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Clock {
            minutes: wrap_to_one_day(minutes + hours * 60),
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock {
            minutes: wrap_to_one_day(self.minutes + minutes),
        }
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.minutes / 60, self.minutes % 60)
    }
}
