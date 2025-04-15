use libra::time::Date;

fn main() {
    let d = match Date::from_parts(12, libra::time::Month::January, 2005) {
        Ok(d) => d,
        Err(err) => panic!("{}", err),
    };

    println!("{}", d.easter_monday(false));
}
