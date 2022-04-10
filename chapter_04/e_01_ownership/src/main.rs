fn main() {
    println!("Ownership");

    let str_literal = "this is a str literal"; // this is stored on stack


    let my_str = String::from("Hello world"); // string literals are immutable by default
    // the above statement allocates memory on heap
    let mut new_str = String::from("Hello world"); // this is stored on heap. so it can be changed
    new_str.push_str(", losers");

    println!("my string is : {}", new_str);

    // when a variable goes out of scope, rust has drop method to free memory.

    /* str has 3 parts
    1: pointer to heap loc of string
    2: length
    3: capacity of string

    these 3 are stored on stack whilw string itself is stored on heap

    */
    let s1 = String::from("move");
    let s2 = s1; // here stack data is copied not the contents of heap, 
    // this is move
    // both s1 & s2 are pointing to the same memory
    // when they go out of scope, rust has drop method to free memory, both will try to free memory 
    // which is a bug called double free error

    // to get rid of this error, rust considers s1 invalid when s2 = s1 is called, only s2 is valid
    // in rust shallow copy is called a move

    let m1 = String::from("clone");
    let m2 = m1.clone(); // deep copy

    println!("{}", m1);


    // primitive data are stred on stack like int flloat, arr etc. 
    // they implement copy trait

    takes_ownership(m1);
    // println!("{}",m1); // this is not valid since value is moved already. 

    let k1 = String::from("Fahad");
    let k1 = takes_ownership_and_give_back(k1);
    println!("{}",k1);

}

fn takes_ownership(s: String){
    println!("{}", s);

}

fn takes_ownership_and_give_back(s: String)->String{ // this method takes ownership of the string , however also returns when it is done with the value. 
    println!("{}", s);
    s
}