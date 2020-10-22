use std::io;
fn main() {
    println!("Hello, world!");
    println!("This is a guessing game!");
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Some error occured");

    print!("Number guessed is {}", guess);
}
