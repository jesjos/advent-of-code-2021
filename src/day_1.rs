pub fn count_increases(data_points: &str) -> u32 {
    let holder = ConsecutiveCounter::new();
    let result = data_points.lines()
        .map(|maybe_number| maybe_number.parse::<u32>().unwrap())
        .fold(holder, |acc, num| acc.handle(num));
    result.increases
}

pub fn count_windowed_increases(data_points: &str) -> usize {
    data_points.lines()
        .filter_map(|n| n.parse().ok())
        .collect::<Vec<u32>>()
        .windows(3)
        .map(|window| window.iter().sum::<u32>())
        .collect::<Vec<u32>>()
        .windows(2)
        .filter(|a| a[0] < a[1])
        .count()

}

#[derive(Debug)]
struct ConsecutiveCounter {
    pub current: Option<u32>,
    pub increases: u32
}

impl ConsecutiveCounter {
    fn new() -> ConsecutiveCounter {
        ConsecutiveCounter { current: Option::None, increases: 0 }
    }
    fn handle(mut self, num: u32) -> ConsecutiveCounter {
        let cur = self.current.unwrap_or(num);
        if num > cur {
            self.increases += 1;
        }
        self.current = Option::Some(num);
        self
    }
}


#[cfg(test)]
mod tests {
    use crate::day_1::{count_increases, count_windowed_increases};

    #[test]
    fn test_one_increase() {
        let text =
            "1\n\
             2\n\
             1\n";
        let result = count_increases(text);
        assert_eq!(1, result);
    }

    #[test]
    fn test_zero_increases() {
        let text =
            "1\n\
             1\n\
             1\n";
        let result = count_increases(text);
        assert_eq!(0, result);
    }

    #[test]
    fn test_many_increases() {
        let text =
            "1\n\
             2\n\
             3\n";
        let result = count_increases(text);
        assert_eq!(2, result);
    }

    #[test]
    fn get_the_final_count() {
        let text = include_str!("day1.txt");
        println!("Day 1 result: {}", count_increases(text));
    }

    #[test]
    fn test_empty_window() {
        let text = "";
        assert_eq!(0, count_windowed_increases(text));
    }

    #[test]
    fn test_small_window() {
        let text =
            "1\n\
             1\n\
             1\n\
             2";
        let result = count_windowed_increases(text);
        assert_eq!(1, result);
    }

    #[test]
    fn get_part2_count() {
        let text = include_str!("day1.txt");
        println!("Day 1 part 2 result: {}", count_windowed_increases(text))
    }
}
