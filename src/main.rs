mod day1;
mod day2;

fn main() {
    println!("Day1a: {}", day1::a(include_bytes!("../inputs/day1.txt")));
    println!("Day1b: {}", day1::b(include_bytes!("../inputs/day1.txt")));

    println!("Day2a: {}", day2::a(include_bytes!("../inputs/day2.txt")));
    println!("Day2b: {}", day2::b(include_bytes!("../inputs/day2.txt"))); // 569
}
