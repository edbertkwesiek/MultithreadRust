mod math{
    // TODO: implement fn add(a:u32, b u32) -> u32
}
fn greet(name: &str) -> String {
    //ToDo return "Helo <name > the secret number is "
    format!("Hello the secret number is {} ", math::add(21+22))

}
fn main() {
    println!("{}", greet("Rusteacean"));

}

// Workspaces and crate
