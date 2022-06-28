use airports::Airports;

fn main() {
    let db = Airports::new();
    println!("lhr: {:?}", db.get_tz_name("lhr"));
    println!("LHR: {:?}", db.get_tz_name("LHR"));
    println!("SOMETHING: {:?}", db.get_tz_name("SOMETHING"));
}
