pub enum IpAddr {
    V4(IpAddrV4),
    V6(IpAddrV6),
}

pub struct IpAddrV4 {
    pub host: String,
    pub port: String,
}
pub struct IpAddrV6 {
    pub host: String,
    pub port: String,
}

impl IpAddr {
    pub fn get_base_url(&self) -> String {
        match self {
            IpAddr::V4(data) => String::from("V4:") + &data.host + ":" + &data.port,
            IpAddr::V6(data) => String::from("V6:") + &data.host + ":" + &data.port,
        }
    }
}
