fn main() {
    println!("Loops");
    let mut i = 0;
    let new_i = loop {
        println!("i is {}", i);
        i = i + 1;
        if i >= 6 {
            break i; // returning some value out of loop
        }
    };

    println!("{}", new_i);

    // while loop 

    let mut x = 10;

    while x >= 0{
        println!("{}", x);
        x = x - 1; 
    }


    println!("=================================================");

    let arr = [10, 20, 30, 40, 50];

    for item in arr.iter(){
        println!("{}", item);
    }

    println!("=================================================");

    for num in (1..6).rev(){
        println!("{}", num);
    }
}
