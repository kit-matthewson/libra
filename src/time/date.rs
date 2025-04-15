// Number tables and equations from QuantLib

use std::{fmt, ops};

use crate::error::{self};

use super::Period;

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
        const EASTER_MONDAYS: [[u16; 300]; 2] = [
            // WESTERN EASTER MONDAYS
            [
                106, 98, 90, 103, 95, 114, 106, 91, 111, 102, // 1900-1909
                87, 107, 99, 83, 103, 95, 115, 99, 91, 111, // 1910-1919
                96, 87, 107, 92, 112, 103, 95, 108, 100, 91, // 1920-1929
                111, 96, 88, 107, 92, 112, 104, 88, 108, 100, // 1930-1939
                85, 104, 96, 116, 101, 92, 112, 97, 89, 108, // 1940-1949
                100, 85, 105, 96, 109, 101, 93, 112, 97, 89, // 1950-1959
                109, 93, 113, 105, 90, 109, 101, 86, 106, 97, // 1960-1969
                89, 102, 94, 113, 105, 90, 110, 101, 86, 106, // 1970-1979
                98, 110, 102, 94, 114, 98, 90, 110, 95, 86, // 1980-1989
                106, 91, 111, 102, 94, 107, 99, 90, 103, 95, // 1990-1999
                115, 106, 91, 111, 103, 87, 107, 99, 84, 103, // 2000-2009
                95, 115, 100, 91, 111, 96, 88, 107, 92, 112, // 2010-2019
                104, 95, 108, 100, 92, 111, 96, 88, 108, 92, // 2020-2029
                112, 104, 89, 108, 100, 85, 105, 96, 116, 101, // 2030-2039
                93, 112, 97, 89, 109, 100, 85, 105, 97, 109, // 2040-2049
                101, 93, 113, 97, 89, 109, 94, 113, 105, 90, // 2050-2059
                110, 101, 86, 106, 98, 89, 102, 94, 114, 105, // 2060-2069
                90, 110, 102, 86, 106, 98, 111, 102, 94, 114, // 2070-2079
                99, 90, 110, 95, 87, 106, 91, 111, 103, 94, // 2080-2089
                107, 99, 91, 103, 95, 115, 107, 91, 111, 103, // 2090-2099
                88, 108, 100, 85, 105, 96, 109, 101, 93, 112, // 2100-2109
                97, 89, 109, 93, 113, 105, 90, 109, 101, 86, // 2110-2119
                106, 97, 89, 102, 94, 113, 105, 90, 110, 101, // 2120-2129
                86, 106, 98, 110, 102, 94, 114, 98, 90, 110, // 2130-2139
                95, 86, 106, 91, 111, 102, 94, 107, 99, 90, // 2140-2149
                103, 95, 115, 106, 91, 111, 103, 87, 107, 99, // 2150-2159
                84, 103, 95, 115, 100, 91, 111, 96, 88, 107, // 2160-2169
                92, 112, 104, 95, 108, 100, 92, 111, 96, 88, // 2170-2179
                108, 92, 112, 104, 89, 108, 100, 85, 105, 96, // 2180-2189
                116, 101, 93, 112, 97, 89, 109, 100, 85, 105, // 2190-2199
            ],
            // ORTHODOX EASTER MONDAYS
            [
                113, 105, 118, 110, 102, 121, 106, 126, 118, 102, // 1900-1909
                122, 114, 99, 118, 110, 95, 115, 106, 126, 111, // 1910-1919
                103, 122, 107, 99, 119, 110, 123, 115, 107, 126, // 1920-1929
                111, 103, 123, 107, 99, 119, 104, 123, 115, 100, // 1930-1939
                120, 111, 96, 116, 108, 127, 112, 104, 124, 115, // 1940-1949
                100, 120, 112, 96, 116, 108, 128, 112, 104, 124, // 1950-1959
                109, 100, 120, 105, 125, 116, 101, 121, 113, 104, // 1960-1969
                117, 109, 101, 120, 105, 125, 117, 101, 121, 113, // 1970-1979
                98, 117, 109, 129, 114, 105, 125, 110, 102, 121, // 1980-1989
                106, 98, 118, 109, 122, 114, 106, 118, 110, 102, // 1990-1999
                122, 106, 126, 118, 103, 122, 114, 99, 119, 110, // 2000-2009
                95, 115, 107, 126, 111, 103, 123, 107, 99, 119, // 2010-2019
                111, 123, 115, 107, 127, 111, 103, 123, 108, 99, // 2020-2029
                119, 104, 124, 115, 100, 120, 112, 96, 116, 108, // 2030-2039
                128, 112, 104, 124, 116, 100, 120, 112, 97, 116, // 2040-2049
                108, 128, 113, 104, 124, 109, 101, 120, 105, 125, // 2050-2059
                117, 101, 121, 113, 105, 117, 109, 101, 121, 105, // 2060-2069
                125, 110, 102, 121, 113, 98, 118, 109, 129, 114, // 2070-2079
                106, 125, 110, 102, 122, 106, 98, 118, 110, 122, // 2080-2089
                114, 99, 119, 110, 102, 115, 107, 126, 118, 103, // 2090-2099
                123, 115, 100, 120, 112, 96, 116, 108, 128, 112, // 2100-2109
                104, 124, 109, 100, 120, 105, 125, 116, 108, 121, // 2110-2119
                113, 104, 124, 109, 101, 120, 105, 125, 117, 101, // 2120-2129
                121, 113, 98, 117, 109, 129, 114, 105, 125, 110, // 2130-2139
                102, 121, 113, 98, 118, 109, 129, 114, 106, 125, // 2140-2149
                110, 102, 122, 106, 126, 118, 103, 122, 114, 99, // 2150-2159
                119, 110, 102, 115, 107, 126, 111, 103, 123, 114, // 2160-2169
                99, 119, 111, 130, 115, 107, 127, 111, 103, 123, // 2170-2179
                108, 99, 119, 104, 124, 115, 100, 120, 112, 103, // 2180-2189
                116, 108, 128, 119, 104, 124, 116, 100, 120, 112, // 2190-2199
            ],
        ];

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
                EASTER_MONDAYS[1][year as usize - 1900],
            )?)
        } else {
            Ok(Self::from_ordinal(
                year,
                EASTER_MONDAYS[0][year as usize - 1900],
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

        while d > Self::get_month_offset(m.next(), is_leap) && m.next() != Month::January {
            m = m.next();
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
        todo!()
    }

    pub fn increment(&self, _period: &Period) -> Result<Self, error::Error> {
        todo!()
    }

    pub fn decrement(&self, _period: &Period) -> Result<Self, error::Error> {
        todo!()
    }

    pub fn next_day(&self) -> Result<Self, error::Error> {
        self.increment(&Period::Day(1))
    }

    pub fn previous_day(&self) -> Result<Self, error::Error> {
        self.decrement(&Period::Day(1))
    }

    pub fn next_month(&self) -> Result<Self, error::Error> {
        self.increment(&Period::Month(1))
    }

    pub fn previous_month(&self) -> Result<Self, error::Error> {
        self.decrement(&Period::Month(1))
    }

    pub fn next_year(&self) -> Result<Self, error::Error> {
        self.increment(&Period::Year(1))
    }

    pub fn previous_year(&self) -> Result<Self, error::Error> {
        self.decrement(&Period::Year(1))
    }

    /// Returns the number of days into the year for the 1st of the given month.
    fn get_month_offset(month: Month, is_leap: bool) -> u32 {
        const OFFSETS: [u32; 12] = [
            0, 31, 59, 90, 120, 151, // Jan - Jun
            181, 212, 243, 273, 304, 334, // Jun - Dec
        ];

        const LEAP_OFFSETS: [u32; 12] = [
            0, 31, 60, 91, 121, 152, // Jan - Jun
            182, 213, 244, 274, 305, 335, // Jun - Dec
        ];

        if is_leap {
            LEAP_OFFSETS[month as usize]
        } else {
            OFFSETS[month as usize]
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

        const OFFSETS: [u32; 301] = [
            0, 366, 731, 1096, 1461, 1827, 2192, 2557, 2922, 3288, // 1900-1909
            3653, 4018, 4383, 4749, 5114, 5479, 5844, 6210, 6575, 6940, // 1910-1919
            7305, 7671, 8036, 8401, 8766, 9132, 9497, 9862, 10227, 10593, // 1920-1929
            10958, 11323, 11688, 12054, 12419, 12784, 13149, 13515, 13880, 14245, // 1930-1939
            14610, 14976, 15341, 15706, 16071, 16437, 16802, 17167, 17532, 17898, // 1940-1949
            18263, 18628, 18993, 19359, 19724, 20089, 20454, 20820, 21185, 21550, // 1950-1959
            21915, 22281, 22646, 23011, 23376, 23742, 24107, 24472, 24837, 25203, // 1960-1969
            25568, 25933, 26298, 26664, 27029, 27394, 27759, 28125, 28490, 28855, // 1970-1979
            29220, 29586, 29951, 30316, 30681, 31047, 31412, 31777, 32142, 32508, // 1980-1989
            32873, 33238, 33603, 33969, 34334, 34699, 35064, 35430, 35795, 36160, // 1990-1999
            36525, 36891, 37256, 37621, 37986, 38352, 38717, 39082, 39447, 39813, // 2000-2009
            40178, 40543, 40908, 41274, 41639, 42004, 42369, 42735, 43100, 43465, // 2010-2019
            43830, 44196, 44561, 44926, 45291, 45657, 46022, 46387, 46752, 47118, // 2020-2029
            47483, 47848, 48213, 48579, 48944, 49309, 49674, 50040, 50405, 50770, // 2030-2039
            51135, 51501, 51866, 52231, 52596, 52962, 53327, 53692, 54057, 54423, // 2040-2049
            54788, 55153, 55518, 55884, 56249, 56614, 56979, 57345, 57710, 58075, // 2050-2059
            58440, 58806, 59171, 59536, 59901, 60267, 60632, 60997, 61362, 61728, // 2060-2069
            62093, 62458, 62823, 63189, 63554, 63919, 64284, 64650, 65015, 65380, // 2070-2079
            65745, 66111, 66476, 66841, 67206, 67572, 67937, 68302, 68667, 69033, // 2080-2089
            69398, 69763, 70128, 70494, 70859, 71224, 71589, 71955, 72320, 72685, // 2090-2099
            73050, 73415, 73780, 74145, 74510, 74876, 75241, 75606, 75971, 76337, // 2100-2109
            76702, 77067, 77432, 77798, 78163, 78528, 78893, 79259, 79624, 79989, // 2110-2119
            80354, 80720, 81085, 81450, 81815, 82181, 82546, 82911, 83276, 83642, // 2120-2129
            84007, 84372, 84737, 85103, 85468, 85833, 86198, 86564, 86929, 87294, // 2130-2139
            87659, 88025, 88390, 88755, 89120, 89486, 89851, 90216, 90581, 90947, // 2140-2149
            91312, 91677, 92042, 92408, 92773, 93138, 93503, 93869, 94234, 94599, // 2150-2159
            94964, 95330, 95695, 96060, 96425, 96791, 97156, 97521, 97886, 98252, // 2160-2169
            98617, 98982, 99347, 99713, 100078, 100443, 100808, 101174, 101539,
            101904, // 2170-2179
            102269, 102635, 103000, 103365, 103730, 104096, 104461, 104826, 105191,
            105557, // 2180-2189
            105922, 106287, 106652, 107018, 107383, 107748, 108113, 108479, 108844,
            109209, // 2190-2199
            109574, // 2200
        ];

        Ok(OFFSETS[year as usize - 1900])
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

        Period::Day(days as u32)
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

    pub fn next(self) -> Self {
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

    pub fn previous(self) -> Self {
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
    pub fn next(&self) -> Self {
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
