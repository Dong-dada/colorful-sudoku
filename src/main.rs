use std::collections::HashSet;
use std::io;
use std::iter::FromIterator;

fn generate_board_array() -> [[u8; 9]; 9] {
    let board_array: [[u8; 9]; 9] = [
        [5, 0, 0, 0, 8, 0, 0, 4, 9],
        [0, 0, 0, 5, 0, 0, 0, 3, 0],
        [0, 6, 7, 3, 0, 0, 0, 0, 1],
        [1, 5, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 2, 0, 8, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 1, 8],
        [7, 0, 0, 0, 0, 4, 1, 5, 0],
        [0, 3, 0, 0, 0, 2, 0, 0, 0],
        [4, 9, 0, 0, 5, 0, 0, 0, 3],
    ];

    return board_array;
}

fn print_board(board_array: &[[u8; 9]; 9]) {
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
    println!("{}", board_str);
}

fn input_number_to_board_array(board_array: &mut [[u8; 9]; 9]) {
    println!("input row: ");
    let mut row = String::new();
    io::stdin()
        .read_line(&mut row)
        .expect("Failed to read row!");
    let row: usize = row.trim().parse().expect("Please type a number!");

    println!("input column: ");
    let mut column = String::new();
    io::stdin()
        .read_line(&mut column)
        .expect("Failed to read column!");
    let column: usize = column.trim().parse().expect("Please type a number!");

    println!("input number: ");
    let mut number = String::new();
    io::stdin()
        .read_line(&mut number)
        .expect("Failed to read number!");
    let number: u8 = number.trim().parse().expect("Please type a number!");

    board_array[row][column] = number;
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

fn main() {
    let mut board_array = generate_board_array();
    print_board(&board_array);
    loop {
        input_number_to_board_array(&mut board_array);
        print_board(&board_array);
        if check_board_array_success(&board_array) {
            println!("Congratulations!");
            break;
        }
    }
}
