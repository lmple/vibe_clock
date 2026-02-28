use chrono::NaiveDateTime;

pub trait Clock {
    fn now(&self) -> NaiveDateTime;
}

pub struct SystemClock;

impl Clock for SystemClock {
    fn now(&self) -> NaiveDateTime {
        chrono::Local::now().naive_local()
    }
}
