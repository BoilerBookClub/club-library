use club_library::*;

#[tokio::main]
async fn main() {
    println!("Application started!");
    println!("{:?}", return_book(9626017388390603956, "Kai Tinkess", "kaitinkess@gmail.com").await);
}