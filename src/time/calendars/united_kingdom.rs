use time::macros::date;
use time::{Date, Duration, Month, Weekday};

use crate::time::{calendar::Calendar, utils};

/// The `UnitedKingdom` struct represents the United Kingdom calendar.
/// Equivalent to the QuantLib `UnitedKingdom.Settlement` class.
pub struct UnitedKingdom;

impl UnitedKingdom {
    /// Checks if a given date is a bank holiday in the United Kingdom.
    fn get_bank_holiday(&self, date: &Date) -> Option<String> {
        let weekday = date.weekday();
        let day = date.day();
        let month = date.month();
        let year = date.year();

        // First Monday of May (Early May Bank Holiday)
        // Moved to May 8th in 1995 and 2020 for V.E. day
        if (day <= 7
            && weekday == Weekday::Monday
            && month == Month::May
            && year != 1995
            && year != 2020)
            || (day == 8 && month == Month::May && (year == 1995 || year == 2020))
        {
            return Some("Early May Bank Holiday".to_string());
        }

        // Last Monday of May (Spring Bank Holiday)
        // 2002: 3rd and 4th June for the Golden Jubilee
        // 2012: 4th and 5th June for the Diamond Jubilee
        // 2022: 2nd and 3rd June for the Platinum Jubilee
        if (day >= 25
            && weekday == Weekday::Monday
            && month == Month::May
            && year != 2002
            && year != 2012
            && year != 2022)
            || ((day == 3 || day == 4) && month == Month::June && year == 2002)
            || ((day == 4 || day == 5) && month == Month::June && year == 2012)
            || ((day == 2 || day == 3) && month == Month::June && year == 2022)
        {
            return Some("Spring Bank Holiday".to_string());
        }

        // Last Monday of August (Summer Bank Holiday)
        if day >= 25 && weekday == Weekday::Monday && month == Month::August {
            return Some("Summer Bank Holiday".to_string());
        }

        // April 29th, 2011 only (Royal Wedding Bank Holiday)
        if day == 29 && month == Month::April && year == 2011 {
            return Some("Royal Wedding Bank Holiday".to_string());
        }

        // September 19th, 2022 only (The Queen's Funeral Bank Holiday)
        if day == 19 && month == Month::September && year == 2022 {
            return Some("The Queen's Funeral Bank Holiday".to_string());
        }

        // May 8th, 2023 (King Charles III Coronation Bank Holiday)
        if day == 8 && month == Month::May && year == 2023 {
            return Some("King Charles III Coronation Bank Holiday".to_string());
        }

        None
    }
}

impl Calendar for UnitedKingdom {
    fn new() -> Self {
        UnitedKingdom
    }

    fn name(&self) -> &'static str {
        "United Kingdom"
    }

    fn get_holiday(&self, date: &Date) -> Option<String> {
        if utils::is_weekend(date) {
            return Some("Weekend".to_string());
        }

        if let Some(holiday) = self.get_bank_holiday(date) {
            return Some(holiday);
        }

        let day = date.day();
        let month = date.month();
        let year = date.year();

        let easter_monday = utils::easter_monday(year, false);

        // Good Friday
        if *date
            == easter_monday
                .checked_sub(Duration::days(3))
                .expect("could not subtract 3 days")
        {
            return Some("Good Friday".to_string());
        }

        // Easter Monday
        if *date == easter_monday {
            return Some("Easter Monday".to_string());
        }

        // New Year's Day (possibly moved to Monday)
        if (day == 1 || ((day == 2 || day == 3) && date.weekday() == Weekday::Monday))
            && month == Month::January
        {
            return Some("New Year's Day".to_string());
        }

        // Christmas (possibly moved to Monday or Tuesday)
        if (day == 25
            || (day == 27
                && (date.weekday() == Weekday::Monday || date.weekday() == Weekday::Tuesday)))
            && month == Month::December
        {
            return Some("Christmas Day".to_string());
        }

        // Boxing Day (possibly moved to Monday or Tuesday)
        if (day == 26
            || (day == 28
                && (date.weekday() == Weekday::Monday || date.weekday() == Weekday::Tuesday)))
            && month == Month::December
        {
            return Some("Boxing Day".to_string());
        }

        // Millenium Celebrations
        if *date == date!(1999 - 12 - 31) {
            return Some("Millenium Celebrations".to_string());
        }

        None
    }
}

