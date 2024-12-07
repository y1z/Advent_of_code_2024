use crate::utility;

// problem : https://adventofcode.com/2024/day/2
pub fn start() {
    let data = utility::read_file("files/day_02.txt");
    //println!("{}", data);
    let mut safe_counter = 0;
    let mut unsafe_counter = 0;
    let mut line_numbers: Vec<i32> = Vec::new();

    for line in data.lines() {
        let mut is_safe = true;

        for number in line.split_whitespace() {
            line_numbers.push(number.parse::<i32>().unwrap());
        }

        let is_increasing = 0 > line_numbers[0] - line_numbers[1];

        for number_index in 0..line_numbers.len() - 1 {
            let difference = line_numbers[number_index] - line_numbers[number_index + 1];
            // makes sure that if the sequence started by increasing it keeps increasing
            let is_consistent = is_increasing == (0 > difference);

            if difference.abs() > 3 || difference == 0 || !is_consistent {
                unsafe_counter += 1;
                is_safe = false;

                break;
            }
        }

        if is_safe {
            safe_counter += 1;
        }
        line_numbers.clear();
    }
    //println!("{}", data);

    println!("safe amount = {}", safe_counter);
    println!("unsafe amount = {}", unsafe_counter);
}
