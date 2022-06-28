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
