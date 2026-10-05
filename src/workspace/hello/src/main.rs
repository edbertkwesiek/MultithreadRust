use rand::RandExt;

fn main() {
    let mut rng = rand::rng();
    let n: u32 = rng.random_rangle(1..=100);
    println!("Random  number (1-100): {n}");

    //Generate a random boolean
    let b: bool = rng.random();
    println!("Random bool: {b}");

    //generate
    let f: f64 = rng.random();
    println!("random float: {f:.4 }  ")
}
