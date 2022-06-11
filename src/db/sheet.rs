use super::*;

extern crate google_sheets4 as sheets4;
use sheets4::{Sheets, oauth2, hyper, hyper_rustls};
use sheets4::Error;
use async_trait::async_trait;

const SPREADSHEET_ID: &str = "103Y8_w8Wu-9tYM0lsrdXSl7yysomt6LT1TzKPNA7VGQ";

pub struct SheetDatabase {
    hub: Sheets,
}

// It is kinda inefficient to create a new connection each time but that's what happens when you are chained to using google sheets as a "database".

impl SheetDatabase {
    pub async fn new() -> SheetDatabase {
        let secret = oauth2::read_application_secret("credentials.json").await.expect("Could not read secret.");
        let auth = oauth2::InstalledFlowAuthenticator::builder(
            secret,
            oauth2::InstalledFlowReturnMethod::HTTPRedirect,
        ) .persist_tokens_to_disk("tokencache.json").build().await.unwrap();

        let hub = Sheets::new(hyper::Client::builder().build(
            hyper_rustls::HttpsConnectorBuilder::new().with_native_roots()
                                                                .https_or_http().enable_http1().enable_http2().build()), 
            auth);

        SheetDatabase { hub }
    }
}

#[async_trait]
impl Database for SheetDatabase {
    async fn retrieve(&self) -> Result<Vec<Book>, ()> {
        let spreadsheet = self.hub.spreadsheets().get(SPREADSHEET_ID).include_grid_data(true).doit().await;
        match_result(spreadsheet);
     
        Ok(vec![])
    }

    async fn update(&self, new: Vec<Book>) -> Result<(), ()> {
        Ok(())
    }

    async fn log_update(&self, time: DateTime<Utc>, update: String) -> Result<(), ()> {
        Ok(())
    }
}

fn match_result<T: std::fmt::Debug>(result: Result<T, Error>) {
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
            | Error::JsonDecodeError(_, _) => println!("{}", e),
        },
        Ok(res) => println!("Success: {:?}", res),
    }
}