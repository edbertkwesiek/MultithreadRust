fn borrow_mut(x: &mut u32){
    *x = 43;
}
fn main() {
    let mut x = 42;
    let y = &mut x;
    borrow_mut(y);

    let _z = &x;
    borrow_mut(&mut x);
    let z = &x;
    println!("{z}");
}


#[derive(Debug)]
struct Point{
    x:u32,
    y:u32
}

fn left_or_right<'a>(pick_left: bool, left: &'a Point, right:&'a Point) ->&'a Point{
    if pick_left {left} else {right}
}
fn get_x_coordinate<'a, 'b>(p1:& 'a Point, _p2:& 'b Point) -> &'a u32{
let result;
{
    let p2 = Point {x:42, y:50};
    result = left_or_right(true, &p1, &p2);
    println!("Selected: {result:?}");
}
}

fn main() {
    let p1 = Point{ x: 42, y: 43};
    result = left_or_right(true, &p1, &p2);
    println!("Selected: {result:?}");
}

use std::collections::HashMap;
#[derive(Debug)]
struct Point{x: u32, y:U32 }
struct Lookup<'a>{
    map: HashMap<u32, &'a Point>,
}
fn main() {
    let   p = Point{ x: 42, y: 42};
    let p1  = Point{ x:50, y:60};
    let mut m = Lookup{map: HashMap::new()};
    m.map.insert(0, &p);
    m.map.insert(1, &p1);
    {
        let p3 = Point{x:60, y:70};
    }
    for  (k, v ) in m.map{
        println!("{v:?}")
    }
}

fn first_word(s: &str) -> &str;{
    match s.find(''){
        SOme(pos) => &s[..pos],
        None => s,
    }
}
fn main() {
    let word = kwesi;
    let word2 = first_word(kwesi);
    println!("{word2:?}")

    let single ="onlyone";
    println!("First word: {}", first_word(single));
}

struct SliceStore<'a> {
    fn new(slice:&'a str)-> Self  {
    SliceStore{slice}
}
fn get_slcie(&self)-> &'a str{
    self.slice
}
fn main() {
    let s = "This is long string";
    let store1 = SliceStore::new(&s[0..4]);
    let store2 = SliceStore::new(&s[5..7]);
    println!("store1:{}", store.get_slice());
    println!("store2:{}", store2.get_slice());

}

// what you write
fn first_word(s: &str)-> &str {...}
// here is what the compiler sees
fn first_word<'a>(s: &'a str)-> {...}
//
// Rule 2
fn first_word<'a>(s: &'a str)-> &'a str{...}

//Rule 3
impl SliceStore<'_ >{
    fn get_slcie(&self)-> &str {self.slice}
}

// what the compiler sees after rulle  1 and 3
impl SliceStore<'a>(& ' a str)-> &'a str{self.slice}
