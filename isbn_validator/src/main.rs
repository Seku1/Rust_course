fn main() {
    println!("Hello, world!");
    let cos = String::from("3-598-21508-8");
    println!("{}",validate(&cos));
}

fn validate(isbn: &String) -> bool {
    let  mut acc = 0;
    let isbn = isbn.replace("-", "");
    println!("{}", isbn);
    let mut n = isbn.len();
    for val in isbn.chars() {
        if val == 'X'{
            acc += 10;
        }else if val.is_numeric(){
            acc += n * val as usize;
            n -= 1
        }
    }
    acc % 11 == 0 && n == 0
}