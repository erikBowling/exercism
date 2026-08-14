use std::fmt;

#[derive(PartialEq, Eq, Debug)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let (h, m) = Clock::normalize_values(hours, minutes);

        Self {
            hours: h,
            minutes: m,
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let (current_hours, current_minutes) =
            Clock::normalize_values(self.hours, self.minutes + minutes);

        Self {
            hours: current_hours,
            minutes: current_minutes,
        }
    }

    pub fn normalize_values(hours: i32, minutes: i32) -> (i32, i32) {
        // If minutes comes in negative or above 60
        // If hours comes in negative or >= 24

        let mut h = hours;
        let mut m = minutes;

        while m >= 60 {
            h += 1;
            m -= 60;
        }

        while m < 0 {
            h -= 1;
            m += 60;
        }

        if h < 0 || h >= 24 {
            h = h.rem_euclid(24);
        }

        (h, m)
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut h: String = String::new();
        let mut m: String = String::new();

        if self.hours < 10 {
            h.push('0');
        }

        if self.minutes < 10 {
            m.push('0');
        }

        h.push_str(&self.hours.to_string());
        m.push_str(&self.minutes.to_string());

        write!(f, "{}:{}", h, m)
    }
}
