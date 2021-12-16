use std::slice::Iter;

pub fn day4_part1(day4_input: &str) -> u32 {
    let (boards, drawn_numbers) = parse_input(day4_input);
    let maybe_bingo = run_bingo(&boards, &drawn_numbers);
    maybe_bingo.map_or(0, calculate_result)
}

fn calculate_result((board, drawn_numbers): BingoResult) -> u32 {
    let remaining_numbers = unmarked_numbers(board, &drawn_numbers);
    let last_number = drawn_numbers.last().unwrap_or(&0);
    remaining_numbers.iter().map(|num| u32::try_from(**num).unwrap_or(0)).sum::<u32>() * u32::try_from(*last_number).unwrap()
}

type Board = [[u8; 5]; 5];

type DrawnNumbers = Vec<u8>;

type Input = (Vec<Board>, DrawnNumbers);

fn parse_board(input: &str) -> Board {
    let mut output: Board = [[0; 5]; 5];
    input.lines()
        .filter(|line| !line.is_empty())
        .map(|row| row.split(' ').filter_map(|n| n.parse::<u8>().ok()))
        .enumerate()
        .for_each(|(index, row)| row.enumerate().for_each(|(col_index, cell)| output[index][col_index] = cell));
    output
}

fn parse_boards(input: &str) -> Vec<Board> {
    input.split("\n\n")
        .filter(|block| !block.is_empty())
        .map(|block| parse_board(block))
        .collect()
}

fn parse_header(input: &str) -> DrawnNumbers {
    input.split(',').filter_map(|n| n.parse::<u8>().ok()).collect()
}

fn parse_input(input: &str) -> Input {
    let mut lines = input.lines();
    let header_string = lines.next().unwrap();
    let drawn_numbers = parse_header(header_string);

    let boards_input = lines.collect::<Vec<&str>>().join("\n");

    (parse_boards(&boards_input), drawn_numbers)
}

fn has_bingo(board: &Board, drawn_numbers: &[u8]) -> bool {
    has_row_bingo(board, drawn_numbers) || has_column_bingo(board, drawn_numbers)
}

fn has_column_bingo(board: &Board, drawn_numbers: &[u8]) -> bool {
    let bingo_column = (0..5)
        .map(|col_index: usize| board.iter()
            .filter_map(|row| row.get(col_index).copied()).collect::<Vec<u8>>()
        ).find(|column| contains_all(&column[..], drawn_numbers));
    bingo_column.is_some()
}

fn has_row_bingo(board: &Board, drawn_numbers: &[u8]) -> bool {
    let bingo_row = board.iter().find(|row| contains_all(&row[..], drawn_numbers));
    bingo_row.is_some()
}

fn contains_all(one: &[u8], other: &[u8]) -> bool {
    one.iter().all(|n| other.contains(n))
}

struct SublistIterator<'a> {
    output: Vec<u8>,
    iter: Iter<'a, u8>
}

impl SublistIterator<'_> {
    fn new(origin: &[u8]) -> SublistIterator {
        SublistIterator {
            output: Vec::new(),
            iter: origin.iter()
        }
    }

    fn drawn_numbers(self) -> Vec<u8> {
        self.output
    }
}

impl Iterator for SublistIterator<'_> {
    type Item = Vec<u8>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(next) = self.iter.next() {
            self.output.push(*next);
            Some(self.output.clone())
        } else {
            None
        }
    }
}

type BingoResult<'a> = (&'a Board, Vec<u8>);

fn run_bingo<'a>(boards: &'a [Board], drawn_numbers: &'_[u8]) -> Option<BingoResult<'a>> {
    let mut iter = SublistIterator::new(drawn_numbers);
    iter
        .find_map(|drawn_numbers| boards.iter()
            .find(|board| has_bingo(board, &drawn_numbers[..]))
        ).map(|bingo_board| (bingo_board, iter.drawn_numbers()))
}

