use std::ops::Add;

fn generate_board_str(board_array: [[u8; 9]; 9]) -> String {
    let mut board_str = String::new();
    board_str = board_str.add("╔═══╤═══╤═══╦═══╤═══╤═══╦═══╤═══╤═══╗");
    board_str = board_str.add("\n");
    for (row_index, row_numbers) in board_array.iter().enumerate() {
        board_str = board_str.add("║");
        for (column_index, number) in row_numbers.iter().enumerate() {
            let number_str = if *number == 0u8 { " ".to_string() } else { number.to_string() };
            if (column_index + 1) % 3 == 0 {
                board_str = board_str.add(&format!(" {} ║", number_str));
            } else {
                board_str = board_str.add(&format!(" {} │", number_str));
            }
        }
        board_str = board_str.add("\n");
        if row_index == 8 {
            board_str = board_str.add("╚═══╧═══╧═══╩═══╧═══╧═══╩═══╧═══╧═══╝");
        } else if (row_index + 1) % 3 == 0 {
            board_str = board_str.add("╠═══╪═══╪═══╬═══╪═══╪═══╬═══╪═══╪═══╣");
            board_str = board_str.add("\n");
        } else {
            board_str = board_str.add("╟───┼───┼───╫───┼───┼───╫───┼───┼───╢");
            board_str = board_str.add("\n");
        }
    }

    return board_str;
}

fn main() {
    let board_array = [[0u8; 9]; 9];

    let board_str = generate_board_str(board_array);
    println!("{}", board_str);
}
