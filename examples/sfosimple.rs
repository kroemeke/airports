use airports::Airports;
use chrono::{DateTime, Utc};
use chrono_tz::Tz;

fn main() {
    let db = Airports::new();
    let x: DateTime<Tz> = Utc::now().with_timezone(&db.get_tz("SFO").unwrap());
    println!("Time in SFO: {}", x);
}
