//! Calendar date and month types, backed by either `chrono` or `jiff`.
//!
//! The backing library is selected with cargo features:
//!
//! * `chrono` (default) - [`Month`] and [`NaiveDate`] are re-exports of `chrono::Month` and
//!   `chrono::NaiveDate`.
//! * `jiff` - [`NaiveDate`] is an alias for `jiff::civil::Date` and [`Month`] is a local
//!   compatibility enum, because `jiff` represents months as plain integers.
//!
//! Exactly one of the two features must be enabled. Enable `jiff` by turning off default features:
//!
//! ```toml
//! credential-exchange-format = { version = "0.4", default-features = false, features = ["jiff"] }
//! ```

#[cfg(feature = "chrono")]
pub use chrono::{Month, NaiveDate};

#[cfg(all(feature = "jiff", not(feature = "chrono")))]
pub use self::jiff_impl::{Month, NaiveDate};

#[cfg(not(any(feature = "chrono", feature = "jiff")))]
compile_error!("either the `chrono` or the `jiff` feature must be enabled");

/// Formats a date using the `yyyy-mm-dd` format expected by the `date` field type.
pub(crate) fn date_to_string(date: &NaiveDate) -> String {
    #[cfg(feature = "chrono")]
    {
        date.format("%Y-%m-%d").to_string()
    }
    #[cfg(all(feature = "jiff", not(feature = "chrono")))]
    {
        date.to_string()
    }
}

/// Parses a date written in the `yyyy-mm-dd` format expected by the `date` field type.
pub(crate) fn date_from_str(value: &str) -> Result<NaiveDate, ()> {
    value.parse().map_err(|_| ())
}

/// Returns the month number in the range `1..=12`.
pub(crate) fn month_number(month: Month) -> u8 {
    #[cfg(feature = "chrono")]
    {
        month.number_from_month() as u8
    }
    #[cfg(all(feature = "jiff", not(feature = "chrono")))]
    {
        month.number_from_month()
    }
}

/// Builds a date from its calendar components, used by tests to stay backend agnostic.
#[cfg(test)]
pub(crate) fn date_from_ymd(year: i16, month: u8, day: u8) -> Option<NaiveDate> {
    #[cfg(feature = "chrono")]
    {
        NaiveDate::from_ymd_opt(i32::from(year), u32::from(month), u32::from(day))
    }
    #[cfg(all(feature = "jiff", not(feature = "chrono")))]
    {
        NaiveDate::new(year, month as i8, day as i8).ok()
    }
}

/// `jiff` backed implementations of the public calendar types.
#[cfg(all(feature = "jiff", not(feature = "chrono")))]
mod jiff_impl {
    use core::{error::Error, fmt};

    /// A Gregorian calendar date, backed by [`jiff::civil::Date`].
    pub type NaiveDate = jiff::civil::Date;

    /// A month of the year.
    ///
    /// `jiff` stores months as `i8`s, so this enum mirrors the shape of `chrono::Month` to keep
    /// this crate's public API identical regardless of which backend is selected.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    #[repr(u8)]
    #[non_exhaustive]
    pub enum Month {
        January = 1,
        February = 2,
        March = 3,
        April = 4,
        May = 5,
        June = 6,
        July = 7,
        August = 8,
        September = 9,
        October = 10,
        November = 11,
        December = 12,
    }

    impl Month {
        /// Returns the month number in the range `1..=12`.
        #[inline]
        pub const fn number_from_month(self) -> u8 {
            self as u8
        }
    }

    impl fmt::Display for Month {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match self {
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
            f.write_str(name)
        }
    }

    /// Error returned when a value falls outside the range of valid months.
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct InvalidMonth(());

    impl fmt::Display for InvalidMonth {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.write_str("month must be in the range 1..=12")
        }
    }

    impl Error for InvalidMonth {}

    impl TryFrom<u8> for Month {
        type Error = InvalidMonth;

        fn try_from(value: u8) -> Result<Month, Self::Error> {
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
                _ => Err(InvalidMonth(())),
            }
        }
    }

    impl TryFrom<i8> for Month {
        type Error = InvalidMonth;

        fn try_from(value: i8) -> Result<Month, Self::Error> {
            u8::try_from(value)
                .map_err(|_| InvalidMonth(()))
                .and_then(Month::try_from)
        }
    }

    impl From<Month> for u8 {
        #[inline]
        fn from(month: Month) -> u8 {
            month.number_from_month()
        }
    }

    impl From<Month> for i8 {
        #[inline]
        fn from(month: Month) -> i8 {
            month as i8
        }
    }
}
