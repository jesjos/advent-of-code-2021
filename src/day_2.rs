use std::str::FromStr;
use crate::day_2::Instruction::{Down, Forward, Up};

fn travel(instructions: &str) -> i32 {
    instructions
        .lines()
        .map(perform_part1)
        .fold(State::new(), |state, delta| state.apply(delta)).value()
}

fn travel_part2(instructions: &str) -> i32 {
    instructions
        .lines()
        .map(Instruction::from_str)
        .filter_map(Result::ok)
        .fold(State::new(), |state, delta| state.apply_part2(delta)).value()
}

struct State {
    depth: i32,
    forward: i32,
    aim: i32
}

impl State {
    fn new() -> State {
        State { depth: 0, forward: 0, aim: 0}
    }

    fn apply(mut self, delta: Delta) -> State {
        self.depth += delta.delta_depth;
        self.forward += delta.delta_horizontal;
        self
    }

    fn apply_part2(mut self, instruction: Instruction) -> State {
        match instruction {
            Forward(steps) => {
                let signed_steps = i32::try_from(steps).unwrap();
                self.forward += signed_steps;
                self.depth += signed_steps * self.aim;
            }
            Down(steps) => self.aim += i32::try_from(steps).unwrap(),
            Up(steps) => self.aim -= i32::try_from(steps).unwrap()
        }
        self
    }

    fn value(&self) -> i32 {
        self.depth * self.forward
    }
}

struct Delta {
    pub delta_depth: i32,
    pub delta_horizontal: i32
}

fn perform_part1(instruction: &str) -> Delta {
    let parsed: Instruction = Instruction::from_str(instruction).unwrap();
    fn to_signed(unsigned: u32) -> i32 {
        i32::try_from(unsigned).unwrap()
    }
    match parsed {
        Forward(steps) => Delta { delta_depth: 0, delta_horizontal: to_signed(steps)},
        Down(steps) => Delta { delta_depth: to_signed(steps), delta_horizontal: 0},
        Up(steps) => Delta { delta_depth: -to_signed(steps), delta_horizontal: 0}
    }
}

#[derive(PartialEq)]
#[derive(Debug)]
enum Instruction {
    Forward(u32),
    Down(u32),
    Up(u32)
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(" ");
        let action = split.next().unwrap();
        let value = split.next().unwrap();
        match action {
            "forward" => Ok(Forward(u32::from_str(value).unwrap())),
            "up" => Ok(Up(u32::from_str(value).unwrap())),
            "down" => Ok(Down(u32::from_str(value).unwrap())),
            _ => Err(String::from("Incorrect action"))
        }
    }
}


#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use crate::day_2::Instruction::{Forward, Up};
    use crate::day_2::{Instruction, travel, travel_part2};

    #[test]
    fn test_example() {
        let example_input = include_str!("day2_example.txt");
        assert_eq!(150, travel(example_input));
    }

    #[test]
    fn test_parse_forward() {
        assert_eq!(Forward(1), Instruction::from_str("forward 1").unwrap());
    }

    #[test]
    fn test_parse_up() {
        assert_eq!(Up(2), Instruction::from_str("up 2").unwrap());
    }

    #[test]
    fn test_answer() {
        let input = include_str!("day2.txt");
        println!("Day 2 {}", travel(input));
    }

    #[test]
    fn example_with_aim() {
        let example_input = include_str!("day2_example.txt");
        assert_eq!(900, travel_part2(example_input));
    }

    #[test]
    fn test_with_aim() {
        let input = include_str!("day2.txt");
        println!("Day2 part 2: {}", travel_part2(input));
    }
}
