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

}
