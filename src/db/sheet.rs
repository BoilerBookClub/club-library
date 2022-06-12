use super::*;

extern crate google_sheets4 as sheets4;
use sheets4::api::ValueRange;
use sheets4::{Sheets, oauth2, hyper, hyper_rustls};
use sheets4::Error;
use async_trait::async_trait;

const SPREADSHEET_ID: &str = "103Y8_w8Wu-9tYM0lsrdXSl7yysomt6LT1TzKPNA7VGQ";

pub struct SheetDatabase {
    hub: Sheets,
}

impl SheetDatabase {
    pub async fn new() -> SheetDatabase {
        let secret = oauth2::read_application_secret("credentials.json").await.expect("Could not read secret.");
        let auth = oauth2::InstalledFlowAuthenticator::builder(secret, oauth2::InstalledFlowReturnMethod::HTTPRedirect)
            .persist_tokens_to_disk("tokencache.json").build().await.unwrap();

        let hub = Sheets::new(hyper::Client::builder().build(
            hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), 
            auth);

        SheetDatabase { hub }
    }
}

#[async_trait]
impl Database<Book> for SheetDatabase {
    async fn retrieve(&self) -> Result<Vec<Book>, ()> {
        let result = self.hub.spreadsheets().values_get(SPREADSHEET_ID, "_Library!A2:H").doit().await;
        let values = validate(result).1.values.unwrap();

        println!("{:?}", values);

        let mut books: Vec<Book> = Vec::new();
        for row in values {
            let students: Vec<Student> = match row.get(7) {
                None => Vec::new(),
                Some(text) => {
                    let tokens = text.split(";");
                    let mut builder: Vec<Student> = Vec::new();

                    for token in tokens {
                        if token.is_empty() { continue };

                        let mut delim = token.split(":");
                        
                        builder.push(Student { 
                            name: delim.next().unwrap().to_string(), email: delim.next().unwrap().to_string()
                        })
                    }

                    builder
                }
            };

            books.push(Book { 
                        id: row.get(0).unwrap().parse().unwrap(), 
                        title: row.get(2).unwrap().to_string(),
                        author: row.get(3).unwrap().to_string(), 
                        genre: row.get(4).unwrap().to_string(), 
                        copies: row.get(5).unwrap().parse().unwrap(), 
                        entered: row.get(1).unwrap().to_string(), 
                        using: students 
                    })
        }
     
        Ok(books)
    }

    async fn rewrite(&self, new: Vec<Book>) -> Result<(), ()> {
        Ok(())
    }

    async fn append(&self, new: Book) -> Result<(), ()> {
        Ok(())
    }

    async fn update(&self, id: u64, values: Book) -> Result<(), ()> {
        Ok(())
    }

    async fn log(&self, time: DateTime<Utc>, update: String) -> Result<(), ()> {
        Ok(())
    }
}

fn validate<T: std::fmt::Debug>(result: Result<T, Error>) -> T {
    match result {
        Err(e) => match e {
            Error::HttpError(_)
            | Error::Io(_)
            | Error::MissingAPIKey
            | Error::MissingToken(_)
            | Error::Cancelled
            | Error::UploadSizeLimitExceeded(_, _)
            | Error::Failure(_)
            | Error::BadRequest(_)
            | Error::FieldClash(_)
            | Error::JsonDecodeError(_, _) => panic!("{}", e),
        },
        Ok(res) => return res
    }
}