#[cfg(test)]
mod test_united_kingdom {
    use std::ffi::CString;

    use crate::time::utils::{MAX_DATE, MIN_DATE};

    use super::*;
    use pyo3::{
        types::{PyAnyMethods, PyModule, PyModuleMethods},
        Python,
    };
    use time::macros::date;

    #[test]
    fn test_name() {
        let calendar = UnitedKingdom::new();
        assert_eq!(calendar.name(), "United Kingdom");
    }

    #[test]
    fn test_weekend() {
        let calendar = UnitedKingdom::new();
        let weekend_date = date!(2023 - 12 - 23); // Saturday
        assert_eq!(
            calendar.get_holiday(&weekend_date),
            Some("Weekend".to_string())
        );
    }

    #[test]
    fn test_early_may_bank_holiday() {
        let calendar = UnitedKingdom::new();
        assert_eq!(
            calendar.get_holiday(&date!(2023 - 05 - 01)),
            Some("Early May Bank Holiday".to_string())
        );
        assert_eq!(
            calendar.get_holiday(&date!(1995 - 05 - 08)),
            Some("Early May Bank Holiday".to_string())
        );
        assert_eq!(
            calendar.get_holiday(&date!(2020 - 05 - 08)),
            Some("Early May Bank Holiday".to_string())
        );
    }

    #[test]
    fn test_spring_bank_holiday() {
        let calendar = UnitedKingdom::new();
        assert_eq!(
            calendar.get_holiday(&date!(2023 - 05 - 29)),
            Some("Spring Bank Holiday".to_string())
        );
        assert_eq!(
            calendar.get_holiday(&date!(2002 - 06 - 03)),
            Some("Spring Bank Holiday".to_string())
        );
        assert_eq!(
            calendar.get_holiday(&date!(2002 - 06 - 04)),
            Some("Spring Bank Holiday".to_string())
        );
        assert_eq!(
            calendar.get_holiday(&date!(2012 - 06 - 04)),
            Some("Spring Bank Holiday".to_string())
        );
        assert_eq!(
            calendar.get_holiday(&date!(2012 - 06 - 05)),
            Some("Spring Bank Holiday".to_string())
        );
        assert_eq!(
            calendar.get_holiday(&date!(2022 - 06 - 02)),
            Some("Spring Bank Holiday".to_string())
        );
        assert_eq!(
            calendar.get_holiday(&date!(2022 - 06 - 03)),
            Some("Spring Bank Holiday".to_string())
        );
    }

    #[test]
    fn test_summer_bank_holiday() {
        let calendar = UnitedKingdom::new();
        assert_eq!(
            calendar.get_holiday(&date!(2023 - 08 - 28)),
            Some("Summer Bank Holiday".to_string())
        );
    }

    #[test]
    fn test_royal_wedding_holiday() {
        let calendar = UnitedKingdom::new();
        assert_eq!(
            calendar.get_holiday(&date!(2011 - 04 - 29)),
            Some("Royal Wedding Bank Holiday".to_string())
        );
    }

    #[test]
    fn test_queens_funeral() {
        let calendar = UnitedKingdom::new();
        assert_eq!(
            calendar.get_holiday(&date!(2022 - 09 - 19)),
            Some("The Queen's Funeral Bank Holiday".to_string())
        );
    }

    #[test]
    fn test_kings_coronation() {
        let calendar = UnitedKingdom::new();
        assert_eq!(
            calendar.get_holiday(&date!(2023 - 05 - 08)),
            Some("King Charles III Coronation Bank Holiday".to_string())
        );
    }

    #[test]
    fn test_new_years_day() {
        let calendar = UnitedKingdom::new();
        assert_eq!(
            calendar.get_holiday(&date!(2024 - 01 - 01)),
            Some("New Year's Day".to_string())
        );
        assert_eq!(
            calendar.get_holiday(&date!(2023 - 01 - 02)),
            Some("New Year's Day".to_string())
        );
    }

