use std::alloc::Layout;
use std::fmt::{Debug, Display, Formatter};
use std::ops::{Add, Deref, DerefMut, Sub};
use std::time::SystemTime;
use once_cell::sync::Lazy;
use serde::de::Error;
use time::{format_description, OffsetDateTime, UtcOffset};
use time::error::InvalidFormatDescription;
use time::format_description::FormatItem;
use crate::std::time::format::{longDayNames, longMonthNames};
use crate::std::errors::Result;
use crate::std::time::sys::Timespec;

/// "Mon, 02 Jan 2006 15:04:05 GMT"
pub const TimeFormat: &'static str = "[weekday], [day] [month] [year] [hour]:[minute]:[second] GMT";
/// "2006-01-02T15:04:05Z07:00"
pub const RFC3339: &'static str = "[year]-[month]-[day]T[hour]:[minute]:[second][offset_hour sign:mandatory]:[offset_minute]";
///"2006-01-02T15:04:05.999999999Z07:00"
pub const RFC3339Nano: &'static str = "[year]-[month]-[day]T[hour]:[minute]:[second].[subsecond][offset_hour sign:mandatory]:[offset_minute]";
///"Mon, 02 Jan 2006 15:04:05 MST"
pub const RFC1123: &'static str = "[weekday repr:short], [day] [month repr:short] [year] [hour]:[minute]:[second] MST";

/// Obtain the offset of Utc time and Local time in seconds, using Lazy only once to improve performance
pub static GLOBAL_OFFSET: Lazy<UtcOffset> = Lazy::new(|| {
    UtcOffset::from_whole_seconds(Timespec::now().local().tm_utcoff).unwrap()
});

/// a time wrapper just like golang
#[derive(Eq, PartialEq, Ord, PartialOrd)]
pub struct Time {
    pub inner: OffsetDateTime,
}

impl Time {
    pub fn unix_timestamp(&self) -> i64 {
        self.inner.unix_timestamp()
    }

    pub fn unix_timestamp_nano(&self) -> i64 {
        self.inner.unix_timestamp_nanos() as i64
    }

    // unix_sec returns the time's seconds since Jan 1 1970 (Unix time).
    pub fn unix(&self) -> i64 {
        self.inner.unix_timestamp()
    }

    // unix_sec returns the time's seconds since Jan 1 1970 (Unix time).
    pub fn unix_nano(&self) -> i64 {
        self.inner.unix_timestamp_nanos() as i64
    }

    pub fn add(&mut self, d: std::time::Duration) {
        self.inner = self.inner.add(d);
    }

    // add_sec adds d seconds to the time.
    pub fn add_sec(&mut self, d: i64) {
        self.inner = self.inner.add(time::Duration::seconds(d));
    }

    /// set_loc sets the location associated with the time.
    pub fn set_loc(&mut self, loc: time::UtcOffset) {
        self.inner = self.inner.to_offset(loc);
    }

    /// after reports whether the time instant t is after u.
    pub fn after(&self, u: &Time) -> bool {
        self.inner.sub(u.inner).is_negative()
    }

    /// before reports whether the time instant t is before u.
    pub fn before(&self, u: &Time) -> bool {
        self.inner.sub(u.inner).is_positive()
    }


    /// equal reports whether t and u represent the same time instanself.
    /// Two times can be equal even if they are in different locations.
    /// For example, 6:00 +0200 and 4:00 UTC are equal.
    /// See the documentation on the Time type for the pitfalls of using == with
    /// Time values; most code should use equal instead.
    pub fn equal(&self, u: &Time) -> bool {
        self.inner.eq(&u.inner)
    }


    /// is_zero reports whether t represents the zero time instant,
    /// January 1, year 1, 00:00:00 UTC.
    pub fn is_zero(&self) -> bool {
        return self.unix_timestamp() == 0;
    }

    /// date returns the (year, month,  day) in which t occurs.
    pub fn date(&self) -> (i32, Month, i32) {
        let d = self.inner.date();
        return (d.year(), d.month().into(), d.day() as i32);
    }

    /// year
    pub fn year(&self) -> i32 {
        self.inner.date().year()
    }

    /// month
    pub fn month(&self) -> Month {
        self.inner.month().into()
    }

    /// day
    pub fn day(&self) -> i32 {
        self.inner.day() as i32
    }

