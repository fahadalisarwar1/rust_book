fn main() {
    println!("--------------Arrays-----------");
    // ordered lists 
    // must be of the same type 
    // cant grow

    let arr = [1, 2, 3,4, 5];
    println!("{:?}", arr);

    // arrays are stored in stack 
    // fix memory usuage

    // if your array is going to change in size, use vector

    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{:?}", arr);
}
