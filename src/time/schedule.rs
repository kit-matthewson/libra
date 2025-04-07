use time::Date;

use super::{calendar::Calendar, period::Period};

pub struct Schedule<C: Calendar> {
    calendar: C,
    effective_date: Date,
    termination_date: Date,
    tenor: Period,
}
