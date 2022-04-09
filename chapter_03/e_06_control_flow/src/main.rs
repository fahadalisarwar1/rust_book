fn is_greater(num: i32) -> bool {
    if num > 5{
        true
    } else {
        false
    }
}
fn main() {
    println!("Control flow");
    let x = 2;
    println!("{} is greater than 5 = {}", x, is_greater(x));
    // tip: use match instead of making too many nested cases, it can get tricky, but/
    // keet flat structure

    let condition = true;

    let money = if condition {400} else {1};
    println!("money = {}", money);

    let money  = false;
    // let status = if money {1} else {"Rich"};// this is an error, 
    // rust must know all types at compile time

    

  
}
    