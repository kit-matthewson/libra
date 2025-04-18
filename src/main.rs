use chrono::{Months, NaiveDate};
use libra::{
    cashflows::Coupons,
    instruments::Bond,
    time::{Calendar, DateAdjustment, DayCountConvention},
};

fn main() {
    let bond = Bond::new(
        Calendar::UnitedKingdom,
        DayCountConvention::Thirty360,
        DateAdjustment::Following,
        NaiveDate::from_ymd_opt(2006, 5, 26).unwrap(),
        NaiveDate::from_ymd_opt(2009, 5, 26).unwrap(),
        98.0,
        100.0,
        Some(Coupons::Fixed(0.055, Months::new(12))),
    );

    println!(
        "{:.4}",
        bond.present_value(0.0544, NaiveDate::from_ymd_opt(2006, 7, 14).unwrap()).unwrap()
    );
}
