mod models;

use std::collections::HashSet;
use pancurses::Input;
use std::convert::TryFrom;
use crate::models::board::Board;

fn generate_board_string(board: &Board) -> String {
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

    for (row, row_slots) in board.get_slots().iter().enumerate() {
        for (column, slot) in row_slots.iter().enumerate() {
            // - rows in canvas are: [1, 3, 5, 7, 9, 11, 13, 15, 17]
            // - columns in canvas are: [2, 6, 10, 14, 18, 22, 26, 30, 34]
            let row_in_canvas = (row + 1) * 2 - 1;
            let column_in_canvas = (column + 1) * 4 - 2;
            let row_length = 38;
            let offset_in_canvas = row_in_canvas * row_length + column_in_canvas;

            if slot.number != 0u8 {
                board_canvas[offset_in_canvas] = char::from(slot.number + 48);
            } else {
                board_canvas[offset_in_canvas] = ' ';
            }
        }
    }

    let board_str: String = board_canvas.into_iter().collect();
    return board_str;
}

struct WindowPosition {
    x: i32,
    y: i32,
}

fn convert_board_position_to_window_position(board: &Board) -> WindowPosition {
    WindowPosition {
        x: ((board.get_pos().column + 1) * 4 - 2) as i32,
        y: ((board.get_pos().row + 1) * 2 - 1) as i32,
    }
}

fn char_to_u8(c: char) -> u8 {
    let char_u32 = c.to_digit(10).unwrap();
    return u8::try_from(char_u32).unwrap();
}

fn main() {
    let mut board = Board::generate();
    let board_string = generate_board_string(&board);

    let window = pancurses::initscr();
    window.addstr(board_string);
    window.refresh();
    window.keypad(true);
    pancurses::noecho();
    loop {
        let window_position = convert_board_position_to_window_position(&board);
        window.mv(window_position.y, window_position.x);
        match window.getch() {
            Some(Input::Character(c)) => {
                let valid_char_set: HashSet<char> = vec!['1', '2', '3', '4', '5', '6', '7', '8', '9', ' '].into_iter().collect();
                if !valid_char_set.contains(&c) {
                    continue;
                }

                let number: u8 = if c == ' ' {0u8} else { char_to_u8(c) };
                board.set_number(number).unwrap();
                window.addch(c);
                window.refresh();

                if board.check_success() {
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
                board.move_left();
            }
            Some(Input::KeyRight) => {
                board.move_right();
            }
            Some(Input::KeyUp) => {
                board.move_up();
            }
            Some(Input::KeyDown) => {
                board.move_down();
            }
            _ => {}
        }
    }
    pancurses::endwin();
}
