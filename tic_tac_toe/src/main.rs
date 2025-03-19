use std::io;

fn main() {
    let mut board = [[" "; 3]; 3];
    let mut turn = 0;

    while !check_win(&board) && turn < 10 {
        turn += 1;
        let mut valid = false;

        while ! valid{

            let mut user_input = String::new();
            if turn % 2 == 0 {
                println!("Please enter your turn (X): ");
            }else {
                println!("Please enter your turn (O): ");
            }

            let _ = io::stdin().read_line(&mut user_input);
            let cmd = user_input.trim().parse::<i32>().unwrap();

            if cmd < 9{
                let row : usize = (cmd / 3) as usize;
                let col : usize = (cmd % 3) as usize;
                if board[row][col] == "O" || board[row][col] == "X" {
                    println!("invalid move");
                }else{
                    board[row][col] = if turn % 2 == 0 { "X" } else { "O" };
                    valid = true;
                }
            }
        }
    }
}

fn check_win(board: &[[&str; 3]; 3]) -> bool {
    print_board(&board);
    if board[1][1] == "O" || board[1][1] == "X" {
        if (board[1][1] == board[0][0] && board[1][1] == board[2][2])
            || (board[1][1] == board[2][0] && board[1][1] == board[0][2]) {
            println!("Player {} wins!", board[1][1]);
            return true;
        }
    }
    for i in 0..3{
        if board[i][0] == board[i][1] && board[i][0] == board[i][2]
            && (board[i][0] == "O" || board[i][0] == "X") {
            println!("Player {} wins!", board[i][0]);
            return true;
        }
        if board[0][i] == board[1][i] && board[1][i] == board[2][i]
            && (board[0][i] == "O" || board[0][i] == "X"){
            println!("{} c", board[0][i]);
            println!("Player {} wins!", board[0][i]);
            return true;
        }
    }
    false
}

fn print_board(board: &[[&str; 3]; 3]) {
    println!("Board:");
    for i in 0..3{
        print!("| ");
        for j in 0..3{
            print!("{} | ", board[i][j]);
        }
        println!("\n-------------");
    }
}