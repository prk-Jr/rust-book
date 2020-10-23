mod ip;
use ip::{IpAddr, IpAddrV4, IpAddrV6};

fn main() {
    println!("Hello, world!");
    let myipv4 = IpAddr::V4(IpAddrV4 {
        host: String::from("127.0.0.1"),
        port: String::from("8000"),
    });
    let myipv6 = IpAddr::V6(IpAddrV6 {
        host: String::from("192.168.43.108"),
        port: String::from("8000"),
    });
    println!(
        "URL: myipv4: {} and myipv6 {}",
        myipv4.get_base_url(),
        myipv6.get_base_url()
    );
}
