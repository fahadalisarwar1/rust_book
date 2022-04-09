fn display_num(num: i32){
    println!("number is {}", num);
}

fn display_two_numbers(num1: i32, num2: i32){
    println!("number 1 is {}", num1);
    println!("number 2 is {}", num2);
}

fn returns_something() -> i32{
    println!("This function returns a number" );
    5
}
fn main() {
    println!("functions");
    display_num(3);
    display_two_numbers(10, 20);

    // rust doesnt support default values for arguments
    // there is no function overloading either

    // statements end in ;'
    // statement dont returna  a value

    // expressions dont end with semicolon

    // expressions return something

    let num = returns_something();
    println!("num is {}", num);

    let x = {
        let y = 30; // statement
        y + 5 // expression
    };
    println!("x is {}", x);

}
