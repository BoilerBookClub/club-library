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
    let books = db.retrieve().await?;
    let entered = Local::now();

    db.log(entered, format!("{} ({}) added {} by {}.", 
                           name, email, title, author)).await?;

    let id = Book::generate_id(&title);

    let mut index = 0;
    for mut book in books {
        if book.id == id {
            book.copies += 1;
            db.update(index, book).await?;
            return Ok(());
        }

        index += 1;
    }

    let book =  Book {
        id: Book::generate_id(&title), title, author, genre, 
        copies: 1, entered: entered.format("%m/%d/%y").to_string(), using: vec![]
    };
    db.append(book).await?;

    Ok(())
}

pub async fn retrieve_books() -> Result<Vec<Book>, ()> {
    db::run().await.retrieve().await
}

pub async fn borrow_book(id: u64, name: &str, email: &str) -> Result<bool, ()> {
    let student = Student {name: name.to_string(), email: email.to_string() };
    let db = db::run().await;
    let books = db.retrieve().await?;


    let mut index = 0;
    for mut book in books {
        if book.id == id {
            if book.using.len() >= book.copies {
                return Ok(false)
            }

            db.log(Local::now(), format!("{} ({}) has borrowed {} by {}.",
                        name, email, book.title, book.author)).await?;

            book.using.push(student);
            db.update(index, book).await?;

            return Ok(true)
        } 

        index += 1;
    }

    Ok(false)
}

pub async fn return_book(id: u64, name: &str, email: &str) -> Result<bool, ()> {
    let student = Student {name: name.to_string(), email: email.to_string() };
    let db = db::run().await;
    let books = db.retrieve().await?;

    let mut index = 0;
    for mut book in books {
        if book.id == id {
            if !book.using.contains(&student) {
                return Ok(false)
            }

            db.log(Local::now(), format!("{} ({}) has returned {} by {}.",
                        name, email, book.title, book.author)).await?;

            book.using.retain(|x| *x != student);
            db.update(index, book).await?;

            return Ok(true)
        } 

        index += 1;
    }

    Ok(false)
}