// Rust lesson on memeory management 
fn main() {
    let a = 42; 
    let b = &a; //first borrow 
    {
        let aa =42;
        let c = &a;
        
    }
}

fn foo(x:&u32) {
    println!("{x}");
}
fn bar(x:u32) {
    println!("{x}");
}
fn main() {
    let a = 42; 
    foo(&a);
    bar(a);
}

fn foo(s: String) {
    println!("{s}");
    //The heap memory pointed to by s will be deallocated
}
fn bar(s: &String){
    println!("{s}");
}
fn main() {
    let s = String::from("Rust string move example");
    foo(s);
    let t = String::from("Rust string borrow example");
    bar(&t);
    println!("{t}");
}

struct Point{
x:u32,
y:u32,
}
fn consume_pointer(p:Point){
    println!("{} {}", p.x, p.y);
    
}
fn borrow_pointer(p: &Point){
    println!("{} {}", p.x, p.y);
}

fn main() {
    let p = Point{x:10, y:20};
    //
    borrow_point(&p);
    consume_point(p);
}

fn main() {
    let s = String::from("Rust");
    let s1 = s.clone();
    println!("{s1}");
    println!("{s}");
}

#[derive(Copy, Clone, Debug )]
struct Point{x: u32, y:u32}
fn main() {
    let p = Point{x:32, y:40};
    let p1 = p;
    println!("p: {p:?}");
    println!("p1:{p:?}");
    let p2 = p1.clone();
}

struct Point { x:u32, y:u32}
//
impl Drop for Point{
    fn drop(&mut self){
        println!("Good bye point x:{}, y:{}", self.x, self.y);
    }
}

fn main() {
    let p = Point{x:43, y:42};
    {
        let p1 = Point{x:43, y:43};
        println!("Exiting inner block ")
    }
    println!("Exiting main ")
}


struct Coordinate {x: u32, y:u64, z:u32}

impl Drop for Coordinate{
    fn drop(&selfmut) {
        println!(" ")
    }
}
fn operator(numereal:u32) -> Result{
    numereal.len()
}