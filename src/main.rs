mod day1;

fn main() {
    println!("Day1a: {}", day1::a(include_bytes!("../inputs/day1.txt")));
    println!("Day1b: {}", day1::b(include_bytes!("../inputs/day1.txt")));
}
