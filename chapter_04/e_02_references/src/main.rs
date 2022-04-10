fn calculate_len(s: &String) -> usize { // this uses references
    // this is immutbale borrow so this function cant change the original String
   //  s.push_str("hehe");  // this will not work 
    s.len()
}  

fn mutable_borrow(s: &mut String)  {
    // this is a mutable reeference to a string, this means it can change the original String
    // to be able to change, the borrowed reference must be mutable.
    // you can only have 1 mutable reference in a scope


    s.push_str(" hellow")
}
fn main() {
    println!("References");

    let s = String::from("This is a string");

    println!("{}", calculate_len(&s)); // calculate_len borrows the string, instead of moving it, 
    println!("string can still be used here : {}", s); 


    let mut s1 = String::from("This is a string");
    mutable_borrow(&mut s1);
    println!("{}", s1);

    let mut str1 = String::from("This is a string");

    /*
        let r1 = &mut str1;
        let r2 = &mut str1;
        println!("{}", r1);
        println!("{}", r2);  // this is an error


*/
    
    {
        // this is ok becasue it is not in the same scope
        let r2 = &mut str1;
        println!("{}", r2);
    }
    let r1 = &mut str1;
    println!("{}", r1);  

    // you can create as many immuttable references as you want
    // however you cant have mutable and immutable reference in the same scope

    // in rust there are no dangling points

    // summary
    /* 
    1: ownership is moved only for data types stored oon heap.
    2: data types stored on stack do deep copy
    3: tuples can be stored on stack or heap depending on their contents.
    4: if the tuple contenets are primitive, they are stored on stack
    5: references are used to borrow values
    6: there can be many immutable references
    7: there can be only only 1 mutable reference at a time
    8: there cant be mutable and immutable reference in the same scope. 
    9: you can use .clone() method to do deep copy explicitly
    10: only mutable references can change the value of a variable
    11: there are no dangling pointers 
    
    */
}
