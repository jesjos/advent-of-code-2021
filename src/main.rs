use crate::day_1::{count_increases, count_windowed_increases};
use crate::day_2::{travel, travel_part2};
use crate::day_3::{power_consumption, life_support_rating};
use crate::day_4::{day4_part1};

mod day_1;
mod day_2;
mod day_3;
mod day_4;

fn main() {
    let day1_input = include_str!("day1.txt");
    println!("Day 1 part 1 result: {}", count_increases(day1_input));
    println!("Day 1 part 2 result: {}", count_windowed_increases(day1_input));

    let day2_input = include_str!("day2.txt");
    println!("Day 2 {}", travel(day2_input));
    println!("Day2 part 2: {}", travel_part2(day2_input));

    let day3_input = include_str!("day3.txt");
    println!("Day 3 part 1 result: {}", power_consumption(day3_input));
    println!("Day 3 part 2 result: {}", life_support_rating(day3_input));

    let day4_input = include_str!("day4.txt");
    println!("Day 4 part 1 result: {}", day4_part1(day4_input));
}
