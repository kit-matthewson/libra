use libra::*;
use ::time::macros::date;

fn main() {
    // A 10-Year UK Gilt
    // let bond = Bond::new(
    //     Calendar::UnitedKingdom,
    //     DayCountConvention::ActualActual,
    //     DateAdjustment::Following,
    //     date!(2010 - 01 - 01),
    //     date!(2020 - 01 - 01),
    //     100.0,
    //     Coupon::Fixed(4.5, Period::SemiAnnual),
    // );

    // let pv = bond.present_value();

    // println!("Present Value: {}", pv);

    // Ex. 8 from A Financial Bestiary
    let cf = vec![
        cashflows::CashFlow::new(1.0, date!(2026 - 01 - 01)),
        cashflows::CashFlow::new(1.0, date!(2027 - 01 - 01)),
        cashflows::CashFlow::new(5.0, date!(2028 - 01 - 01)),
    ];

    let total: f64 = cf
        .iter()
        .map(|f| {
            f.simple_present_value(&date!(2025 - 01 - 01), 0.05, time::DayCountConvention::Actual360)
        })
        .sum();

    println!("Exercise 8: ${:.2} million", total);
}
