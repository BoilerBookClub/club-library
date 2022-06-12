use chrono::prelude::*;
use db::Database;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

mod db;

#[derive(PartialEq, Debug)]
pub struct Book {
    pub id: u64,
    pub title: String,
    pub author: String,
    pub genre: String,
    pub copies: usize,
    pub entered: String,
    pub using: Vec<Student>,
}

#[derive(PartialEq, Debug)]
pub struct Student {
    pub name: String,
    pub email: String,
}

impl Book {
    fn generate_id(title: &String) -> u64 {
        let mut s = DefaultHasher::new();
        title.hash(&mut s);
        s.finish()
    }
}

pub async fn add_book(title: String, author: String, genre: String, 
                      name: String, email: String) -> Result<(), ()> {
    let db = db::run().await;
    let mut books = db.retrieve().await?;
    let entered = Utc::now();

    db.log(entered, format!("{} ({}) added {} by {}.", 
                           name, email, title, author)).await?;

    let id = Book::generate_id(&title);
    for mut book in &mut books {
        if book.id == id {
            book.copies += 1;
            db.rewrite(books).await?;
            return Ok(());
        }
    }

    let book =  Book {
        id: Book::generate_id(&title), title, author, genre, 
        copies: 1, entered: entered.format("%m/%d/%y)").to_string(), using: vec![]
    };
    db.append(book).await?;

    Ok(())
}

pub async fn retrieve_books() -> Result<Vec<Book>, ()> {
    db::run().await.retrieve().await
}

pub async fn borrow_book(id: u64, student: Student) -> Result<bool, ()> {
    let db = db::run().await;
    let books = db.retrieve().await?;

    for mut book in books {
        if book.id == id {
            if book.using.len() >= book.copies {
                return Ok(false)
            }

            book.using.push(student);
            db.update(id, book).await?;

            return Ok(true)
        } 
    }

    Ok(false)
}

pub async fn return_book(id: u64, student: Student) -> Result<(bool), ()> {
    let db = db::run().await;
    let books = db.retrieve().await?;

    for mut book in books {
        if book.id == id {
            if !book.using.contains(&student) {
                return Ok(false)
            }

            book.using.retain(|x| *x != student);
            db.update(id, book).await?;

            return Ok(true)
        } 
    }

    Ok(false)
}