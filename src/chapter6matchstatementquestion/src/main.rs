

enum Operation{
    Add(u64, u64),
    Subtract(u64, u64),
}

enum CalcResult {
    ok(u64),
    Invalid(String),
}

fn calculate(op: Operation) -> CalcResult{
    match op{
        Operation::Add(a, b) =>CalcResult:: Ok(a + b),
        Operation::Subtract(a, b) =>{
            if  a >= b  {
                CalcResult::Ok(a - b)
            }else {CalcResult::Invalid("Underflow".to_string())}
        }

    }


    //


fn main() {
    match calculate(Operation::Add(10, 20)){
        CalcResult::ok(Result)=> println!("10 + 20 = {result}"),
        CalcResult::Invalid(msg) => println!("Error: {msg}"),
    }
    match calculate(Operation::Subtract(5, 10)) {
        CalcResult::Ok(result) => println!("5 - 10 = {result}"),
    }
}
