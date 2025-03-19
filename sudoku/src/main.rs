mod test;

fn main() {
    let board = [
        [5, 3, 0, 0, 7, 0, 0, 0, 0],
        [6, 0, 0, 1, 9, 5, 0, 0, 0],
        [0, 9, 8, 0, 0, 0, 0, 6, 0],
        [8, 0, 0, 0, 6, 0, 0, 0, 3],
        [4, 0, 0, 8, 0, 3, 0, 0, 1],
        [7, 0, 0, 0, 2, 0, 0, 0, 6],
        [0, 6, 0, 0, 0, 0, 2, 8, 0],
        [0, 0, 0, 4, 1, 9, 0, 0, 5],
        [0, 0, 0, 0, 8, 0, 0, 7, 9]
    ];

    println!("{}", validator(&board));

}

fn check_lines(board: &[[i32; 9]; 9]) -> bool {
    for i in 0..9{
        let mut row = [true; 9];
        let mut col = [true; 9];
        for j in 0..9{
            if 0 <= board[i][j] && board[i][j] < 10 && 0 <= board[j][i] && board[j][i] < 10{
                if board[i][j] != 0 {
                    if row[(board[i][j] - 1) as usize] {
                        row[(board[i][j] - 1) as usize] = false;
                    }else {
                        println!("{}", board[i][j]);
                        return false
                    }
                }
                if board[j][i] != 0 {
                    if col[(board[j][i] - 1) as usize]{
                        col[(board[j][i] - 1) as usize] = false;
                    }else{
                        println!("{}", board[j][i]);
                        return false
                    }
                }
            } else {
                println!("number {} is out of bounds", if 0 <= board[i][j] || board[i][j] < 10 {board[i][j]} else {board[j][i]} );
                return false;
            }
        }
    }
    true
}

fn check_squares(board: &[[i32; 9]; 9]) -> bool {
    for i in 0..9{
        let mut sqr =[true; 9];
        for j in 0..9{
            let row : usize = (j / 3  + 3 * (i / 3)) as usize;
            let col : usize = ((j) % 3 + 3 * (i % 3)) as usize;

            if board[row][col] != 0{
                if sqr[(board[row][col] - 1) as usize]{
                    sqr[(board[row][col] - 1) as usize] = false;
                }else { return false; }

            }
        }
    }
    true
}


fn validator(&board: &[[i32; 9]; 9]) -> bool {
    check_lines(&board) && check_squares(&board)
}