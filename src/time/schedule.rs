use chrono::Months;

pub enum Schedule {
    FixedRate(f64, Months),
}
