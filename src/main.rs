use std::collections::HashSet;
use std::iter::FromIterator;
use pancurses::Input;
use std::convert::TryFrom;

fn generate_board_array() -> [[u8; 9]; 9] {
    // let board_array: [[u8; 9]; 9] = [
    //     [5, 0, 0, 0, 8, 0, 0, 4, 9],
    //     [0, 0, 0, 5, 0, 0, 0, 3, 0],
    //     [0, 6, 7, 3, 0, 0, 0, 0, 1],
    //     [1, 5, 0, 0, 0, 0, 0, 0, 0],
    //     [0, 0, 0, 2, 0, 8, 0, 0, 0],
    //     [0, 0, 0, 0, 0, 0, 0, 1, 8],
    //     [7, 0, 0, 0, 0, 4, 1, 5, 0],
    //     [0, 3, 0, 0, 0, 2, 0, 0, 0],
    //     [4, 9, 0, 0, 5, 0, 0, 0, 3],
    // ];

    let board_array: [[u8; 9]; 9] = [
        [5, 1, 3, 6, 8, 7, 2, 4, 9],
        [8, 4, 9, 5, 2, 1, 6, 3, 7],
        [2, 6, 7, 3, 4, 9, 0, 8, 1],
        [1, 5, 8, 4, 6, 3, 9, 7, 2],
        [9, 7, 4, 2, 1, 8, 3, 6, 5],
        [3, 2, 6, 7, 9, 5, 4, 1, 8],
        [7, 8, 2, 9, 3, 4, 1, 5, 6],
        [6, 3, 5, 1, 7, 2, 8, 9, 4],
        [4, 9, 1, 8, 5, 6, 7, 2, 3],
    ];

    return board_array;
}

fn generate_board_string(board_array: &[[u8; 9]; 9]) -> String {
    let mut board_canvas: Vec<char> = "\
        ╔═══╤═══╤═══╦═══╤═══╤═══╦═══╤═══╤═══╗\n\
        ║   │   │   ║   │   │   ║   │   │   ║\n\
        ╟───┼───┼───╫───┼───┼───╫───┼───┼───╢\n\
        ║   │   │   ║   │   │   ║   │   │   ║\n\
        ╟───┼───┼───╫───┼───┼───╫───┼───┼───╢\n\
        ║   │   │   ║   │   │   ║   │   │   ║\n\
        ╠═══╪═══╪═══╬═══╪═══╪═══╬═══╪═══╪═══╣\n\
        ║   │   │   ║   │   │   ║   │   │   ║\n\
        ╟───┼───┼───╫───┼───┼───╫───┼───┼───╢\n\
        ║   │   │   ║   │   │   ║   │   │   ║\n\
        ╟───┼───┼───╫───┼───┼───╫───┼───┼───╢\n\
        ║   │   │   ║   │   │   ║   │   │   ║\n\
        ╠═══╪═══╪═══╬═══╪═══╪═══╬═══╪═══╪═══╣\n\
        ║   │   │   ║   │   │   ║   │   │   ║\n\
        ╟───┼───┼───╫───┼───┼───╫───┼───┼───╢\n\
        ║   │   │   ║   │   │   ║   │   │   ║\n\
        ╟───┼───┼───╫───┼───┼───╫───┼───┼───╢\n\
        ║   │   │   ║   │   │   ║   │   │   ║\n\
        ╚═══╧═══╧═══╩═══╧═══╧═══╩═══╧═══╧═══╝"
        .chars()
        .collect();

    for (row, numbers) in board_array.iter().enumerate() {
        for (column, number) in numbers.iter().enumerate() {
            // - rows in canvas are: [1, 3, 5, 7, 9, 11, 13, 15, 17]
            // - columns in canvas are: [2, 6, 10, 14, 18, 22, 26, 30, 34]
            let row_in_canvas = (row + 1) * 2 - 1;
            let column_in_canvas = (column + 1) * 4 - 2;
            let row_length = 38;
            let offset_in_canvas = row_in_canvas * row_length + column_in_canvas;

            if *number != 0u8 {
                board_canvas[offset_in_canvas] = char::from(*number + 48);
            } else {
                board_canvas[offset_in_canvas] = ' ';
            }
        }
    }

    let board_str: String = board_canvas.into_iter().collect();
    return board_str;
}

