use regex::Regex;
use time::now;

fn main() {
    println!("Hello, world!");
    let re = Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();
    println!("{}", re);
    println!("Did our date match? {}", re.is_match("2014-01-01"));
    let fmt = format!("{:?}", now());
    println!("{}", fmt)
    
}
