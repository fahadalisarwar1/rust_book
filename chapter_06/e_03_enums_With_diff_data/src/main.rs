
enum Action{
    Quit,
    Move(i32, i32),
    Write(String),
    ChangeColor(i32, i32, i32),
}

// we can define an impl block on enums as well

impl Action {
    fn do_action(self: &Self){
        println!("Action is being done");
    }
}
fn main() {
    let act = Action::Quit;
    println!("Enums with different data types");
    act.do_action();

}
