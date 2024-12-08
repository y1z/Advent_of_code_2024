use regex::Regex;

use crate::utility;

/* (mul)\(\d{1,},\d{1,}\) */
pub fn start() {
    let data = utility::read_file("files/day_03_test.txt");
    let regex_pattern = regex::Regex::new(r"(mul)\(\d{1,},\d{1,}\)").unwrap();

    for i in regex_pattern.find_iter(&data) {
        println!("{}", i.as_str());
    }

    //if let Ok(regex_pattern) = regex::Regex::new(r"(mul)\(\d{1,},\d{1,}\)") {}

    //print!("{}", data);
}
