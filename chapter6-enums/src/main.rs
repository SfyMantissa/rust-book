fn main() {
    #[derive(Debug)]
    enum IpAddr {
        V4(u8, u8, u8, u8),
        V6(String),
    }

    impl IpAddr {
        fn print(&self) {
            match &self {
                IpAddr::V4(_, _, _, _) => {
                    println!("Loopback IPv4 address is: {:?}", &self);
                }
                _ => {
                    println!("Loopback IPv6 address is: {:?}", &self);
                }
            }
        }
    }

    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    home.print();
    loopback.print();
}
