fn main() {
    println!("Hello, world!");
    let mut s = String::from("New String");
    update_string(&mut s);
    // println!("{}", s);
    let st = &s;
    // println!("s:{} str:{}", s, st);
    let data = passing_immutable(st);
    println!("data is {} \n st is {} and s is {} ", data, st, s);
}

fn update_string(string: &mut String) {
    string.push_str(" Updated");
}

fn passing_immutable(string: &String) -> &String {
    string
}
