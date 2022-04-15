
enum Coin{
    Penny,
    Nickel,
    Dime, 
    Quarter,
    
}

fn is_nickel(coin: &Coin) -> bool {
    match coin {
        Coin::Nickel => true,
        _ => false,
    }

}

fn get_value_in_cents(coin: &Coin) -> u8 {
        match coin {
            Coin::Penny => 1,
            Coin::Nickel => {
                // if you want to run multiple lines for a match, use curly brackets
                println!("this is a nickel");
                5
            },
            Coin::Dime => 10,
            Coin::Quarter => 25,
            _ => {
                println!("invalid");
                0
            }
        }
        
}

fn plus_one(x: Option<i32>)->Option<i32> {
    match x{
        None => None,
        Some(value)=> Some(value+1)
    }
}


fn main() {
    println!("MATCH control flow");
    /* 

    Rust has a match control flow. 

    patterns are used for matching
    all cases must be matched 

    */

    let my_coin = Coin::Nickel;
    let coin_value = get_value_in_cents(&my_coin);

    println!("coin value: {}", coin_value);


    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    /*
    Combining match and enums is useful in many situations. You’ll see this pattern a lot in Rust code: match against an enum, 
    bind a variable to the data inside, and then execute code based on it. 
    It’s a bit tricky at first, but once you get used to it, you’ll wish you had it in all languages. 
    It’s consistently a user favorite.
     */


     // Matches are exhaustive , all the options needs to be covered.

     let my_coin = Coin::Nickel;
     println!("{}", is_nickel(&my_coin));


     let dice_roll = 6;
     match dice_roll{
         6 => println!("you win"),
         _ => println!("roll again") // use _ option for any other case. 
     }

}
