mod test;

use rand::{Rng};


fn main() {
    let chars: [char; 2] = [
       'a', 'B'
    ];

    println!("{}", generate_random_password(10));
    println!("{}", generate_random_password_with_chars(10, &chars));
}




fn generate_random_password(pass_len: usize) -> String {
    let password_chars: [char; 93] = [
        'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
        '0', '1', '2', '3', '4', '5', '6', '7', '8', '9',
        '!', '@', '#', '$', '%', '^', '&', '*', '(', ')', '-', '_', '=', '+', '[', ']', '{', '}', '|', ';', ':', '\'', '"', ',', '<', '.', '>', '/', '?', '`', '~'
    ];
    let n = password_chars.len();
    let mut password = String::new();
    let mut rng = rand::rng();
    for _ in 0..pass_len {
        let rand_num = rng.random_range(0..n);
        password.push(password_chars[rand_num]);
    }
    password
}



fn generate_random_password_with_chars(pass_len: usize, char_map: &[char]) -> String {
    let password_chars: [char; 93] = [
        'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
        '0', '1', '2', '3', '4', '5', '6', '7', '8', '9',
        '!', '@', '#', '$', '%', '^', '&', '*', '(', ')', '-', '_', '=', '+', '[', ']', '{', '}', '|', ';', ':', '\'', '"', ',', '<', '.', '>', '/', '?', '`', '~'
    ];
    let n = password_chars.len();
    let mut password = String::new();
    let mut rng = rand::rng();
    if char_map.len() > 0{
        for _ in 0..pass_len{
            let rand_num = rng.random_range(0..char_map.len());
            password.push(char_map[rand_num]);
        }
    }
    else{
        for _ in 0..pass_len {
            let rand_num = rng.random_range(0..n);
            password.push(password_chars[rand_num]);
        }
    }
    password
}