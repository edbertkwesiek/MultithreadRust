
//Box<T> for heap location builds on ownership and lifetime concepts
fn main() {
    // create a pointer to an integer with value 42 created on heap
//    let f = Box::new(42);
//    let s = Box::new(house);
    // is it allowed to use a string ina box in rust .
//    println!("{} {}" *f, f );
    // cloning the box creates new heap allocation
//    let mut g = f.clone();
//    *g = 43;
//    println!("{f} {g} ");
//}

fn rust_ownership_safety(){
    let data = Box::new(42);

    let moved_data = data;  // ownership is transfferd to moved_data.

    let borrowed = &moved_data; //immutable borrow
    println!(" {}", borrowed);
}


fn borrowing_rules_examples(){
    let mut data = vec![1, 2, 3, 4, 5];

   //multiple immutable borrows
   let ref1 = &data;
  let ref2 = &data;
 println!("{:?} {:?}", ref1, ref2 );

//mutable borrow
let ref_mut = &mut  data;
ref_mut.push(6);   // ref 1 and 2 cannot be used whilst ref_mut is active

let ref3 = &data;
println!("{:?}", ref3);


}

// interior mutability RefCell<T> and Cell<T>
struct Employee {
    employee_id: u64,
    on_vacation: bool,
}

#[derive(Debug)]
struct Empployee{
    employee_id: u64
}
fn main() {
    let mut us_employees = vec![];
    let mut all_global_employees = Vec::<Employee>::new();
    let employee = Employee{employee_id: 42};
    us.employees .push(employee);

}


use std::rc::Rc;
#[(derive debug)]

struct Employees { employee_id:u64}
fn main(){
    let mut us_employees = vec![];
    let mut all_global_employees = vec![];
    let employee = Employee{employee_id: 42};
    let employee_rc = Rc::new(employee);
    us.employees.push(employee_rc.clone());
    all_global_employees.push(employee_rc.clone());
    let employee_one = all_global_employees.get(0);
    for e  in us_employees{
        println!("{}", e.employee_id);
    }
    println!("employee_one:?");
}



use std::rc::{Rc, weak};

struct Node {
    value: i32,
    parent: Option<weak<Node>>,
}

fn main(){
    let parent = Rc::new(Node{ value: 1, parent: None });
    let child = Rc::new(Node{ value: 2, parent: Some(Rc::downgrade(&parent))});
    // to use weak
    let Some(parent_rc) = child.parent.as_ref().unwrap().upgrade(){
        println!("Parent value: {}", parent_rc.value);
    }

    println!("Parent strong count: {} ",Rc::strong_count(&parent));
}


use std::cell::{cell, RefCell};
use std::rc::RC;

#[derive(Debug)]
struct Employee{
    employee_id: u64,
    name:RefCell<String>,
    on_vacation: Cell<bool>,
}
fn toggle_vacation(emp: &Employee){
    //
    emp.on_vacation.set(!emp.on_vacation.get());

}
fn append_title(emp: &Employee, title: &str){
    emp.name.borrow_mut().push_str(title);

}
fn main() {

    let us_employees = Employee::employee_id.clone();
    let details = RC::new::(Employee{ employee_id: 68, name: kwesi, on_vacation:True});
    let  us_employee = details.clone();
    let global_employees = details.clone();
    println!("{},{}" )



}

fn main() {
    let emp = Rc::new(Employee{
        employee_id: 42,
        name:RefCell::new("Alice".to_string()),
        on_vacation:Cell::new(false),
    });
    let mut us_employees = vec![];
    let mut global_employees = vec![];
    us_employees.push(Rc::clone(&emp));
    global_employees.push(Rc::clone(&emp));

    //toggle vacation throug /an immutable refference
    toggle_vacation(&emp);
    println!("on vacation: {}", emp.on_vacation.get());

    //append title through an immutable refference
    append_title("&emp, sr.engineer");
    println!("Name: {}", emp.name.borrow());

    //Both vecs
    println!("Global: {:?}", global_employees[0].name.borrow());
    println!("US: {:?}", global_employees[0].name.borrow());
    println!("Rc strong count: {}", Rc::strong_count(&emp));
}
