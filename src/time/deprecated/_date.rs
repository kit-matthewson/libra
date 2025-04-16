// Number tables and equations from QuantLib

use std::{fmt, ops};

use super::Period;
use crate::error::{self};
use crate::time::data_tables as tables;

/// Representation of a date
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Date {
    /// The days since January 1st, 1901
    serial: u32,
}

impl Date {
    /// The minimum allowed serial (inclusive)
    /// Equal to 01-01-1900
    pub const MIN_SERIAL: u32 = 0;
    /// The maximum allowed serial (inclusive)
    /// Equal to 31-12-2199
    pub const MAX_SERIAL: u32 = 109573;

    /// The minimum allowed year (inclusive)
    pub const MIN_YEAR: u16 = 1900;
    /// The maximum allowed year (inclusive)
    pub const MAX_YEAR: u16 = 2199;

    /// The minimum allowed date (inclusive)
    /// 01-01-1900
    pub const MIN_DATE: Self = unsafe { Self::from_serial_unchecked(Self::MIN_SERIAL) };
    /// The maximum allowed date (inclusive)
    /// 31-12-2199
    pub const MAX_DATE: Self = unsafe { Self::from_serial_unchecked(Self::MAX_SERIAL) };

    pub fn from_parts(day: u8, month: Month, year: u16) -> Result<Self, error::ArgumentRange> {
        if year > Self::MAX_YEAR || year < Self::MIN_YEAR {
            return Err(error::ArgumentRange {
                name: "year",
                min: Self::MIN_YEAR as i64,
                max: Self::MAX_YEAR as i64,
                value: year as i64,
                conditional_message: None,
            });
        }

        let leap = Self::is_leap_year(year)?;
        let month_days = month.days(leap);

        if day > month_days || day < 1 {
            return Err(error::ArgumentRange {
                name: "day",
                min: 1,
                max: month_days as i64,
                value: day as i64,
                conditional_message: Some(format!("{} {} has {} days", month, year, month_days)),
            });
        }

        Ok(Self {
            serial: day as u32 + Self::get_month_offset(month, leap) + Self::get_year_offset(year)?
                - 1,
        })
    }

    pub fn from_ordinal(year: u16, day_of_year: u16) -> Result<Self, error::ArgumentRange> {
        if year > Self::MAX_YEAR || year < Self::MIN_YEAR {
            return Err(error::ArgumentRange {
                name: "year",
                min: Self::MIN_YEAR as i64,
                max: Self::MAX_YEAR as i64,
                value: year as i64,
                conditional_message: None,
            });
        }

        if day_of_year > 366 || day_of_year < 1 {
            return Err(error::ArgumentRange {
                name: "day_of_year",
                min: 1,
                max: 366,
                value: day_of_year as i64,
                conditional_message: None,
            });
        }

        Ok(Self::from_serial(
            Self::get_year_offset(year)? + day_of_year as u32 - 1,
        )?)
    }

    pub fn from_serial(serial_number: u32) -> Result<Self, error::ArgumentRange> {
        if serial_number > Self::MAX_SERIAL || serial_number < Self::MIN_SERIAL {
            return Err(error::ArgumentRange {
                name: "serial_number",
                min: Self::MIN_SERIAL as i64,
                max: Self::MAX_SERIAL as i64,
                value: serial_number as i64,
                conditional_message: None,
            });
        }

        Ok(Self {
            serial: serial_number,
        })
    }

    pub const unsafe fn from_serial_unchecked(serial_number: u32) -> Self {
        Date {
            serial: serial_number,
        }
    }

    pub fn is_leap_year(year: u16) -> Result<bool, error::ArgumentRange> {
        if year > Self::MAX_YEAR || year < Self::MIN_YEAR {
            return Err(error::ArgumentRange {
                name: "year",
                min: Self::MIN_YEAR as i64,
                max: Self::MAX_YEAR as i64,
                value: year as i64,
                conditional_message: None,
            });
        }

        // A year is a leap year if it is divisible by 4, unless it is also divisible by 100 but not 400.

        if year % 4 != 0 {
            return Ok(false);
        }

        if year % 100 == 0 {
            return Ok(year % 400 == 0);
        }

        Ok(true)
    }

