use chrono::prelude::*;
use db::Database;
use serde::{Serialize, Deserialize};
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

mod db;
mod util;

#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub struct Book {
    pub id: String,
    pub title: String,
    pub author: String,
    pub genre: String,
    pub copies: usize,
    pub entered: String,
    pub image: String,
    pub using: Vec<Student>,
}

#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub struct Student {
    pub name: String,
    pub email: String,
}

impl Book {
    fn generate_id(title: &String) -> String {
        let mut s = DefaultHasher::new();
        title.hash(&mut s);
        s.finish().to_string()
    }
}

pub async fn add_book(title: String, author: String, genre: String, 
                      name: String, email: String) -> Result<(), ()> {
    let db = db::run().await;
    let books = db.retrieve().await?;
    let entered = Local::now();

    db.log(entered, format!("{} ({}) added {} by {}.", 
                           name, email, title, author)).await?;

    let id = Book::generate_id(&title);

    let mut index = 0;
    for mut book in books {
        if book.id.eq(&id) {
            book.copies += 1;
            db.update(index, book).await?;
            return Ok(());
        }

        index += 1;
    }

    let image = util::retrieve_image(&title).await.unwrap();

    let book =  Book {
        id, title, author, genre, 
        copies: 1, entered: entered.format("%m/%d/%y").to_string(), using: vec![],
        image 
    };
    db.append(book).await?;

    Ok(())
}

pub async fn retrieve_books() -> Result<Vec<Book>, ()> {
    db::run().await.retrieve().await
}

pub async fn retrieve_borrowing_books(name: &str, email: &str) -> Result<Vec<Book>, ()> {
    let student = Student {name: name.to_string(), email: email.to_string() };
    let books = db::run().await.retrieve().await.unwrap();
    let mut student_books: Vec<Book> = Vec::new();

    for book in books {
        if book.using.contains(&student) {
            student_books.push(book);
        }
    }

    Ok(student_books)
}

pub async fn borrow_book(id: String, name: &str, email: &str) -> Result<(), ()> {
    let student = Student {name: name.to_string(), email: email.to_string() };
    let db = db::run().await;
    let books = db.retrieve().await?;


    let mut index = 0;
    for mut book in books {
        if book.id.eq(&id) {
            if book.using.len() >= book.copies {
                return Err(())
            }

            db.log(Local::now(), format!("{} ({}) has borrowed {} by {}.",
                        name, email, book.title, book.author)).await?;

            book.using.push(student);
            db.update(index, book).await?;

            return Ok(())
        } 

        index += 1;
    }

    Err(())
}

pub async fn return_book(id: String, name: &str, email: &str) -> Result<(), ()> {
    let student = Student {name: name.to_string(), email: email.to_string() };
    let db = db::run().await;
    let books = db.retrieve().await?;

    let mut index = 0;
    for mut book in books {
        if book.id.eq(&id) {
            if !book.using.contains(&student) {
                return Err(())
            }

            db.log(Local::now(), format!("{} ({}) has returned {} by {}.",
                        name, email, book.title, book.author)).await?;

            book.using.retain(|x| *x != student);
            db.update(index, book).await?;

            return Ok(())
        } 

        index += 1;
    }

    Err(())
}
