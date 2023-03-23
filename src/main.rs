use std::time::Duration;
use std::sync::Arc;

use tokio;

#[tokio::main]
async fn main() {
    println!("Hello, world!");

    let notify = Arc::new(tokio::sync::Notify::new());

    let fut = async move {
        let fut_f = async move {
            loop {
                f().await;
            }
        };
        let fut_g = async move {
            loop {
                g().await;
            }
        };

        tokio::join!(fut_f, fut_g);
    };

    tokio::spawn(stop_later(notify.clone()));

    tokio::select! {
        _ = fut => {
            println!("fut_join");
        }
        _ = notify.notified() => {
            println!("notified");
        }
    }
}

async fn f() {
    println!("f() starts");
    tokio::time::sleep(Duration::from_millis(500)).await;
    println!("f() ends");
}

async fn g() {
    println!("g() starts");
    tokio::time::sleep(Duration::from_millis(200)).await;
    println!("g() ends");
}

async fn stop_later(notify: Arc<tokio::sync::Notify>) {
    tokio::time::sleep(Duration::from_millis(2000)).await;
    notify.notify_one();
}
