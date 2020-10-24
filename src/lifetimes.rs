// static is predefined i.e., it will live as long as the program lives
pub fn largest(a: &'static str, b: &'static str) -> &'static str {
    if a.len() > b.len() {
        a
    } else {
        b
    }
}
pub fn largest_another<'my_another, 'your_another>(
    a: &'my_another str,
    b: &'my_another str,
) -> &'my_another str {
    if a.len() > b.len() {
        a
    } else {
        b
    }
}
//with one parameter you don't need to define the lifetimes
pub fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
