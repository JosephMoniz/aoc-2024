mod common;
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;

fn main() {
    println!("Day1a: {}", day1::a(include_bytes!("../inputs/day1.txt")));
    println!("Day1b: {}", day1::b(include_bytes!("../inputs/day1.txt")));

    println!("Day2a: {}", day2::a(include_bytes!("../inputs/day2.txt")));
    println!("Day2b: {}", day2::b(include_bytes!("../inputs/day2.txt")));

    println!("Day3a: {}", day3::a(include_bytes!("../inputs/day3.txt")));
    println!("Day3b: {}", day3::b(include_bytes!("../inputs/day3.txt")));

    println!("Day4a: {}", day4::a(include_bytes!("../inputs/day4.txt")));
    println!("Day4b: {}", day4::b(include_bytes!("../inputs/day4.txt")));

    println!("Day5a: {}", day5::a(include_bytes!("../inputs/day5.txt")));
    println!("Day5b: {}", day5::b(include_bytes!("../inputs/day5.txt")));

    println!("Day6a: {}", day6::a(include_bytes!("../inputs/day6.txt")));
    println!("Day6b: {}", day6::b(include_bytes!("../inputs/day6.txt")));

    println!("Day7a: {}", day7::a(include_bytes!("../inputs/day7.txt")));
    println!("Day7b: {}", day7::b(include_bytes!("../inputs/day7.txt")));
}