    pub fn is_leap(&self) -> bool {
        match Self::is_leap_year(self.year()) {
            Ok(v) => v,
            Err(err) => unreachable!("{}", err),
        }
    }

    pub fn get_easter_monday(year: u16, orthodox: bool) -> Result<Self, error::ArgumentRange> {
        if year > Self::MAX_YEAR || year < Self::MIN_YEAR {
            return Err(error::ArgumentRange {
                name: "year",
                min: Self::MIN_YEAR as i64,
                max: Self::MAX_YEAR as i64,
                value: year as i64,
                conditional_message: None,
            });
        }

        if orthodox {
            Ok(Self::from_ordinal(
                year,
                tables::ORTHODOX_EASTER_MONDAYS[year as usize - 1900],
            )?)
        } else {
            Ok(Self::from_ordinal(
                year,
                tables::WESTERN_EASTER_MONDAYS[year as usize - 1900],
            )?)
        }
    }

    pub fn easter_monday(&self, orthodox: bool) -> Self {
        match Self::get_easter_monday(self.year(), orthodox) {
            Ok(v) => v,
            Err(err) => unreachable!("{}", err),
        }
    }

    /// The year contained in this date
    pub fn year(&self) -> u16 {
        let mut year = ((self.serial / 365) + 1900) as u16;

        if self.serial < Self::get_year_offset(year).unwrap() {
            year -= 1;
        }

        year
    }

    /// The month contained in this date
    pub fn month(&self) -> Month {
        let d = self.day_of_year() as u32;

        let mut m = match Month::try_from(((d / 30) as u8) % 12 + 1) {
            Ok(m) => m,
            Err(err) => unreachable!("{}", err),
        };

        let is_leap = self.is_leap();

        while d < Self::get_month_offset(m, is_leap) {
            m = m.previous();
        }

        while d > Self::get_month_offset(m.following(), is_leap) && m.following() != Month::January
        {
            m = m.following();
        }

        match Month::try_from(m as u8 + 1) {
            Ok(m) => m,
            Err(err) => unreachable!("{}", err),
        }
    }

    /// Returns the day of the month that this date refers to.
    pub fn day(&self) -> u8 {
        (self.serial - self.year_offset() - self.month_offset()) as u8 + 1
    }

    /// Returns the day of the year that this date refers to, with January 1st as 1.
    pub fn day_of_year(&self) -> u16 {
        (self.serial - self.year_offset()) as u16
    }

    pub fn weekday(&self) -> Weekday {
        // 01-01-1900 was a Monday
        Weekday::try_from((self.serial % 7) as u8 + 1).unwrap()
    }

    /// Increments date by the given period.
    /// When incrementing by months or years, the day of month is clamped to the length of the new month.
    ///
    /// *Examples*
    /// Feb 29th 2004 + 1 Year = Fen 28th 2005
    /// March 31th + 1 Month = April 30th
    pub fn increment(&self, period: &Period) -> Result<Self, error::ArgumentRange> {
        match period {
            Period::Years(years) => {
                let new_month_days = self.month().days(Self::is_leap_year(self.year() + years)?);
                let date = Self::from_parts(
                    self.day().min(new_month_days),
                    self.month(),
                    self.year() + years,
                )?;
                Ok(date)
            }
            Period::Months(months) => {
                let mut years = months / 12;
                let months = months % 12;

                let new_month_days = self.month().days(Self::is_leap_year(self.year() + years)?);

                if self.month().nth_following(months) < self.month() {
                    years = years + 1;
                }

                let date = Self::from_parts(
                    self.day().min(new_month_days),
                    self.month().nth_following(months),
                    self.year() + years,
                )?;
                Ok(date)
            }
            Period::Days(days) => {
                let date = Self::from_serial(self.serial + days)?;
                Ok(date)
            }
        }
    }

