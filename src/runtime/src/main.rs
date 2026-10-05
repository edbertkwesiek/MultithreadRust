//use tokio::time::{Duration, sleep};

//async fn say_world() {
//    println!("world");
//    sleep(Duration::from_millis(500)).await;
//}

//async fn example(x: i32) -> i32 {
//    let x = async {
//        return 5;
//    };
//
//    sleep(Duration::from_millis(300)).await;
//}

//#[tokio::main]
//async fn main() {
// Calling `say_world()` does not execute the body of `say_world()`.
//    let _op = say_world().await;
//    let _instance_ = example().await;

// This println! comes first
//    println!("hello");
//    println!("Goodbye");

// Calling `.await` on `op` starts executing `say_world`.
//}

#[allow(unused_imports)]
use tokio::time::{Duration, sleep};

// First async function: simulates fetching data
//async fn fetch_user_data(user_id: u32) -> String {
//    println!("Fetching data for user {}...", user_id);
//    sleep(Duration::from_millis(500)).await; // Simulate network delay
//    format!("User Data for ID: {}", user_id)
//}

// Second async function: simulates processing data
//async fn process_data(data: String) -> String {
//    println!("Processing: '{}'...", data);
//    sleep(Duration::from_millis(300)).await; // Simulate work being done
//    data.to_uppercase()
//}

//#[tokio::main]
//async fn main() {
//    println!("--- Async Workflow Started ---");

// Execute the async functions sequentially using .await
//    let user_data = fetch_user_data(42).await;
//    let processed_result = process_data(user_data).await;

//    println!("Final Result: {}", processed_result);
//    println!("--- Async Workflow Completed ---");
//}
