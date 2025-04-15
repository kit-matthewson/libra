use crate::time::calendar::CalendarInterface;
use crate::time::{Date, Month, Period, Weekday};

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

impl CalendarInterface for UnitedKingdom {
    fn name(&self) -> &'static str {
        "United Kingdom"
    }

    fn get_holiday(&self, date: &Date) -> Option<String> {
        if date.weekday().is_weekend() {
            return Some("Weekend".to_string());
        }

        if let Some(holiday) = self.get_bank_holiday(date) {
            return Some(holiday);
        }

        let day = date.day();
        let month = date.month();
        let year = date.year();

        let easter_monday = Date::get_easter_monday(year, false).unwrap();

        // Good Friday
        if *date
            == easter_monday
                .decrement(&Period::Day(3))
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
        if *date == Date::from_parts(31, Month::December, 1999).unwrap() {
            return Some("Millenium Celebrations".to_string());
        }

        None
    }
}
