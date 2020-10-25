use std::env::args;
use std::fs;
fn main() {
    let args: Vec<String> = args().collect();
    println!("Hello, world!");
    match args.len() {
        3 => call(&args[1], &args[2]),
        _ => println!("Invalid args"),
    }
}

fn call(text: &String, file: &String) {
    println!("Searching for text: {} in file {} ", text, file);
    let filesys = fs::read_to_string(file).unwrap_or_else(|res| {
        panic!("Error {:?} occured while reading file: {} ", res, file);
    });
    for line in search(
        text.to_lowercase().as_str(),
        filesys.to_lowercase().as_str(),
    ) {
        print!("Line {}", line);
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    if results.len() == 0 {
        print!("text not found");
    } else {
        println!("Found in line with");
    }

    results
}