fn check_board_array_success(board_array: &[[u8; 9]; 9]) -> bool {
    let number_set: HashSet<u8> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9].into_iter().collect();

    for row_numbers in board_array.iter() {
        let row_number_set: HashSet<u8> = HashSet::from_iter(row_numbers.iter().cloned());
        if number_set != row_number_set {
            return false;
        }
    }

    for column in 0..board_array.len() {
        let mut row_number_set: HashSet<u8> = HashSet::new();
        for row_numbers in board_array.iter() {
            row_number_set.insert(row_numbers[column]);
        }
        if number_set != row_number_set {
            return false;
        }
    }

    for row in [0, 3, 6].iter() {
        for column in [0, 3, 6].iter() {
            let mut grid_number_set: HashSet<u8> = HashSet::new();
            grid_number_set.insert(board_array[*row][*column]);
            grid_number_set.insert(board_array[*row][*column + 1]);
            grid_number_set.insert(board_array[*row][*column + 2]);
            grid_number_set.insert(board_array[*row + 1][*column]);
            grid_number_set.insert(board_array[*row + 1][*column + 1]);
            grid_number_set.insert(board_array[*row + 1][*column + 2]);
            grid_number_set.insert(board_array[*row + 2][*column]);
            grid_number_set.insert(board_array[*row + 2][*column + 1]);
            grid_number_set.insert(board_array[*row + 2][*column + 2]);
            if number_set != grid_number_set {
                return false;
            }
        }
    }

    return true;
}

struct Position {
    x: i32,
    y: i32,
}

fn convert_board_position_to_window_position(board_position: &Position) -> Position {
    Position {
        x: (board_position.x + 1) * 4 - 2,
        y: (board_position.y + 1) * 2 - 1,
    }
}

fn char_to_u8(c: char) -> u8 {
    let char_u32 = c.to_digit(10).unwrap();
    return u8::try_from(char_u32).unwrap();
}

fn main() {
    let mut board_array = generate_board_array();
    let board_string = generate_board_string(&board_array);

    let window = pancurses::initscr();
    window.addstr(board_string);
    window.refresh();
    window.keypad(true);
    pancurses::noecho();
    let mut board_position = Position { x: 0, y: 0 };
    loop {
        let window_position = convert_board_position_to_window_position(&board_position);
        window.mv(window_position.y, window_position.x);
        match window.getch() {
            Some(Input::Character(c)) => {
                let valid_char_set: HashSet<char> = vec!['1', '2', '3', '4', '5', '6', '7', '8', '9', ' '].into_iter().collect();
                if !valid_char_set.contains(&c) {
                    continue;
                }

                let row: usize = board_position.y as usize;
                let column: usize = board_position.x as usize;
                let number: u8 = if c == ' ' {0u8} else { char_to_u8(c) };
                board_array[row][column] = number;
                window.addch(c);
                window.refresh();

                if check_board_array_success(&board_array) {
                    pancurses::napms(1000);
                    window.clear();
                    window.addstr("Congratulations!");
                    window.refresh();
                    pancurses::napms(2000);
                    break;
                }
            }
            Some(Input::KeyDC) => break,
            Some(Input::KeyLeft) => {
                if board_position.x == 0 {
                    continue;
                }

                board_position.x -= 1;
            }
            Some(Input::KeyRight) => {
                if board_position.x >= 8 {
                    continue;
                }

                board_position.x += 1;
            }
            Some(Input::KeyUp) => {
                if board_position.y <= 0 {
                    continue;
                }

                board_position.y -= 1;
            }
            Some(Input::KeyDown) => {
                if board_position.y >= 8 {
                    continue;
                }

                board_position.y += 1;
            }
            _ => {}
        }
    }
    pancurses::endwin();
}
