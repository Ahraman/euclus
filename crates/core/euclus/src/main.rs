use dotenvy;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().unwrap();

    println!("Hello world!");
}
