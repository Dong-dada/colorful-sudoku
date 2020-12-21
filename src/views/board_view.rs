use crate::models::board::Board;

pub struct BoardView<'a> {
    board: &'a Board,
}

struct WindowPosition {
    x: i32,
    y: i32,
}

impl BoardView<'_> {
    pub fn new(board: &Board) -> BoardView {
        BoardView {
            board
        }
    }

    pub fn generate_string(&self) -> String {
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

        for (row, row_slots) in self.board.get_slots().iter().enumerate() {
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

    fn convert_to_window_position(row: usize, column: usize) -> WindowPosition {
        WindowPosition {
            x: ((column + 1) * 4 - 2) as i32,
            y: ((row + 1) * 2 - 1) as i32,
        }
    }

    fn number_to_char(number: u8) -> char {
        if number == 0 {
            return ' ';
        }

        return char::from(number + 48);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_string() {
        let mut board = Board::generate_empty();
        board.set_number(1).unwrap();
        board.set_pos(1, 3);
        board.set_number(3).unwrap();
        let window = pancurses::initscr();
        let board_view = BoardView { board: &board };
        let board_string = board_view.generate_string();
        assert_eq!(board_string, "\
        ╔═══╤═══╤═══╦═══╤═══╤═══╦═══╤═══╤═══╗\n\
        ║ 1 │   │   ║   │   │   ║   │   │   ║\n\
        ╟───┼───┼───╫───┼───┼───╫───┼───┼───╢\n\
        ║   │   │   ║ 3 │   │   ║   │   │   ║\n\
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
        ╚═══╧═══╧═══╩═══╧═══╧═══╩═══╧═══╧═══╝");
    }
}

