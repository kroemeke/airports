//! Easy mapping between IATA airport codes and timezones + cooridnates.
//!
//! Provides mappings between IATA airport codes like "LHR" / "SFO" to
//! non-abbreviated timezone name like "Europe/London". Also includes
//! ability to get DateTime object in the given time zone for convience.
//!
//! Actual mapping has been assambled from various sources available online
//! and spot check validated of around 400 locations.
//!
//! ## Examples
//!
//! Fetch time zone string of the LHR (London) airport.
//!
//! ```
//! use airports::Airports;
//!
//! fn main() {
//!    let db = Airports::new();
//!    println!("lhr: {:?}", db.get_tz_name("lhr"));
//!    println!("LHR: {:?}", db.get_tz_name("LHR"));
//!    println!("SOMETHING: {:?}", db.get_tz_name("SOMETHING"));
//! }
//! ```
//!
//! Which produces the following output:
//! ```
//! lhr: Some("Europe/London")
//! LHR: Some("Europe/London")
//! SOMETHING: None
//! ```
//!
//! The get_tz function returns Option on Tz allowing easy check
//! of time in the given airport location. See examples/ for more details.
//!
//! ```
//!    let db = Airports::new();
//!    let x: DateTime<Tz>= Utc::now().with_timezone(&db.get_tz("SFO").unwrap());
//!    println!("Time in SFO: {}", x);
//! ```
//! ```
//! Time in SFO: 2022-06-28 08:27:11.605433 PDT
//! ```

//#![warn(missing_docs)]
use chrono_tz::Tz;
use std::collections::BTreeMap;

#[derive(Clone, Debug)]
pub struct Airports {
    db: BTreeMap<String, String>,
}

impl Default for Airports {
    fn default() -> Self {
        Self::new()
    }
}

impl Airports {
    pub fn new() -> Self {
        let db: &'static str = include_str!("airports.yaml");
        let ddb: BTreeMap<String, String> = serde_yaml::from_str(db).unwrap();
        Airports { db: ddb }
    }

    /// Returns an Option on String, with String containing the timezone name,
    /// or None if mapping not found.
    ///
    /// # Example
    ///
    /// ```
    /// let lhr_timezone = db.get_tz_name("LHR");
    /// ```
    pub fn get_tz_name(&self, code: &str) -> Option<&String> {
        self.db.get(&code.to_uppercase())
    }

    /// Returns an Option on Tz (from chrono_tz) for given IATA airport code.
    /// This can be then passed to chrono/DataTime to get time using specific
    /// time zone.
    ///
    /// # Example
    ///
    /// use airports::Airports;
    /// use chrono::{DateTime, Utc};
    /// use chrono_tz::Tz;
    ///
    /// fn main() {
    ///    let db = Airports::new();
    ///    let x: DateTime<Tz>= Utc::now().with_timezone(&db.get_tz("SFO").unwrap());
    ///    println!("Time in SFO: {}", x);
    ///}
    pub fn get_tz(&self, code: &str) -> Option<Tz> {
        match self.get_tz_name(code) {
            Some(t) => {
                let tz: Tz = t.parse().expect("Failed to parse timezone.");
                Some(tz)
            }
            None => None,
        }
    }
}
