
struct Color(i32, i32, i32);

#[derive(Debug)]
struct Rectangle{
    width: i32,
    length: i32,
}
fn main() {
    let color_1 = Color(10, 20, 255);
    println!("{}", color_1.0);
    println!("{}", color_1.1);
    println!("{}", color_1.2);


    let rect = Rectangle{width:10, length:20};

    println!("rectangle is {:?}", rect);
    // primitive data types implement the std::fmt::Display trait. 
    println!("Area of the rectangle is {}", calculate_area(&rect));
    dbg!("Area {}", &rect);
    // unit like structs 

    // you can also use dbg!() macro as well, however this macro prints output to stderr stream rather than stdout ,



}

fn calculate_area(rect: &Rectangle) -> i32 {
    rect.width * rect.length
}