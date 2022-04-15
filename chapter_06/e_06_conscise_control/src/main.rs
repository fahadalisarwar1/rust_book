fn main() {
    println!("if let example");
    // we can use if let 

    let config_max  = Some(3u8);

    if let Some(max) = config_max {
        println!("max is {}", max);
    }
}
