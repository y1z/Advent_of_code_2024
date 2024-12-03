use crate::utility;

pub fn start() {
    let data = utility::read_file("files/day_01.txt");
    let iterator = data.split_whitespace();
    let mut index: usize = 0;
    let mut left_list: Vec<u32> = Vec::new();
    let mut right_list: Vec<u32> = Vec::new();

    for part in iterator {
        if index % 2 == 0 {
            let left_value = part.parse::<u32>().unwrap();
            left_list.push(left_value);
        } else {
            let right_value = part.parse::<u32>().unwrap();
            right_list.push(right_value);
        }
        index += 1;
    }

    left_list.sort();
    right_list.sort();

    index = 0;
    let mut total_distance: u32 = 0;
    for value in left_list {
        //println!("{}  {}", value, right_list[index]);
        total_distance += right_list[index].abs_diff(value);
        index += 1;
    }
    print!("total distance = {}", total_distance);
}
