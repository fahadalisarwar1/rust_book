fn main() {
    println!("E01 - Variables");
    let x = 5;
    println!("x = {}", x);


    // since x is immutable, lets try to mutate it
    //   x = 55; // this will cause an error
    println!("x = {}", x);

    // ---------------------------------------------------------------------
    
    // lets create a mutable variable
    let mut x_mut = 30;
    println!("mutable x is {}", x_mut);
    x_mut = 44;
    println!("mutable x is now {}", x_mut);

    /* ----------------------------------------------------------------------*/
    // lets create a constant
    const PI: f32 = 3.14;  // you can't use 'let' keyword here
    println!("{}", PI);
    // constant can be declared in any scope

    /*------------------------------------------------------------- */
    // shadowing
    let x = 4;
    let x = x * 4; // even though it is immutable, we are not assigning it, but rather redeclaring it

    println!("x = {}", x);


    
     
 
}
