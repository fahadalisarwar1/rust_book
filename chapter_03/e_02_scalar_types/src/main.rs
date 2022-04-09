


fn main() {
    let x: u32 = 100;
    println!("x u32 is {}", x);

    let x: i32 = -344;
    println!("x i32 is {}", x);

    let y: f32 = 445.3;
    println!("y f32 is {}", y);

    let z: f64 = 44.555;
    println!("z f64 is {}",z);

    // let x: u8 = 455;  // this is overflow error
    // println!("x u8 is {}",x);

    let g = 33.486; // f32 infered
    let k: f32 = 99.0; // f32


    let sum = g + k; // this is ok (here g is type infered to f32)
    println!("sum = {}",sum );

/*
    let g: f64 = 77.99;
    let k: f32 = 77.55;
    let sum = g + k; // this will not work 
    println!("sum = {}", sum);
*/
    let is_rich: bool = false;
    println!("am i rich = {}",is_rich);


    // math 
    let x = 3;
    let y = -26;
    println!("{}", y % x);

    // characters 
    // rust char is 4 bytes and represents unicode
    let heart_eyed_cat = '😻';
    println!("{}", heart_eyed_cat);

}
