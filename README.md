# Club Library

A backend application built in Rust to hold logic for the club library. Previously this was done purely through a google sheet, but this will enable more accessible methods to be built. Currently, only basic features such as adding, borrowing, and returning are being prioritized. The project will hopefully encompass all the functions that a true library has, such as holds and renewals, in the future.

## Installation

> To run the application with the club library you must have access to the boilerbookclub@gmail.com account. Otherwise, you will have to modify the spreadsheet ID used in `src/db/sheet.rs` and supply your own.

1. Install Rust through [rustup](https://rustup.rs)
2. Create a `credentials.json` file in the top level `club-library/` directory. Populate it with the following tokens:

## Usage

Once deployed, the api has several simple paths that will update the sheet. For example:

----
### Contributors

- Kai Tinkess

### Technology

- Rust
- [Axum](https://crates.io/crates/axum) is the API crate of choice.
- [sheets4](https://crates.io/crates/google-sheets4) is used to update the google sheet.
- Various other crates like Tokio and Serde






