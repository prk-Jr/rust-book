mod generics;
mod lifetimes;
mod traits;
use lifetimes::{first_word, largest, largest_another};
use traits::{MyNumber, Numbers};

fn main() {
    let vec = vec![1, 2, 5, 4, 3];
    let vec_char = vec!['a', 'e', 'u', 'i', 'o'];

    println!(
        "Hello, world! \nLargest in {:#?} is {:?}",
        vec,
        generics::largest(&vec)
    );
    println!(
        "Hello, world! \nLargest in {:#?} is {:?}",
        vec_char,
        generics::largest(&vec_char)
    );

    // let mut num = Numbers::default();
    let mut num = Numbers::new(10, 5);

    println!("Before swap num  {:#?}", num);
    num.swap();
    println!("After swap num {:#?}", num);

    println!("Longer str is {}", largest("Hello India", "हेलो भारत"));
    println!(
        "Longest another str is {}",
        largest_another("Hello India", "हेलो")
    );
    println!(
        "Longest another str is {}",
        largest_another("Hello India", "हे राम")
    );
    println!("first_world str is {}", first_word("राम",));
}
