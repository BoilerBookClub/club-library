# Club Library

A backend application built in Rust to hold logic for the Boiler Book Club library. This was previously done purely through a google sheet, but we would like more accessible methods to be built. Currently, only basic features such as adding, borrowing, and returning are being prioritized. The project will hopefully encompass all the functions that a real library has, such as holds and renewals, in the future.

## Installation

> To run the application with the club library you must have access to the boilerbookclub@gmail.com account. Otherwise, you will have to modify the spreadsheet ID used in `src/db/sheet.rs` and supply your own. You can find a template for the spreadsheet [here](https://docs.google.com/spreadsheets/d/1zr16x_9DHDppHz_tXc_1W8a4QVUJyUR2r_BO5be-beU/edit?usp=sharing). 

1. Install Rust through [rustup](https://rustup.rs)
2. Clone this repository
3. Create a `credentials.json` file in the top level `club-library/` directory. You can obtain the ID and secret from the google API console. Populate it with something like the following:
```json
{
    "installed": {
        "client_id": "your id",
        "client_secret": "your secret",
        "project_id": "your project id",
        "auth_uri": "https://accounts.google.com/o/oauth2/auth",
        "token_uri": "https://accounts.google.com/o/oauth2/token",
        "redirect_uris": ["urn:ietf:wg:oauth:2.0:oob", "http://localhost"]
    }
}
```
4. Run the program with `cargo run`

Please also note that the program requests the API token for you and caches it. As a result, the first time you run it you will have to manually approve it by copy pasting a link. These tokens do expire, so this will need to be repeated every so often.
## Usage

Once deployed, updating the sheet is easy. We have:
- `/`: A GET request to the root will return all books currently in the library.
- `/books`: A POST request to /books will add a new book and require a title, author, genre, name, and email.
- `/borrowing`: A POST request to /borrowing will require an id, name, and email.
- `/returning`: A POST request to /returnin will require an id, name, and email.

For example, you could do:
```
localhost:3000/books?title=Siege and Storm&author=Leigh Bardugo&genre=YA Fantasy&name=Kai Tinkess&email=kaitinkess@gmail.com
```

----
### Checklist
These do not have tracking issues or PRs yet, but are good places to take the project.
- [ ] Make status codes and errors more accurate
- [ ] Hard due dates
- [ ] Renewals
- [ ] Holds
- [ ] Email Reminders
- [ ] Removing books
### Contributors

- Kai Tinkess

### Technology

- Rust
- [Axum](https://crates.io/crates/axum) is the API crate of choice.
- [sheets4](https://crates.io/crates/google-sheets4) is used to update the google sheet.
- Various other crates like Tokio and Serde






