use libra::{instruments::fixed_rate_bond::FixedRateBond, time::{calendar::Calendar, calendars, date, utils::is_weekend}};
use time::macros::date;

fn main() {
    let calendar = calendars::UnitedKingdom::new();

    let bond = FixedRateBond::new::<calendar>(
        date!(2023-1-1),
        date!(2024-1-1),
        0.05,
        1000.0,
    );
}