    /// weekday
    pub fn weekday(&self) -> Weekday {
        self.inner.weekday().into()
    }
    /// iso_week
    pub fn iso_week(&self) -> (i32, i32) {
        (self.inner.year(), self.inner.iso_week() as i32)
    }
    /// hour
    pub fn hour(&self) -> i32 {
        self.inner.hour().into()
    }

    /// minute
    pub fn minute(&self) -> i32 {
        self.inner.minute().into()
    }

    /// minute
    pub fn second(&self) -> i32 {
        self.inner.second().into()
    }

    /// millisecond
    pub fn millisecond(&self) -> i32 {
        self.inner.millisecond() as i32
    }

    /// microsecond
    pub fn microsecond(&self) -> i32 {
        self.inner.microsecond() as i32
    }

    /// nanosecond
    pub fn nanosecond(&self) -> i32 {
        self.inner.nanosecond() as i32
    }

    /// for example:
    /// "[year]-[month] [ordinal] [weekday] [week_number]-[day] [hour]:[minute] [period]:[second].[subsecond] [offset_hour sign:mandatory]:[offset_minute]:[offset_second]"
    ///
    pub fn format(&self, layout: &str) -> String {
        let f = {
            match format_description::parse(layout) {
                Ok(v) => {
                    v
                }
                Err(_) => {
                    vec![]
                }
            }
        };
        if f.is_empty() {
            return String::new();
        }
        self.inner.format(&f).unwrap_or_default()
    }

    /// for example:
    /// "[year]-[month] [ordinal] [weekday] [week_number]-[day] [hour]:[minute] [period]:[second].[subsecond] [offset_hour sign:mandatory]:[offset_minute]:[offset_second]"
    ///
    pub fn parse(layout: &str, value: &str) -> Result<Self> {
        match format_description::parse(layout) {
            Ok(v) => {
                Ok(Self {
                    inner: time::OffsetDateTime::parse(value, &v)?
                })
            }
            Err(e) => {
                return Err(e.into());
            }
        }
    }

    // now returns the current local time.
    pub fn now() -> Time {
        let mut now = time::OffsetDateTime::now_utc();
        now = now.to_offset(GLOBAL_OFFSET.clone());
        return Time {
            inner: now
        };
    }

    pub fn now_utc() -> Time {
        let mut now = time::OffsetDateTime::now_utc();
        return Time {
            inner: now
        };
    }
}

impl Debug for Time {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.inner, f)
    }
}

impl Display for Time {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.format(RFC3339Nano), f)
    }
}

impl serde::Serialize for Time {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer {
        serializer.serialize_str(&self.format(RFC3339Nano))
    }
}

impl<'de> serde::Deserialize<'de> for Time {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de> {
        match Time::parse(RFC3339Nano, &String::deserialize(deserializer)?) {
            Ok(v) => { Ok(v) }
            Err(e) => {
                Err(D::Error::custom(e.to_string()))
            }
        }
    }
}

// A month specifies a month of the year (January = 1, ...).
#[derive(Eq, PartialEq, Debug, PartialOrd)]
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

impl From<time::Month> for Month {
    fn from(arg: time::Month) -> Self {
        match arg {
            time::Month::January => { Month::January }
            time::Month::February => { Month::February }
            time::Month::March => { Month::March }
            time::Month::April => { Month::April }
            time::Month::May => { Month::May }
            time::Month::June => { Month::June }
            time::Month::July => { Month::July }
            time::Month::August => { Month::August }
            time::Month::September => { Month::September }
            time::Month::October => { Month::October }
            time::Month::November => { Month::November }
            time::Month::December => { Month::December }
        }
    }
}


impl From<i64> for Month {
    fn from(arg: i64) -> Self {
        match arg {
            1 => {
                return Month::January;
            }
            2 => {
                return Month::February;
            }
            3 => {
                return Month::March;
            }
            4 => {
                return Month::April;
            }
            5 => {
                return Month::May;
            }
            6 => {
                return Month::June;
            }
            7 => {
                return Month::July;
            }
            8 => {
                return Month::August;
            }
            9 => {
                return Month::September;
            }
            10 => {
                return Month::October;
            }
            11 => {
                return Month::November;
            }
            12 => {
                return Month::December;
            }
            _ => {}
        }
        Month::January
    }
}

impl From<Month> for i64 {
    fn from(arg: Month) -> Self {
        match arg {
            Month::January => { 1 }
            Month::February => { 2 }
            Month::March => { 3 }
            Month::April => { 4 }
            Month::May => { 5 }
            Month::June => { 6 }
            Month::July => { 7 }
            Month::August => { 8 }
            Month::September => { 9 }
            Month::October => { 10 }
            Month::November => { 11 }
            Month::December => { 23 }
        }
    }
}

