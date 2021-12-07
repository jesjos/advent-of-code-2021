fn transpose<T>(matrix: Vec<Vec<T>>) -> Vec<Vec<T>> where T: Clone {
    if matrix.len() == 0 {
        return Vec::new()
    }
    (0..matrix[0].len())
        .map(|i| matrix.iter().map(|row| row[i].clone()).collect::<Vec<T>>())
        .collect()
}

fn mean(vec: &Vec<u32>) -> u32 {
    let total: u32 = vec.iter().sum();
    let float_mean = f64::from(total) / vec.len() as f64;
    u32::try_from(float_mean.round() as i32).unwrap()
}

fn gamma(input: &str) -> Vec<u32> {
    let matrix: Vec<Vec<u32>> = input
        .lines()
        .map(|line| line.split("").filter_map(|char| char.parse::<u32>().ok()).collect::<Vec<u32>>())
        .collect();
    let transposed = transpose(matrix);
    let means: Vec<u32> = transposed.iter().map(|column| mean(column)).collect();
    means
}

fn epsilon(input: &Vec<u32>) -> Vec<u32> {
    input.iter().map(|n| (n + 1) % 2).collect()
}

fn binary_string_to_int(input: Vec<u32>) -> u32 {
    u32::from_str_radix(&input.iter().map(|n| n.to_string()).collect::<Vec<String>>().join(""), 2).unwrap()
}

fn power_consumption(input: &str) -> u32 {
    let gamma_vec = gamma(input);
    let epsilon_vec = epsilon(&gamma_vec);
    binary_string_to_int(gamma_vec) * binary_string_to_int(epsilon_vec)
}

#[cfg(test)]
mod tests {
    use crate::day_3::{epsilon, gamma, mean, power_consumption, transpose};

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
        assert_eq!(expected_output, transpose(input));
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
}
