use super::period::Period;

pub enum Schedule {
    FixedRate(f64, Period),
}