    pub fn decrement(&self, period: &Period) -> Result<Self, error::ArgumentRange> {
        match period {
            Period::Years(years) => {
                let new_month_days = self.month().days(Self::is_leap_year(self.year() - years)?);
                let date = Self::from_parts(
                    self.day().min(new_month_days),
                    self.month(),
                    self.year() - years,
                )?;
                Ok(date)
            }
            Period::Months(months) => {
                let mut years = months / 12;
                let months = months % 12;

                let new_month_days = self.month().days(Self::is_leap_year(self.year() - years)?);

                if self.month().nth_following(months) < self.month() {
                    years = years + 1;
                }

                let date = Self::from_parts(
                    self.day().min(new_month_days),
                    self.month().nth_previous(months),
                    self.year() - years,
                )?;
                Ok(date)
            }
            Period::Days(days) => {
                let date = Self::from_serial(self.serial - days)?;
                Ok(date)
            }
        }
    }

    pub fn next_day(&self) -> Result<Self, error::ArgumentRange> {
        self.increment(&Period::Days(1))
    }

    pub fn previous_day(&self) -> Result<Self, error::ArgumentRange> {
        self.decrement(&Period::Days(1))
    }

    pub fn next_month(&self) -> Result<Self, error::ArgumentRange> {
        self.increment(&Period::Months(1))
    }

    pub fn previous_month(&self) -> Result<Self, error::ArgumentRange> {
        self.decrement(&Period::Months(1))
    }

    pub fn next_year(&self) -> Result<Self, error::ArgumentRange> {
        self.increment(&Period::Years(1))
    }

    pub fn previous_year(&self) -> Result<Self, error::ArgumentRange> {
        self.decrement(&Period::Years(1))
    }

    /// Returns the number of days into the year for the 1st of the given month.
    fn get_month_offset(month: Month, is_leap: bool) -> u32 {
        if is_leap {
            tables::LEAP_MONTH_OFFSETS[month as usize]
        } else {
            tables::MONTH_OFFSETS[month as usize]
        }
    }

    fn month_offset(&self) -> u32 {
        Self::get_month_offset(self.month(), self.is_leap())
    }

    /// Returns the serial for 1st January of the given year.
    fn get_year_offset(year: u16) -> Result<u32, error::ArgumentRange> {
        // This function must support a max year one higher than normal, as a year guess may be too high
        if year > Self::MAX_YEAR + 1 || year < Self::MIN_YEAR {
            return Err(error::ArgumentRange {
                name: "year",
                min: Self::MIN_YEAR as i64,
                max: Self::MAX_YEAR as i64,
                value: year as i64,
                conditional_message: None,
            });
        }

        Ok(tables::YEAR_OFFSETS[year as usize - 1900])
    }

    pub fn year_offset(&self) -> u32 {
        match Self::get_year_offset(self.year()) {
            Ok(v) => v,
            Err(err) => unreachable!("{}", err),
        }
    }
}

impl ops::Sub for Date {
    type Output = Period;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut days = self.serial as i64 - rhs.serial as i64;

        if days < 0 {
            days = -days;
        }

        Period::Days(days as u32)
    }
}

impl fmt::Display for Date {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:02}-{:02}-{}",
            self.day(),
            self.month() as u8 + 1,
            self.year()
        )
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Month {
    January,
    February,
    March,
    April,
    May,
    June,
    July,
    August,
    September,
    October,
    November,
    December,
}

impl Month {
    pub fn days(&self, is_leap: bool) -> u8 {
        match self {
            Month::January => 31,
            Month::February => {
                if is_leap {
                    29
                } else {
                    28
                }
            }
            Month::March => 31,
            Month::April => 30,
            Month::May => 31,
            Month::June => 30,
            Month::July => 31,
            Month::August => 31,
            Month::September => 30,
            Month::October => 31,
            Month::November => 30,
            Month::December => 31,
        }
    }

    pub fn following(&self) -> Self {
        match self {
            Month::January => Month::February,
            Month::February => Month::March,
            Month::March => Month::April,
            Month::April => Month::May,
            Month::May => Month::June,
            Month::June => Month::July,
            Month::July => Month::August,
            Month::August => Month::September,
            Month::September => Month::October,
            Month::October => Month::November,
            Month::November => Month::December,
            Month::December => Month::January,
        }
    }

