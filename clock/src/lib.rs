
use std::fmt::{Display, Formatter, Result};

#[derive(PartialEq, Debug)]
pub struct Clock {
    hours: i32,
    minutes: i32
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {

        let mut new_hours = hours;
        if minutes.abs() >= 60 {
            let foo = minutes / 60;
            new_hours = hours + foo;
        } 

        let mut hours_formatted = new_hours % 24;
        let mut minutes_formatted = minutes % 60;

        if minutes_formatted < 0 {
            minutes_formatted = 60 + minutes_formatted;
            hours_formatted = hours_formatted - 1;
        }

        if hours_formatted < 0 {
            hours_formatted = 24 + hours_formatted;
        }

        Clock { 
            hours: hours_formatted, 
            minutes: minutes_formatted 
        }

    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let h = self.hours;
        let m = self.minutes + minutes;
        Clock::new(h, m)
    }


}


impl Display for Clock {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}
