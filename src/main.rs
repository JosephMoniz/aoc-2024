mod day1;
mod day2;
mod day3;

fn main() {
    println!("Day1a: {}", day1::a(include_bytes!("../inputs/day1.txt")));
    println!("Day1b: {}", day1::b(include_bytes!("../inputs/day1.txt")));

    println!("Day2a: {}", day2::a(include_bytes!("../inputs/day2.txt")));
    println!("Day2b: {}", day2::b(include_bytes!("../inputs/day2.txt")));

    println!("Day3a: {}", day3::a(include_bytes!("../inputs/day3.txt")));
    println!("Day3b: {}", day3::b(include_bytes!("../inputs/day3.txt")));
}
