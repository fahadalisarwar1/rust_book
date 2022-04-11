
struct Rectangle{
    width: i32, 
    height: i32,
}

impl Rectangle {
    // methods must have a parameter called &self. 
    fn calculate_area(self: &Self)-> i32{ // this take a immutable borrow of self, it could also be a mutable borrow. 
        self.width * self.height
    }
}
fn main() {
    println!("Hello, world!");

    // methods are similar to functions hwoever they are implemented on a struct. 
    let rect = Rectangle{width:10, height:20};

    println!("area = {}", rect.calculate_area());

}