    #[test]
    fn test_christmas_and_boxing_day() {
        let calendar = UnitedKingdom::new();
        assert_eq!(
            calendar.get_holiday(&date!(2024 - 12 - 25)),
            Some("Christmas Day".to_string())
        );
        assert_eq!(
            calendar.get_holiday(&date!(2024 - 12 - 26)),
            Some("Boxing Day".to_string())
        );

        // Observed shifts
        assert_eq!(
            calendar.get_holiday(&date!(2021 - 12 - 27)),
            Some("Christmas Day".to_string())
        );
        assert_eq!(
            calendar.get_holiday(&date!(2021 - 12 - 28)),
            Some("Boxing Day".to_string())
        );
    }

    #[test]
    fn test_good_friday_and_easter_monday() {
        let calendar = UnitedKingdom::new();

        // Easter Monday 2024 is April 1
        assert_eq!(
            calendar.get_holiday(&date!(2024 - 04 - 01)),
            Some("Easter Monday".to_string())
        );
        assert_eq!(
            calendar.get_holiday(&date!(2024 - 03 - 29)),
            Some("Good Friday".to_string())
        );
    }

    #[test]
    fn test_non_holiday_weekday() {
        let calendar = UnitedKingdom::new();
        assert_eq!(calendar.get_holiday(&date!(2023 - 07 - 04)), None); // Regular Tuesday
    }

    #[test]
    fn test_construct_holiday_vec() {
        // Use QuantLib to calculate the expected number of holidays
        let expected_count: usize = Python::with_gil(|py| {
            let code = r#"
import QuantLib as ql

calendar = ql.UnitedKingdom(ql.UnitedKingdom.Settlement)
start = ql.Date(1, 1, 1901)
end = ql.Date(30, 12, 2199)

holidays = ql.Calendar.holidayList(calendar, start, end, includeWeekEnds=True)
result = len(holidays)
"#;

            // Run the code
            let locals = PyModule::new(py, "locals").unwrap().dict();
            py.run(
                &CString::new(code).expect("Failed to convert to CString"),
                None,
                Some(&locals),
            )
            .expect("Failed to execute Python code");

            // Extract the result
            locals
                .get_item("result")
                .expect("result not found")
                .extract()
                .expect("Failed to extract result as usize")
        });

        let calendar = UnitedKingdom::new();
        let from = MIN_DATE;
        let to = MAX_DATE;
        let holidays = calendar.construct_holiday_vec(from, to);

        assert_eq!(holidays.len(), expected_count);
    }

    #[test]
    fn test_compare_rust_and_quantlib_holidays_1900_2000() {
        use std::collections::HashSet;

        let rust_holidays: HashSet<_> = {
            let calendar = UnitedKingdom::new();
            let from = date!(1950 - 01 - 01);
            let to = date!(2000 - 12 - 31);
            calendar
                .construct_holiday_vec(from, to)
                .into_iter()
                .collect::<HashSet<_>>()
        };

        let quantlib_holidays: HashSet<String> = Python::with_gil(|py| {
            let code = r#"
import QuantLib as ql

calendar = ql.UnitedKingdom(ql.UnitedKingdom.Settlement)
start = ql.Date(1, 1, 1950)
end = ql.Date(31, 12, 2000)

holidays = ql.Calendar.holidayList(calendar, start, end, includeWeekEnds=True)
result = [d.ISO() for d in holidays]
"#;
            let locals = PyModule::new(py, "locals").unwrap().dict();
            py.run(
                &CString::new(code).expect("failed to convert to CString"),
                None,
                Some(&locals),
            )
            .expect("failed to execute Python code");

            locals
                .get_item("result")
                .expect("result not found")
                .extract::<Vec<String>>()
                .expect("failed to extract holidays")
                .into_iter()
                .collect()
        });

        let rust_holiday_strs: HashSet<String> = rust_holidays
            .into_iter()
            .map(|d| d.to_string()) // ISO 8601 format: "YYYY-MM-DD"
            .collect();

        let only_in_rust: Vec<_> = rust_holiday_strs.difference(&quantlib_holidays).collect();
        let only_in_quantlib: Vec<_> = quantlib_holidays.difference(&rust_holiday_strs).collect();

        assert!(
            only_in_rust.is_empty() && only_in_quantlib.is_empty(),
            "holiday mismatch\nOnly in Rust: {:?}\nonly in QuantLib: {:?}",
            only_in_rust,
            only_in_quantlib
        );
    }
}
