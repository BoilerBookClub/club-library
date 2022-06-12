use club_library::retrieve_books;

#[tokio::main]
async fn main() {
    println!("Application started!");
    println!("{:?}", retrieve_books().await.unwrap());
}