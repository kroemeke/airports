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

//#![warn(missing_docs)]
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
}
