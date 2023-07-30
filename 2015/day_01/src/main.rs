mod get_input;
use tokio;

#[tokio::main]
async fn main() {
    let input: String = get_input::get_input().await;
    println!("Hello, world!");
}
