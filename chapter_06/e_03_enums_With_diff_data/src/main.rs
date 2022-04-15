
enum Action{
    Quit,
    Move(i32, i32),
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    let act = Action::Quit;
    println!("Enums with different data types");

}
