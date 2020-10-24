mod generics;

fn main() {
    let vec = vec![1, 2, 5, 4, 3];
    let vec_char = vec!['a', 'e', 'u', 'i', 'o'];

    println!(
        "Hello, world! \nLargest in {:?} is {:?}",
        vec,
        generics::largest(&vec)
    );
    println!(
        "Hello, world! \nLargest in {:?} is {:?}",
        vec_char,
        generics::largest(&vec_char)
    );
}
