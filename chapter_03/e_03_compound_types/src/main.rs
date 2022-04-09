fn main() {
    println!("Compound types");
    // two primitive compound types are tuples and arrays

    // tuples group together other primitive types
    // tuples are not growable

    let coordinates = (10, 10, 30); // creating a tuple
    println!("coordinates = {:?}", coordinates);

    let (x, y, z) = coordinates;  // deconstructing a tuple
    println!("x = {:?}", x);
    println!("y = {:?}", y);
    println!("z = {:?}", z);

    // tuples can have different types 
    let info = ("fahad", 34);

    let (name, age) = info; // name: &str, age: i32
    println!("name = {:?}", name);
    println!("age = {:?}", age);

    // accessing via index access

    let book = ("Lord of the rings", 500);

    println!("Book name is {:?}", book.0);
    println!("Book pages are {:?}", book.1);

}
