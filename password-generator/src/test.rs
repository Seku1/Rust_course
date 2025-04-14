#[cfg(test)]
mod test{
    use crate::{generate_random_password, generate_random_password_with_chars};

    #[test]
    fn check_length(){

        let password_chars: [char; 93] = [
            'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
            'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
            '0', '1', '2', '3', '4', '5', '6', '7', '8', '9',
            '!', '@', '#', '$', '%', '^', '&', '*', '(', ')', '-', '_', '=', '+', '[', ']', '{', '}', '|', ';', ':', '\'', '"', ',', '<', '.', '>', '/', '?', '`', '~'
        ];

        assert_eq!(generate_random_password(10).len(), 10 );
        assert_eq!(generate_random_password_with_chars(20, &password_chars).len(), 20 );
    }
}