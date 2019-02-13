extern crate adventofcodelib;

use adventofcodelib::*;

// the path to the files with input data
pub const FILE_NAME_DAY1: &str = "./input_data/day1_data.txt";
pub const FILE_NAME_DAY2: &str = "./input_data/day2_data.txt";
pub const FILE_NAME_DAY3: &str = "./input_data/day3_data.txt";
pub const FILE_NAME_DAY4: &str = "./input_data/day4_data.txt";
pub const FILE_NAME_DAY5: &str = "./input_data/day5_data.txt";

fn main() {
    println!("day1 first task={}", day1_task1(FILE_NAME_DAY1));
    println!("day1 second task={}", day1_task2(FILE_NAME_DAY1));
    println!("day2 first task={}", day2_task1(FILE_NAME_DAY2));
    println!("day2 second task={}", day2_task2(FILE_NAME_DAY2));
    println!("day3 first task={}", day3_task1(FILE_NAME_DAY3));
    println!("day3 second task={}", day3_task2(FILE_NAME_DAY3));
    println!("day4 first task={}", day4_task1(FILE_NAME_DAY4));
    println!("day4 second task={}", day4_task2(FILE_NAME_DAY4));
    println!("day5 first task={}", day5_task1(FILE_NAME_DAY5));
    println!("day5 second task={}", day5_task2(FILE_NAME_DAY5));
}
