pub fn hash_map() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(20);
    scores.insert(String::from("Blues"), 50);

    println!("{:?}", scores);

    let text = " hello world world wonderful world wonderful";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        // count is an mutable reference to the value of word
        *count += 1;
    }

    println!("{:?}", map);
}
