use std::time::Duration;
use tokio;

#[tokio::main]
async fn main() {
    println!("Hello, world!");

    let mut value = 0u32;

    loop {
        tokio::select! {
            _ = f(&mut value) => {
                println!("f called");
            }
            _ = g() => {
                println!("g called");
                value += 1000;
            }
            else => {
                println!("end");
                break;
            }
        }
    }
}

async fn f(value: &mut u32) {
    println!("before: {value}");
    tokio::time::sleep(Duration::from_millis(500)).await;
    *value += 1;
    println!("after: {value}");
}

async fn g() {
    tokio::time::sleep(Duration::from_millis(200)).await;
}
