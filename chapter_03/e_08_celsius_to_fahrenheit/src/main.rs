use std::io;


fn read_user_input() -> u32 {
    let mut user_input = String::new();
    let io_handler = io::stdin();
    io_handler
        .read_line(&mut user_input)
        .expect("failed to read");

    let user_option: u32 = match user_input.trim().parse() {
        Ok(num) => num,
        Err(err) => {
            println!("{}", err);
            0
        }
    };
    user_option
}

fn read_temp() -> f32 {
    let mut user_input = String::new();
    let io_handler = io::stdin();
    io_handler
        .read_line(&mut user_input)
        .expect("failed to read");

    let user_option: f32 = match user_input.trim().parse() {
        Ok(num) => num,
        Err(err) => {
            println!("{}", err);
            panic!("failed to parse");
        }
    };
    user_option
}

fn cen_to_fahrenheit() {
    println!("Enter Temperature(C): ");
    let temp_c = read_temp();
    let temp_f: f32 = (temp_c * 1.8) + 32.0;
    println!("temp in F = {}", temp_f);
}

fn fahrenheit_to_cel() {
    println!("Enter Temperature (F): ");
    let temp_f: f32 = read_temp();
    let temp_c: f32 = (temp_f - 32.0) / 1.8;
    println!("temp in C = {}", temp_c);
}

fn main() {
    println!("[+] ----------CEL TO FAR----------------\n");

    println!("\t\t[1] celcius to fahrenheit");
    println!("\t\t[2] fahrenheit to celcius");
    println!("[+] Select option: ");

    let user_option = read_user_input();
    match user_option {
        1 => cen_to_fahrenheit(),
        2 => fahrenheit_to_cel(),
        _ => println!("invalid option"),
    }
}