impl From<&Month> for i64 {
    fn from(arg: &Month) -> Self {
        match arg {
            Month::January => { 1 }
            Month::February => { 2 }
            Month::March => { 3 }
            Month::April => { 4 }
            Month::May => { 5 }
            Month::June => { 6 }
            Month::July => { 7 }
            Month::August => { 8 }
            Month::September => { 9 }
            Month::October => { 10 }
            Month::November => { 11 }
            Month::December => { 23 }
        }
    }
}

impl Month {
    pub fn i64(&self) -> i64 {
        self.into()
    }
    // String returns the English name of the month ("January", "February", ...).
    pub fn String(&self) -> String {
        if Month::January <= *self && *self <= Month::December {
            return longMonthNames[(self.i64() - 1) as usize].to_string();
        }
        let mut buf = Vec::with_capacity(20);
        for _ in 0..20 {
            buf.push(0);
        }
        let n = fmtInt(&mut buf, self.i64() as u64) as usize;
        return "%!month(".to_string() + &String::from_utf8(buf[n..].to_vec()).unwrap_or_default() + ")";
    }
}

// fmtInt formats v into the tail of buf.
// It returns the index where the output begins.
fn fmtInt(buf: &mut Vec<u8>, mut v: u64) -> i64 {
    let mut w = buf.len();
    if v == 0 {
        w -= 1;
        buf[w] = '0' as u8;
    } else {
        while v > 0
        {
            w -= 1;
            buf[w] = (v % 10) as u8 + '0' as u8;
            v /= 10
        }
    }
    return w as i64;
}

#[derive(Eq, PartialEq, Debug, PartialOrd)]
pub enum Weekday {
    Sunday = 0,
    Monday = 1,
    Tuesday = 2,
    Wednesday = 3,
    Thursday = 4,
    Friday = 5,
    Saturday = 6,
}

impl From<time::Weekday> for Weekday {
    fn from(arg: time::Weekday) -> Self {
        match arg {
            time::Weekday::Monday => { Weekday::Monday }
            time::Weekday::Tuesday => { Weekday::Tuesday }
            time::Weekday::Wednesday => { Weekday::Wednesday }
            time::Weekday::Thursday => { Weekday::Thursday }
            time::Weekday::Friday => { Weekday::Friday }
            time::Weekday::Saturday => { Weekday::Saturday }
            time::Weekday::Sunday => { Weekday::Sunday }
        }
    }
}

impl Weekday {
    pub fn i64(&self) -> i64 {
        match self {
            Weekday::Sunday => { 0 }
            Weekday::Monday => { 1 }
            Weekday::Tuesday => { 2 }
            Weekday::Wednesday => { 3 }
            Weekday::Thursday => { 4 }
            Weekday::Friday => { 5 }
            Weekday::Saturday => { 6 }
        }
    }
    pub fn String(&self) -> String {
        if Weekday::Sunday <= *self && *self <= Weekday::Saturday {
            return longDayNames[self.i64() as usize].to_string();
        }
        let mut buf = Vec::with_capacity(20);
        for _ in 0..20 {
            buf.push(0u8);
        }
        let n = fmtInt(&mut buf, self.i64() as u64);
        return format!("%!weekday({})", String::from_utf8(buf[n as usize..].to_vec()).unwrap_or_default());
    }
}


#[cfg(test)]
mod test {
    use std::time::Duration;
    use crate::coroutine::sleep;
    use crate::std::time::time::{Month, RFC3339, RFC3339Nano, Time, TimeFormat};

    #[test]
    fn test_mon() {
        let m = Month::May;
        println!("{}", m.String());
        assert_eq!("May", m.String());
    }


    #[test]
    fn test_parse() {
        let now = Time::now();
        println!("{}", now.format(TimeFormat));
        println!("{}", now.format(RFC3339));
        println!("{}", now.format(RFC3339Nano));
        assert_eq!(now, Time::parse(RFC3339Nano, &now.format(RFC3339Nano)).unwrap())
    }

    #[test]
    fn test_eq() {
        let now = Time::now();
        sleep(Duration::from_secs(1));
        println!("{}", now.format(RFC3339Nano));
        assert_eq!(false, now.before(&Time::now()));
    }
}
