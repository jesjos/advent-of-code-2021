pub fn day4_part1(_day4_input: &str) -> u32 {
    0
}

type Board = [[u8; 5]; 5];

type DrawnNumbers = Vec<u8>;

type Input = (Vec<Board>, DrawnNumbers);

fn parse_board(input: &str) -> Board {
    let mut output: Board = [[0; 5]; 5];
    input.lines()
        .filter(|line| line.len() > 0)
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
            .filter_map(|row| row.get(col_index).map_or(None, |num| Some(num.clone()))).collect::<Vec<u8>>()
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

#[cfg(test)]
mod tests {
    use crate::day_4::{Board, has_bingo, has_column_bingo, has_row_bingo, parse_board, parse_header, parse_input};

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

        assert_eq!(false, has_column_bingo(&board, &vec![1,2]));
        assert_eq!(true, has_column_bingo(&board, &vec![1,6,11,16,21]));
    }

    #[test]
    fn test_has_bingo() {
        let board: Board = make_board();

        assert_eq!(false, has_bingo(&board, &vec![1,2,3]));
        assert_eq!(true, has_bingo(&board, &vec![6,7,8,9,10]));
        assert_eq!(true, has_bingo(&board, &vec![2,7,12,17,22]));
    }
}
