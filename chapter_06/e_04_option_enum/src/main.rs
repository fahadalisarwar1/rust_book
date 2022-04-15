
/**
 * Option enum is defined by the standard library. 
 * it encoddes the scenario where value could be "Something" or "Nothing"
 * Option enum is already included in the prelude, you dont need to explicitly bring it into scope
 * 
 */
fn main() {
    println!("Option Enum!");
    let some_number: Option<i32> = Some(35);
    let some_string: Option<String> = Some("Fahad".to_owned());

    let absent_number: Option<i32> = None;
    println!("{:?}", absent_number);


    /*
    This wont compile 
        let x: i8 = 3;
        let y: Option<i8> = Some(4);

        let sum = x + y;  this is an error because x and y are not the same type

        the compiler wont let us use y unless we have the value. 

        everytime there is a value of type Option, you have to explicitly handle the None case, 
        this ensures that you dont run into a value which is null and not handled.

        mostly it is done with the help of match expression 
        The match expression is a control flow construct that does just this when used with enums: 
        it will run different code depending on which variant of the enum it has, 
        and that code can use the data inside the matching value

     */


}
