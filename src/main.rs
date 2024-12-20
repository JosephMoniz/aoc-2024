mod common;
mod day1;
mod day10;
mod day11;
mod day12;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

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

    println!("Day8a: {}", day8::a(include_bytes!("../inputs/day8.txt")));
    println!("Day8b: {}", day8::b(include_bytes!("../inputs/day8.txt")));

    println!("Day9a: {}", day9::a(include_bytes!("../inputs/day9.txt")));
    println!("Day9b: {}", day9::b(include_bytes!("../inputs/day9.txt")));

    println!(
        "Day10a: {}",
        day10::a(include_bytes!("../inputs/day10.txt"))
    );
    println!(
        "Day10b: {}",
        day10::b(include_bytes!("../inputs/day10.txt"))
    );

    println!(
        "Day11a: {}",
        day11::a(include_bytes!("../inputs/day11.txt"))
    );
    println!(
        "Day11b: {}",
        day11::b(include_bytes!("../inputs/day11.txt"))
    );

    println!(
        "Day12a: {}",
        day12::a(include_bytes!("../inputs/day12.txt"))
    );
    println!(
        "Day12b: {}",
        day12::b(include_bytes!("../inputs/day12.txt"))
    );
}
