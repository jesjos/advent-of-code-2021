fn transpose<T>(matrix: &[Vec<T>]) -> Vec<Vec<T>> where T: Clone {
    if matrix.is_empty() {
        return Vec::new()
    }
    (0..matrix[0].len())
        .map(|i| matrix.iter().map(|row| row[i].clone()).collect::<Vec<T>>())
        .collect()
}

fn mean(vec: &[u8]) -> u8 {
    let total: u32 = vec.iter().map(|n| *n as u32).sum();
    let float_mean = f64::from(total) / vec.len() as f64;
    u8::try_from(float_mean.round() as i32).unwrap()
}

fn gamma(input: &str) -> Vec<u8> {
    means(&read_input(input))
}

fn means(matrix: &[Vec<u8>]) -> Vec<u8> {
    let transposed = transpose(matrix);
    transposed.iter().map(|column| mean(column)).collect()
}

fn read_input(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|line| line.split("").filter_map(|char| char.parse::<u8>().ok()).collect::<Vec<u8>>())
        .collect()
}

fn epsilon(input: &[u8]) -> Vec<u8> {
    input.iter().map(|n| (n + 1) % 2).collect()
}

fn binary_string_to_int(input: &[u8]) -> u32 {
    let vec_of_strings = input.iter().map(|number| number.to_string()).collect::<Vec<String>>();
    let string_representation = vec_of_strings.join("");
    u32::from_str_radix(&string_representation, 2).unwrap()
}

pub(crate) fn power_consumption(input: &str) -> u32 {
    let gamma_vec = gamma(input);
    let epsilon_vec = epsilon(&gamma_vec);
    binary_string_to_int(&gamma_vec) * binary_string_to_int(&epsilon_vec)
}

struct OxygenIterator {
    curr_position: usize,
    matrix: Vec<Vec<u8>>,
    flip: u8
}

impl OxygenIterator {
    fn new(matrix: Vec<Vec<u8>>) -> OxygenIterator {
        OxygenIterator { curr_position: 0, flip: 0, matrix }
    }

    fn negating(matrix: Vec<Vec<u8>>) -> OxygenIterator {
        OxygenIterator { curr_position: 0, flip: 1, matrix }
    }

    fn filter(&mut self) {
        let to_keep = bit_to_keep(&self.matrix, self.curr_position);
        let to_keep = (to_keep + self.flip) % 2;
        let new_matrix: Vec<Vec<u8>> = self.matrix.iter()
            .filter(|row| row[self.curr_position] == to_keep)
            // .map(|vec| vec.clone())
            .cloned()
            .collect::<Vec<Vec<u8>>>();
        self.matrix = new_matrix;
        self.curr_position += 1;
    }
}

impl Iterator for OxygenIterator {
    type Item = Vec<Vec<u8>>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.matrix.len() > 1 {
            self.filter();
            Option::Some(self.matrix.clone())
        } else {
            Option::None
        }
    }
}

fn oxygen_generator_rating(input: &str) -> u32 {
    let matrix = read_input(input);
    let iterator = OxygenIterator::new(matrix);
    calculate_rating(iterator)
}

fn co2_scrubber_rating(input: &str) -> u32 {
    let matrix = read_input(input);
    let iterator = OxygenIterator::negating(matrix);
    calculate_rating(iterator)
}

fn calculate_rating(iterator: OxygenIterator) -> u32 {
    let last_iteration = iterator.last().unwrap();
    let remaining_value = last_iteration.first().unwrap();
    let owned = remaining_value.to_vec();
    binary_string_to_int(&owned)
}

pub(crate) fn life_support_rating(input: &str) -> u32 {
    oxygen_generator_rating(input) * co2_scrubber_rating(input)
}

fn bit_to_keep(matrix: &[Vec<u8>], position: usize) -> u8 {
    let row_means = means(matrix);
    row_means[position]
}

#[cfg(test)]
mod tests {
    use crate::day_3::{bit_to_keep, co2_scrubber_rating, epsilon, gamma, life_support_rating, mean, oxygen_generator_rating, OxygenIterator, power_consumption, transpose};

    #[test]
    fn test_gamma_rate() {
        let example_input = include_str!("day3_example.txt");
        assert_eq!(vec![1,0,1,1,0], gamma(example_input));
    }

    #[test]
    fn test_epsilon_rate() {
        assert_eq!(vec![0,1,1,0,0], epsilon(&vec![1,0,0,1,1]));
    }
    #[test]
    fn test_example() {
        let example_input = include_str!("day3_example.txt");
        assert_eq!(198, power_consumption(example_input));
    }

    #[test]
    fn test_transpose() {
        let input = vec![vec![1,2,3], vec![4,5,6]];
        let expected_output = vec![vec![1,4], vec![2,5], vec![3,6]];
        assert_eq!(expected_output, transpose(&input));
    }

    #[test]
    fn test_mean() {
        assert_eq!(1, mean(&vec![1,1,1]));
        assert_eq!(3, mean(&vec![1,3,5]));
        assert_eq!(1, mean(&vec![1,1,0,0]));
        assert_eq!(1, mean(&vec![1,1,1,0]));
    }

    #[test]
    fn part1_result() {
        let input = include_str!("day3.txt");
        println!("Result: {}", power_consumption(input));
    }
    
    #[test]
    fn test_oxygen_generator_rating() {
        let input = include_str!("day3_example.txt");
        assert_eq!(23, oxygen_generator_rating(input));
    }
    
    #[test]
    fn test_co2_scrubber_rating() {
        let input = include_str!("day3_example.txt");
        assert_eq!(10, co2_scrubber_rating(input));
    }

    #[test]
    fn test_iterator() {
        let vec = vec![vec![1, 0], vec![0, 0]];
        let mut iterator = OxygenIterator::new(vec);
        iterator.next();
        assert_eq!(vec![vec![1,0]], iterator.matrix);
        assert_eq!(Option::None, iterator.next());
    }

    #[test]
    fn test_iterator_consume() {
        let vec = vec![vec![1, 0], vec![0, 0]];
        let iterator = OxygenIterator::new(vec);
        assert_eq!(vec![vec![1, 0]], iterator.last().unwrap())
    }

    #[test]
    fn test_to_keep() {
        let vec = vec![vec![1, 0], vec![0, 0]];
        assert_eq!(1, bit_to_keep(&vec, 0));
        assert_eq!(0, bit_to_keep(&vec, 1));
    }

    #[test]
    fn test_life_support_example() {
        let input = include_str!("day3_example.txt");
        assert_eq!(230, life_support_rating(input));
    }

    #[test]
    fn part2_result() {
        let input = include_str!("day3.txt");
        println!("Part 2 result: {}", life_support_rating(input));
    }
}
