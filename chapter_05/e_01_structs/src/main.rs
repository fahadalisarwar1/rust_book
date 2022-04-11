
#[derive(Debug)]
struct User{
    name: String,
    email: String,
    login_count: u32,
    is_active: bool,
}
fn main() {
    // structs are named compound data types that
    let u1: User = build_user("fahad".to_owned(), "fahad@gmail.com".to_owned());

    println!("the user name  is ={}", u1.name);
    println!("the email is = {}", u1.email);
    println!("the login_count is = {}", u1.login_count);
    println!("user active = {}", u1.is_active);

    let user2 = User{
        name: String::from("jinko"),
        email: String::from("jinkp@h2v.com"),
        ..u1
    };

    println!("{}", user2.login_count);
}

fn build_user(name: String, email: String)->User{
    let user = User{
        email: email,
        //email,   // just email, this is also Ok

        name: name,
        login_count: 0,
        is_active: true,
    };

    user
}