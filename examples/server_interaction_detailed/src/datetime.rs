use std::cmp::Ordering;
use std::convert::TryInto;
use std::fmt;

use chrono;
use chrono::offset::TimeZone;
use chrono::Datelike;
use js_sys;

type Cdate = chrono::Date<chrono::Utc>;

/// Wrap the Chrono date, with a simpler API, and compatibility with features not supported
/// for it with the wasm target.
/// For now, utc only.
///
/// 1-based month and day indexing.
#[derive(Clone)]
pub struct Date {
    wrapped: Cdate,
}
//
//impl Ord for Date {
//    fn cmp(&self, other: &Date) -> Ordering {
////        self.height.cmp(&other.height)
//
//        let year_diff = self.year as u16 - other.year as u16;
//        if year_diff > 0 {
//            return self.year.cmp(other.year)
//        } else if year_diff < 0 {
//            return other.year.cmp(self.year)
//        } else {
//
//            let month_diff = self.month as i8 - othermonth as i8;
//            if month_diff > 0 {
//                return self.month.cmp(other.month)
//            } else if month_diff < 0 {
//                return other.month.cmp(self.month)
//            } else {
//
//                let day = self.day as i8 - other.day as i8;
//                if day_diff > 0 {
//                    return self.day.cmp(other.day)
//                } else if day_diff < 0 {
//                    return other.day.cmp(self.day)
//                }
//            }
//        }
//    }
//    return self.day.cmp(other.day)  // they equal eaqch other.
//}

impl PartialEq for Date {
    fn eq(&self, other: &Self) -> bool {
        self.wrapped == other.wrapped
    }
}

impl PartialOrd for Date {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.wrapped.cmp(&other.wrapped))
    }
}

///// todo From<String>?
//impl From<&str> for Date {
//    fn from(iso_date: &str) -> Self {
//        // todo regex? Validation?
//        let date = iso_date.to_string();
//
//        let year = &date[0..4];
//        let month = &date[7..9];
//        let day = &date[11..13];
//
//        Self {
//            year: year.parse::<u16>().expect("Problem parsing year"),
//            month: month.parse::<u8>().expect("Problem parsing month"),
//            day: day.parse::<u8>().expect("Problem parsing day"),
//        }
//    }
//}

impl fmt::Debug for Date {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{:?}, {:?}",
            self.wrapped,
            self.wrapped.format("%Y-%h-%D").to_string()
        )
    }
}

//impl Add for Date {
//    type Output = Self;
//
//    fn add(self, dur: chrono::Duration) -> Self {
//        Self {
//            wrapped: self.wrapped + dur
//        }
//    }
//}
//
//impl Sub for Date {
//    type Output = Self;
//
//    fn sub(self, dur: chrono::Duration) -> Self {
//        Self {
//            wrapped: self.wrapped - dur
//        }
//    }
//}

impl From<Cdate> for Date {
    fn from(date: Cdate) -> Self {
        Self { wrapped: date }
    }
}

impl Date {
    pub fn new(year: i32, month: u32, day: u32) -> Self {
        Self {
            wrapped: chrono::Utc.ymd(year, month, day),
        }
    }

    /// We use js_sys::Date, serialize it, then turn it into a Chrono date, due to limitations
    /// with Crono on the wasm target.
    pub fn today() -> Self {
        let today_js = js_sys::Date::new_0();

        Self::new(
            today_js
                .get_utc_full_year()
                .try_into()
                .expect("casting js year into chrono year failed"),
            today_js.get_utc_month() as u32 + 1, // JS using 0-based month indexing. Fix it.
            today_js.get_utc_date() as u32,
        )
    }

    /// Convert an iso-format date, eg "2019-01-05" for January 5, 2019" to a Date.
    pub fn from_iso(date: &str) -> Self {
        // Chrono needs time too, and can't parse date-only directly.
        let padded = &(date.to_string() + "T000000");
        Self {
            wrapped: chrono::Utc
                .datetime_from_str(padded, "%Y-%m-%dT%H%M%S")
                .unwrap_or_else(|_| panic!("Can't format date: {}", date))
                .date(),
        }
    }

    // todo operator overload with other being diff type?
    pub fn add(&self, dur: chrono::Duration) -> Self {
        Self {
            wrapped: self.wrapped + dur,
        }
    }

    pub fn sub(&self, dur: chrono::Duration) -> Self {
        Self {
            wrapped: self.wrapped - dur,
        }
    }

    /// Getters/setters. Perhaps there's a better way.
    pub fn year(&self) -> u16 {
        self.wrapped
            .year()
            .try_into()
            .expect("casting chrono year (i32) into u16 failed")
    }
}
