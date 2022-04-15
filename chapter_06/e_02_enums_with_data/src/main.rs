// the previous example used enums but without data
// to use them we needed to define a struct with contained their address data and

// this can be complicated so we will create a new type of enum which can contain data fields. 

#[derive(Debug)]
enum IpAddr{
    V4(String),
    V6(String),
}

fn main() {
    println!("Enums with data");


    let home = IpAddr::V4("192.168.0.1".to_owned());

    let loopback = IpAddr::V6(":1".to_owned());

    println!("Home = {:?}", home);
    println!("Loopback = {:?}", loopback);

    // here you can see that the implementation of enum is quite consice and more easy looking. 
    // we dont need to define a struct here because data is already there 
    // the same scheme is also used in standard library, here  how it looks like
    /*
    
    std::net::IpAddr
        pub enum IpAddr {
            V4(Ipv4Addr),
            V6(Ipv6Addr),
        }


     */
    // the std lib example shows that you can embed any kind of data inside enums, not just primitive types like strings 
    

}
