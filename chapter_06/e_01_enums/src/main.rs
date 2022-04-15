#[derive(Debug)]
enum IpAddrKind{
    Version_4, 
    Version_6,
}

#[derive(Debug)]
struct IpAddr{
    kind: IpAddrKind,
    Address: String,
}

fn main() {

    let home_ip = IpAddr{
        kind: IpAddrKind::Version_4,
        Address: String::from("192.168.1.55"),
    };

    let loopback = IpAddr{
        kind: IpAddrKind::Version_6,
        Address: String::from(":1"),
    };

    println!("home  = {:?}", home_ip);
    println!("loopback = {:?}", loopback);





}
