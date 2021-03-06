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
        let result = self.hub.spreadsheets().values_get(SPREADSHEET_ID, "_Library!A2:I").doit().await;
        let values = match validate(result).1.values {
            Some(s) => s,
            None => return Ok(Vec::new())
        };

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
                        image: row.get(8).unwrap().to_string(),
                        using: students 
                    })
        }
     
        Ok(books)
    }

    async fn append(&self, new: Book) -> Result<(), ()> {
        let mut req = ValueRange::default();
        req.major_dimension = Some("ROWS".to_string());
        req.values = Some(vec![vec![new.id.to_string(), new.entered, new.title, new.author, new.genre, new.copies.to_string(), 
                                    (new.copies - new.using.len()).to_string(), serialize_using(new.using), new.image]]);
            
        self.hub.spreadsheets().values_append(req, SPREADSHEET_ID, "_Library!A2:I")
                         .value_input_option("USER_ENTERED").doit().await.unwrap();

        Ok(())
    }

    async fn update(&self, index: u64, new: Book) -> Result<(), ()> {
        let mut req = ValueRange::default();
        req.major_dimension = Some("ROWS".to_string());
        req.values = Some(vec![vec![new.id.to_string(), new.entered, new.title, new.author, new.genre, new.copies.to_string(), 
                                    (new.copies - new.using.len()).to_string(), serialize_using(new.using), new.image]]);
            
        self.hub.spreadsheets().values_update(req, SPREADSHEET_ID, format!("_Library!A{}:I{}", index + 2, index + 2).as_str())
                         .value_input_option("RAW").doit().await.unwrap();

        Ok(())
    }

    async fn log(&self, time: DateTime<Local>, update: String) -> Result<(), ()> {
        let mut req = ValueRange::default();
        req.major_dimension = Some("ROWS".to_string());
        req.values = Some(vec![vec![time.format("%m/%d/%y %H:%M:%S").to_string(), update]]);
            
        self.hub.spreadsheets().values_append(req, SPREADSHEET_ID, "_Record!A2:B")
                         .value_input_option("RAW").doit().await.unwrap();

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
        Ok(res) => {
            println!("{:?}\n", res);
            res
        }
    }
}

fn serialize_using(students: Vec<Student>) -> String {
    let mut string = String::new();
    for student in students {
        string.push_str(&student.name[..]);
        string.push_str(":");
        string.push_str(&student.email[..]);
        string.push_str(";");
    }

    string
}