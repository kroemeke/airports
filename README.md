# airports
Crate to map airport IATA code to timezone

## Example
```
use airports::Airports;

fn main() {
    let db = Airports::new();
    println!("lhr: {:?}", db.get_tz_name("lhr"));
    println!("LHR: {:?}", db.get_tz_name("LHR"));
    println!("SOMETHING: {:?}", db.get_tz_name("SOMETHING"));
}
```

Prints out
```
lhr: Some("Europe/London")
LHR: Some("Europe/London")
SOMETHING: None
```

It is also possible to directly map the IATA code to IANA time zone
and get chrono_tz Tz wrapped in an option for convinience.
```
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
```
```
SFO timezone: "America/Los_Angeles"
Current time in SFO: 2022-06-28 08:31:43.613397 PDT
```
