use std::collections::HashSet;

pub struct Slot {
    pub number: u8,
    pub modifiable: bool,
}

impl Default for Slot {
    fn default() -> Slot {
        Slot {
            number: 0,
            modifiable: true,
        }
    }
}

pub struct Position {
    pub row: usize,
    pub column: usize,
}

pub struct Board {
    slots: [[Slot; 9]; 9],
    current_pos: Position,
}

#[derive(PartialEq, Debug)]
pub enum Error {
    // 试图修改一个 modifiable = false 的 slot 时，返回此错误
    SlotCannotModify,
}

impl Board {
    pub fn generate_empty() -> Board {
        return Board {
            slots: Default::default(),
            current_pos: Position { column: 0, row: 0 },
        };
    }

    pub fn generate() -> Board {
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

        let mut board = Board {
            slots: Default::default(),
            current_pos: Position { column: 0, row: 0 },
        };
        for (row, row_slots) in board.slots.iter_mut().enumerate() {
            for (column, slot) in row_slots.iter_mut().enumerate() {
                let number = board_array[row][column];
                if number == 0 {
                    continue;
                }

                slot.number = number;
                slot.modifiable = false;
            }
        }
        return board;
    }

    pub fn get_pos(&self) -> &Position {
        return &self.current_pos;
    }

    pub fn set_pos(&mut self, row: usize, column: usize) {
        if row >= 9 || column >= 9 {
            return;
        }

        self.current_pos.row = row;
        self.current_pos.column = column;
    }

    pub fn move_left(&mut self) {
        if self.current_pos.column <= 0 {
            return;
        }

        self.current_pos.column -= 1;
    }

    pub fn move_right(&mut self) {
        if self.current_pos.column >= 8 {
            return;
        }

        self.current_pos.column += 1;
    }

    pub fn move_up(&mut self) {
        if self.current_pos.row <= 0 {
            return;
        }

        self.current_pos.row -= 1;
    }

    pub fn move_down(&mut self) {
        if self.current_pos.row >= 8 {
            return;
        }

        self.current_pos.row += 1;
    }

    pub fn set_number(&mut self, number: u8) -> Result<(), Error> {
        if !self.slots[self.current_pos.row][self.current_pos.column].modifiable {
            return Result::Err(Error::SlotCannotModify);
        }

        self.slots[self.current_pos.row][self.current_pos.column].number = number;
        return Result::Ok(());
    }

    pub fn get_number(&self) -> Option<u8> {
        let number = self.slots[self.current_pos.row][self.current_pos.column].number;
        if number == 0 {
            return Option::None;
        }

        return Option::Some(number);
    }

    pub fn get_slots(&self) -> &[[Slot; 9]; 9] {
        return &self.slots;
    }

    pub fn check_success(&self) -> bool {
        let number_set: HashSet<u8> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9].into_iter().collect();

        for row_numbers in self.slots.iter() {
            let row_number_set: HashSet<u8> = row_numbers.iter().map(|slot| slot.number).collect();
            if number_set != row_number_set {
                return false;
            }
        }

        for column in 0..self.slots.len() {
            let mut row_number_set: HashSet<u8> = HashSet::new();
            for row_numbers in self.slots.iter() {
                row_number_set.insert(row_numbers[column].number);
            }
            if number_set != row_number_set {
                return false;
            }
        }

        for row in [0, 3, 6].iter() {
            for column in [0, 3, 6].iter() {
                let mut grid_number_set: HashSet<u8> = HashSet::new();
                grid_number_set.insert(self.slots[*row][*column].number);
                grid_number_set.insert(self.slots[*row][*column + 1].number);
                grid_number_set.insert(self.slots[*row][*column + 2].number);
                grid_number_set.insert(self.slots[*row + 1][*column].number);
                grid_number_set.insert(self.slots[*row + 1][*column + 1].number);
                grid_number_set.insert(self.slots[*row + 1][*column + 2].number);
                grid_number_set.insert(self.slots[*row + 2][*column].number);
                grid_number_set.insert(self.slots[*row + 2][*column + 1].number);
                grid_number_set.insert(self.slots[*row + 2][*column + 2].number);
                if number_set != grid_number_set {
                    return false;
                }
            }
        }

        return true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_empty_board() {
        let board = Board::generate_empty();
        assert_eq!(board.current_pos.column, 0);
        assert_eq!(board.current_pos.row, 0);
        for row_slots in board.slots.iter() {
            for slot in row_slots.iter() {
                assert_eq!(slot.number, 0);
                assert_eq!(slot.modifiable, true);
            }
        }
    }

    #[test]
    fn generate_board() {
        let board = Board::generate();
        assert_eq!(board.current_pos.column, 0);
        assert_eq!(board.current_pos.row, 0);
    }

    #[test]
    fn get_pos() {
        let board = Board::generate();
        let pos = board.get_pos();
        assert_eq!(pos.column, 0);
        assert_eq!(pos.row, 0);
    }

    #[test]
    fn set_pos() {
        let mut board = Board::generate();
        board.set_pos(4, 1);
        let pos = board.get_pos();
        assert_eq!(pos.row, 4);
        assert_eq!(pos.column, 1);

        board.set_pos(9, 0);
        let pos = board.get_pos();
        assert_eq!(pos.row, 4);
        assert_eq!(pos.column, 1);

        board.set_pos(0, 9);
        let pos = board.get_pos();
        assert_eq!(pos.row, 4);
        assert_eq!(pos.column, 1);
    }

    #[test]
    fn move_left() {
        let mut board = Board::generate();
        board.set_pos(0, 1);
        board.move_left();
        let pos = board.get_pos();
        assert_eq!(pos.row, 0);
        assert_eq!(pos.column, 0);

        board.move_left();
        let pos = board.get_pos();
        assert_eq!(pos.row, 0);
        assert_eq!(pos.column, 0);
    }

    #[test]
    fn move_right() {
        let mut board = Board::generate();
        board.set_pos(0, 7);
        board.move_right();
        let pos = board.get_pos();
        assert_eq!(pos.row, 0);
        assert_eq!(pos.column, 8);

        board.move_right();
        let pos = board.get_pos();
        assert_eq!(pos.row, 0);
        assert_eq!(pos.column, 8);
    }

    #[test]
    fn move_up() {
        let mut board = Board::generate();
        board.set_pos(1, 0);
        board.move_up();
        let pos = board.get_pos();
        assert_eq!(pos.row, 0);
        assert_eq!(pos.column, 0);

        board.move_up();
        let pos = board.get_pos();
        assert_eq!(pos.row, 0);
        assert_eq!(pos.column, 0);
    }

    #[test]
    fn move_down() {
        let mut board = Board::generate();
        board.set_pos(7, 0);
        board.move_down();
        let pos = board.get_pos();
        assert_eq!(pos.row, 8);
        assert_eq!(pos.column, 0);

        board.move_down();
        let pos = board.get_pos();
        assert_eq!(pos.row, 8);
        assert_eq!(pos.column, 0);
    }

    #[test]
    fn set_number() {
        let mut board = Board::generate();
        let result = board.set_number(3);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), Error::SlotCannotModify);

        board.set_pos(2, 6);
        let result = board.set_number(5);
        assert!(result.is_ok());
        assert_eq!(board.get_number().unwrap(), 5);
    }

    #[test]
    fn check_success() {
        let mut board = Board::generate();
        let success = board.check_success();
        assert_eq!(success, false);

        board.set_pos(2, 6);
        board.set_number(5).unwrap();
        let success = board.check_success();
        assert!(success);
    }
}