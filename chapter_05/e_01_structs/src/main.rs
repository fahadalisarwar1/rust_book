
#[derive(Debug)]
struct User{
    name: String,
    email: String,
    login_count: u32,
    is_active: bool,
}
fn main() {
    // structs are named compound data types that
    let u1: User = User{
        name: String::from("Fahad"),
        email: String::from("fahad@gmail.com"),
        login_count: 3,
        is_active: true,
    };

    println!("the user name  is ={}", u1.name);
    println!("the email is ={}", u1.email);
    println!("the login_count is {}", u1.login_count);
    println!("user active = {}", u1.is_active);
}