fn unmarked_numbers<'a>(board: &'a Board, drawn_numbers: &'_[u8]) -> Vec<&'a u8> {
    board.iter()
        .flatten()
        .filter(|num| !drawn_numbers.contains(num))
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::day4_part1;
    use crate::day_4::{Board, DrawnNumbers, has_bingo, has_column_bingo, has_row_bingo, parse_board, parse_header, parse_input, run_bingo, SublistIterator, unmarked_numbers};

    #[test]
    fn test_parse_board() {
        let input = "1 2 3 4 5\n\n6 7 8 9 10\n\n11 12 13 14 15\n\n16 17 18 19 20\n\n21 22 23 24 25";
        let expected_board: Board = make_board();
        assert_eq!(expected_board, parse_board(input));
    }

    fn make_board() -> Board {
        [[1, 2, 3, 4, 5], [6, 7, 8, 9, 10], [11, 12, 13, 14, 15], [16, 17, 18, 19, 20], [21, 22, 23, 24, 25]]
    }

    #[test]
    fn test_parse_header() {
        let input = "1,2,3,4,5";
        assert_eq!(vec![1,2,3,4,5], parse_header(input));
    }

    #[test]
    fn test_parse_input() {
        let input = include_str!("day4_example.txt");

        let expected_drawn_numbers: Vec<u8> = vec![7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1];

        let expected_board_1 = [
            [22, 13, 17, 11, 0],
            [8, 2, 23, 4, 24],
            [21, 9, 14, 16, 7],
            [6, 10, 3, 18, 5],
            [1, 12, 20, 15, 19]
        ];

        let expected_board_2 = [
            [3, 15, 0, 2, 22],
            [9, 18, 13, 17, 5],
            [19, 8, 7, 25, 23],
            [20, 11, 10, 24, 4],
            [14, 21, 16, 12, 6]
        ];

        let expected_board_3 = [
            [14, 21, 17, 24, 4],
            [10, 16, 15, 9, 19],
            [18, 8, 23, 26, 20],
            [22, 11, 13, 6, 5],
            [2, 0, 12, 3, 7]
        ];

        assert_eq!((vec![expected_board_1, expected_board_2, expected_board_3], expected_drawn_numbers), parse_input(input));
    }

    #[test]
    fn test_has_row_bingo() {
        let board: Board = make_board();

        assert_eq!(false, has_row_bingo(&board, &vec![1,2]));
        assert_eq!(true, has_row_bingo(&board, &vec![6,7,8,9, 10]));
    }

    #[test]
    fn test_has_column_bingo() {
        let board: Board = make_board();

        assert!(!has_column_bingo(&board, &vec![1,2]));
        assert!(has_column_bingo(&board, &vec![1,6,11,16,21]));
    }

    #[test]
    fn test_has_bingo() {
        let board: Board = make_board();

        assert!(!has_bingo(&board, &vec![1,2,3]));
        assert!(has_bingo(&board, &vec![6,7,8,9,10]));
        assert!(has_bingo(&board, &vec![2,7,12,17,22]));
    }

    #[test]
    fn test_sublist_iterator() {
        let list: [u8; 5] = [1,2,3,4,5];

        let mut iter = SublistIterator::new(&list);

        assert_eq!(vec![1], iter.next().unwrap());
        assert_eq!(vec![1,2], iter.next().unwrap());
        assert_eq!(vec![1,2,3], iter.next().unwrap());
        assert_eq!(vec![1,2,3,4], iter.next().unwrap());
        assert_eq!(vec![1,2,3,4,5], iter.next().unwrap());
        assert_eq!(None, iter.next());
    }

    #[test]
    fn test_run_bingo() {
        let bingo_board = make_board();
        let no_bingo_board = make_no_bingo_board();

        let boards = vec![bingo_board, no_bingo_board];
        let should_be_empty = run_bingo(&boards, &vec![1]);
        assert!(should_be_empty.is_none());

        let should_be_bingo = run_bingo(&boards, &vec![1,2,3,4,5,6]);

        assert!(should_be_bingo.is_some());
        let (board, drawn_numbers) = should_be_bingo.unwrap();
        assert_eq!(bingo_board, *board);
        assert_eq!(vec![1,2,3,4,5], drawn_numbers)

    }

    #[test]
    fn test_unmarked() {
        let board = make_board();

        let marked_numbers: DrawnNumbers = (1..=23).collect();
        let unmarked = unmarked_numbers(&board, &marked_numbers);

        assert_eq!(vec![24,25], unmarked.iter().map(|num| *num.clone()).collect::<Vec<u8>>());
    }

    #[test]
    fn test_example() {
        assert_eq!(4512, day4_part1(include_str!("day4_example.txt")));
    }

    fn make_no_bingo_board() -> Board {
        [[99; 5]; 5]
    }
}
