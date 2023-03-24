use std::time::Duration;
use tokio;

#[tokio::main]
async fn main() {
    println!("Hello, world!");

    tokio::pin! {
        let fut_f = f();
        let fut_g = g();
    }
    loop {
        tokio::select! {
            _ = &mut fut_f => {
                fut_f.set(f()); // .set() is a method on Pin
                println!("f called");
            }
            _ = &mut fut_g => {
                fut_g.set(g()); // .set() is a method on Pin
                println!("g called");
            }
            else => {
                println!("end");
                break;
            }
        }
    }
}

async fn f() {
    println!("before");
    tokio::time::sleep(Duration::from_millis(500)).await;
    println!("after");
}

async fn g() {
    tokio::time::sleep(Duration::from_millis(200)).await;
}
