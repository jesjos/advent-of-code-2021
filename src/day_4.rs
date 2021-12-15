use crate::day_4::BingoResult::NoBingo;

#[derive(PartialEq, Debug)]
struct Board {
    matrix: Vec<Vec<Cell>>
}

impl Board {
    fn try_from(matrix: Vec<Vec<u8>>) -> Result<Board, &'static str> {
        if matrix.len() == 5 && matrix.iter().all(|row| row.len() == 5) {
            let cell_matrix = matrix.iter().map(|row| row.iter().map(to_cell).collect()).collect();
            Ok(Board { matrix: cell_matrix })
        } else {
            Err("Malformed board")
        }
    }

    fn has_bingo(&self) -> bool {
        self.has_row_bingo() || self.has_column_bingo()
    }

    fn mark<'a>(self: Board, number: u8) -> BingoResult<'a> {
        self.matrix.iter().for_each(|row| row.iter().for_each(|mut cell| cell.maybe_mark(number)));
        if self.has_bingo() {
            BingoResult::Bingo(&self)
        } else {
            BingoResult::NoBingo
        }
    }

    fn has_row_bingo(&self) -> bool {
        let bingo_row = self.matrix.iter().find(|row| row.iter().all(|cell| cell.marked));
        bingo_row.is_some()
    }

    fn has_column_bingo(&self) -> bool {
        let columns: Vec<Vec<&Cell>> = (0..5).map(|column| self.matrix.iter().map(|row| row.get(column).unwrap()).collect()).collect();
        columns.iter().find(|column| column.iter().all(|cell| cell.marked)).is_some()
    }
}

#[derive(PartialEq)]
enum BingoResult<'a> {
    Bingo(&'a Board),
    NoBingo
}

struct Bingo {
    drawn_numbers: Vec<u8>,
    boards: Vec<Board>
}

impl Bingo {
    fn mark<'a>(self: Bingo, number: u8) -> BingoResult<'a> {
        let bingo_board = self.boards.iter().find(|board| match board.mark(number) {
            BingoResult::Bingo(bord) => true,
            NoBingo => false
        });
        bingo_board.map_or(BingoResult::NoBingo, |board| BingoResult::Bingo(board))

    }
}


#[derive(PartialEq, Debug)]
struct Cell {
    value: u8,
    marked: bool
}

impl Cell {
    fn maybe_mark(mut self: Cell, number: u8) {
        if self.value == number {
            self.marked = true
        }
    }
}

fn to_cell(value: &u8) -> Cell {
    Cell { value: value.clone(), marked: false}
}

pub(crate) fn day4_part1(input: &str) -> u32 {
    0
}

fn run_bingo(bingo: &Bingo) -> BingoResult {
    let bingo_board = bingo.drawn_numbers.iter().map(|number| bingo.mark(*number)).find(|result| matches!(result, BingoResult::Bingo(_)));
    bingo_board.unwrap()
}

fn all_marked(row: &[Cell]) -> bool {
    row.iter().all(|cell| cell.marked)
}

// fn has_bingo(drawn_numbers: &[u8], board: &Board) -> bool {
//     for drawn_number in drawn_numbers {
//         mark(drawn_number, board)
//     }
//     true
// }

fn parse_drawn_numbers(input: &str) -> Vec<u8> {
    let first_line = input.lines().next().unwrap();
    first_line.split(',')
        .filter_map(|n| n.parse::<u8>().ok())
        .collect()
}

fn to_board(input: &str) -> Board {
    let matrix = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|lines| lines.split(' ').filter_map(|number| number.parse::<u8>().ok()).collect())
        .collect();
    Board::try_from(matrix).unwrap()
}

fn parse_boards(input_without_header: &str) -> Vec<Board> {
    let blocks = split_on_empty_line(input_without_header);
    blocks.iter().map(|string|to_board(*string)).collect()
}

fn split_on_empty_line(input: &str) -> Vec<&str> {
    input.split("\n\n").collect()
}

fn parse_input(input: &str) -> (Vec<u8>, Vec<Board>) {
    let drawn_numbers = parse_drawn_numbers(input);
    let input_without_headers = input.lines().skip(1).collect::<Vec<&str>>().join("\n");
    let boards = parse_boards(&input_without_headers);
    (drawn_numbers, boards)
}

#[cfg(test)]
mod tests {
    use crate::day4_part1;
    use crate::day_4::{Board, Cell, parse_boards, parse_drawn_numbers, parse_input, all_marked, split_on_empty_line, to_board, to_cell, Bingo, run_bingo, BingoResult};

    #[test]
    fn part_1_result() {
        let input = include_str!("day4_example.txt");
        println!("Day 4 part 1 result: {}", day4_part1(input));
    }

    #[test]
    fn test_parse_drawn_numbers() {
        assert_eq!(vec![7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1],
                   parse_drawn_numbers(include_str!("day4_example.txt")));
    }

    #[test]
    fn test_split_on_empty_line() {
        let string = "a\n\
        b\n\nc\n";
        assert_eq!(vec!["a\nb", "c\n"], split_on_empty_line(string));
    }

    #[test]
    fn test_parse_one_board() {
        let board = "1 2 3 4 5\n6 7 8 9 10\n11 12 13 14 15\n16 17 18 19 20\n21 22 23 24 25";

        let expected = make_board();

        assert_eq!(expected, to_board(board));
    }

    fn make_board() -> Board {
        fn try_u8(n: i32) -> Cell {
            to_cell(&u8::try_from(n).unwrap())
        }
        Board {
            matrix: vec![(1..=5).map(try_u8).collect(),
                         (6..=10).map(try_u8).collect(),
                         (11..=15).map(try_u8).collect(),
                         (16..=20).map(try_u8).collect(),
                         (21..=25).map(try_u8).collect()]
        }
    }

    #[test]
    fn test_parse_input() {
        let one_board = "1 2 3 4 5\n6 7 8 9 10\n11 12 13 14 15\n16 17 18 19 20\n21 22 23 24 25";
        let boards = format!("{}\n\n{}", one_board, one_board);

        let expected_board = make_board();
        let other_expected_board = make_board();

        assert_eq!(vec![ expected_board, other_expected_board ], parse_boards(&boards));
    }

    #[test]
    fn test_parse_example() {
        let (drawn_numbers, parsed_boards) = parse_input(include_str!("day4_example.txt"));

        assert_eq!(27, drawn_numbers.len());
        assert_eq!(3, parsed_boards.len());
    }

    #[test]
    fn test_row_has_bingo() {
        let row: Vec<Cell>= (1..=5).map(|n| Cell { value: n, marked: true}).collect();
        assert_eq!(true, all_marked(&row));
    }

    #[test]
    fn test_run_bingo() {
        let board = make_board();
        let bingo = Bingo { drawn_numbers: vec![1, 5, 3, 4, 2], boards: vec![board]};
        if let BingoResult::Bingo(bingo_board) = run_bingo(&bingo) {
            assert_eq!(bingo_board, &board);
        }
    }
}
