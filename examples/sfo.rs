use airports::Airports;
use chrono::{DateTime, TimeZone, Utc};
use chrono_tz::Tz;

fn main() {
    let db = Airports::new();
    println!("SFO timezone: {:?}", db.get_tz_name("sfo").unwrap());

    match db.get_tz("SFO") {
        Some(t) => {
            let x: DateTime<Tz> = Utc::now().with_timezone(&t);
            println!("Current time in SFO: {}", x);
        }
        None => println!("Not found!"),
    }
}
