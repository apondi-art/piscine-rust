use chrono::Duration;
use colored::*;

#[derive(Debug, Eq, PartialEq)]
pub enum Position {
    Top,
    Bottom,
    Center,
}

#[derive(Debug, Eq, PartialEq)]
pub struct Notification {
    pub size: u32,
    pub color: (u8, u8, u8),
    pub position: Position,
    pub content: String,
}

#[derive(Debug)]
pub enum Event<'a> {
    Remainder(&'a str),
    Registration(Duration),
    Appointment(&'a str),
    Holiday,
}

use std::fmt;

#[derive(Debug)]
struct DurationInHours {
    hours: i64,
    minutes: i64,
    seconds: i64,
}

impl From<&Duration> for DurationInHours {
    fn from(duration: &Duration) -> DurationInHours {
        let mut left = duration.num_seconds();
        let hours = left / 3600;
        left %= 3600;
        let minutes = left / 60;
        left %= 60;
        let seconds = left;
        DurationInHours {
            hours,
            minutes,
            seconds,
        }
    }
}

impl fmt::Display for DurationInHours {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}H:{}M:{}S", self.hours, self.minutes, self.seconds)
    }
}

impl fmt::Display for Notification {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "({:?}, {}, {})",
            self.position,
            self.size,
            self.content
                .truecolor(self.color.0, self.color.1, self.color.2)
        )
    }
}