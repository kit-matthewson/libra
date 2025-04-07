use time::Date;

pub trait Calendar {
    fn new() -> Self;
    fn name(&self) -> &'static str;

    fn construct_holiday_vec(&self, from: Date, to: Date) -> Vec<Date> {
        let mut holiday_dates = Vec::new();

        let mut date = from;
        while date <= to {
            if self.get_holiday(&date).is_some() {
                holiday_dates.push(date);
            }

            date = date.next_day().expect("could not increment day");
        }

        holiday_dates
    }

    fn get_holiday(&self, date: &Date) -> Option<String>;

    fn is_buisness_day(&self, date: &Date) -> bool {
        self.get_holiday(date).is_none()
    }
}

#[cfg(test)]
mod test_calendar {
    use super::*;
    use crate::time::calendars::UnitedKingdom;
    use time::macros::date;

    #[test]
    fn test_is_buisness_day() {
        let calendar = UnitedKingdom::new();
        let date = date!(2023 - 12 - 25); // Christmas Day
        assert!(!calendar.is_buisness_day(&date));
    }
}
