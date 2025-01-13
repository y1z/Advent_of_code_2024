use crate::utility;
use regex::Regex;

pub fn start() {
    let data = utility::read_file("files/day_03.txt");
    let re = Regex::new(r"(mul\((\d{1,}),(\d{1,})\))").unwrap();
    let mut total = 0 as i32;

    // the 2 `_` are the entire captured substring
    for (_, [_, first_num, second_num]) in re.captures_iter(&data).map(|c| c.extract()) {

        //println!("{} {}", first_num, second_num);
        let val_first_num:i32 = first_num.parse().unwrap();
        let val_second_num:i32 = second_num.parse().unwrap();
        total += val_first_num * val_second_num;
    }
    println!("{}",total);

    //println!("{}", data);
}
