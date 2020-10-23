mod db_row;
use db_row::DBTable;

fn main() {
    println!("{}, Bharat", String::from("नमस्ते"));
    let mut v = vec![100, 32, 57];
    if let Some(element) = v.get_mut(1) {
        *element = 1
    }

    for i in &mut v {
        println!("{}", i);
    }

    // let data = match v.pop() {
    //     Some(arg) => arg,
    //     None => 1,
    // };
    // println!("Enter data {}", data);

    let mut rows: Vec<DBTable> = Vec::new();
    for i in 0..5 {
        rows.push(DBTable {
            id: i,
            username: format!("i : {}", i),
            email: match i % 2 == 0 {
                true => Some(format!("{}@email.com", i)),
                _ => None,
            },
            password: String::from("password"),
        })
    }

    for item in rows.iter() {
        println!("item: {:?}", item);
    }
}
