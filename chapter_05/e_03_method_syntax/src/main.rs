
struct Rectangle{
    width: i32, 
    height: i32,
}

impl Rectangle {

    // all functions defined within the impl block are called associated functions. 
    // this function is not "associated" since if doesnt take self as parameter, therefor is not implemented on an instance. 
    fn new()-> Rectangle{
        let rect = Rectangle{width: 0, height: 0};
        rect
    }


    // methods must have a parameter called &self. 
    fn calculate_area(self: &Self)-> i32{ // this take a immutable borrow of self, it could also be a mutable borrow. 
        self.width * self.height
    }

    fn width(self: &Self)-> i32{ // getter method for width
        if self.width > 0 { // check to ensure width is greater than zero
            self.width
        }else{
            0
        }
    }
    fn height(self: &Self)-> i32{ // getter method for height
        self.height
    }

    fn can_hold(self: &Self, other: &Rectangle) -> bool{
        self.width > other.width && self.height > other.height
    }
}
fn main() {
    println!("Hello, world!");

    // methods are similar to functions hwoever they are implemented on a struct. 
    let rect = Rectangle{width:10, height:20};

    println!("width is {}", rect.width());
    println!("height is {}", rect.height());
    println!("area = {}", rect.calculate_area());

    let rec2 = Rectangle{width:40, height:50};

    println!("{}", rec2.can_hold(&rect));
    let rec3 = Rectangle::new(); // non associated functions can be accessed like this with :: syntax

    println!("{}", rec3.width());
}

// summary: 
/*

1: methods are functions defined on a struct
2: they included self as a parameter
3: method can take another instance of the same type as an argument, take a look at the can_hold() example
4: you can define non associated functions inside an implementation block that are defined on structs itself, they dont take self
as an argument 
5: you can define multiple impl blocks for same struct with different functions. 
6: you can define setter and getter methods on the struct. 

Structs let you create custom types that are meaningful for your domain. By using structs, 
you can keep associated pieces of data connected to each other and name each piece to make your code clear. 
In impl blocks, you can define functions that are associated with your type, and methods are a kind of associated function that 
let you specify the behavior that instances of your structs have.


 */