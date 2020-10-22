use rand::Rng;
use std::cmp::Ordering;
use std::io;
fn main() {
    let secret = rand::thread_rng().gen_range(1, 101);

    println!("Hello, world! Secrit is {}", secret);
    println!("This is a guessing game!");
    loop {
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Some error occured");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret) {
            Ordering::Less => println!("Your guess is less than my secret"),
            Ordering::Equal => {
                println!("You got the right answer. Secret is {}", secret);
                break;
            }
            Ordering::Greater => println!("Your guess is more than my secret"),
        }
    }
}
