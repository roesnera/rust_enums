fn main() {
    let ip_4 = IpAddrKind::V4;
    let ip_6 = IpAddrKind::V6;

    println!("{:?}",ip_4);
    println!("{:?}",ip_6);
}

// creates an enum that can be formatted using the debug formatter
#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6
}