    pub fn nth_following(&self, n: u16) -> Self {
        let mut m = self.clone();

        for _ in 0..n {
            m = m.following();
        }

        m
    }

    pub fn previous(&self) -> Self {
        match self {
            Month::January => Month::December,
            Month::February => Month::January,
            Month::March => Month::February,
            Month::April => Month::March,
            Month::May => Month::April,
            Month::June => Month::May,
            Month::July => Month::June,
            Month::August => Month::July,
            Month::September => Month::August,
            Month::October => Month::September,
            Month::November => Month::October,
            Month::December => Month::November,
        }
    }

    pub fn nth_previous(&self, n: u16) -> Self {
        let mut m = self.clone();

        for _ in 0..n {
            m = m.previous();
        }

        m
    }
}

impl fmt::Display for Month {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let month_str = match self {
            Month::January => "January",
            Month::February => "February",
            Month::March => "March",
            Month::April => "April",
            Month::May => "May",
            Month::June => "June",
            Month::July => "July",
            Month::August => "August",
            Month::September => "September",
            Month::October => "October",
            Month::November => "November",
            Month::December => "December",
        };

        write!(f, "{}", month_str)
    }
}

impl From<Month> for u8 {
    fn from(value: Month) -> Self {
        value as Self
    }
}

impl TryFrom<u8> for Month {
    type Error = error::ArgumentRange;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Month::January),
            2 => Ok(Month::February),
            3 => Ok(Month::March),
            4 => Ok(Month::April),
            5 => Ok(Month::May),
            6 => Ok(Month::June),
            7 => Ok(Month::July),
            8 => Ok(Month::August),
            9 => Ok(Month::September),
            10 => Ok(Month::October),
            11 => Ok(Month::November),
            12 => Ok(Month::December),
            _ => Err(error::ArgumentRange {
                name: "month",
                min: 1,
                max: 12,
                value: value as i64,
                conditional_message: None,
            }),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Weekday {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

impl Weekday {
    pub fn following(&self) -> Self {
        match self {
            Weekday::Monday => Weekday::Tuesday,
            Weekday::Tuesday => Weekday::Wednesday,
            Weekday::Wednesday => Weekday::Thursday,
            Weekday::Thursday => Weekday::Friday,
            Weekday::Friday => Weekday::Saturday,
            Weekday::Saturday => Weekday::Sunday,
            Weekday::Sunday => Weekday::Monday,
        }
    }

    pub fn previous(&self) -> Self {
        match self {
            Weekday::Monday => Weekday::Sunday,
            Weekday::Tuesday => Weekday::Monday,
            Weekday::Wednesday => Weekday::Tuesday,
            Weekday::Thursday => Weekday::Wednesday,
            Weekday::Friday => Weekday::Thursday,
            Weekday::Saturday => Weekday::Friday,
            Weekday::Sunday => Weekday::Saturday,
        }
    }

    pub fn is_weekend(&self) -> bool {
        match self {
            Weekday::Saturday | Weekday::Sunday => true,
            _ => false,
        }
    }

    pub fn is_weekday(&self) -> bool {
        !self.is_weekend()
    }
}

impl TryFrom<u8> for Weekday {
    type Error = error::ArgumentRange;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Weekday::Monday),
            2 => Ok(Weekday::Tuesday),
            3 => Ok(Weekday::Wednesday),
            4 => Ok(Weekday::Thursday),
            5 => Ok(Weekday::Friday),
            6 => Ok(Self::Saturday),
            7 => Ok(Weekday::Sunday),
            _ => Err(error::ArgumentRange {
                name: "weekday",
                min: 1,
                max: 7,
                value: value as i64,
                conditional_message: None,
            }),
        }
    }
}

impl fmt::Display for Weekday {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let weekday_str = match self {
            Weekday::Monday => "Monday",
            Weekday::Tuesday => "Tuesday",
            Weekday::Wednesday => "Wednesday",
            Weekday::Thursday => "Thursday",
            Weekday::Friday => "Friday",
            Weekday::Saturday => "Saturday",
            Weekday::Sunday => "Sunday",
        };

        write!(f, "{}", weekday_str)
    }
}
