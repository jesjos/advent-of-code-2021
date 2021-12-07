mod day_1;
mod day_2;
mod day_3;

fn main() {
    let content = include_str!("day1.txt");
    println!("Result {}", day_1::count_increases(&content))
}
