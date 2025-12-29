use std::io::{self, Write};
use chrono::Local;

// fn get_user() {

//     let today = Local::now();

//     println!("Enter a name: ");

//     let mut input = String::new();


// }

fn main() {
    // Gets the date for Folder name
    let now = Local::now();
    let date = now.format("%Y-%m-%d").to_string();

    // Sets the base directory for youtube (Later want this to scan for the folder named "youtube")

    print!("Enter folder name: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input = input.trim().to_string();

    println!("File name will be: {}_{}", date, input);
}
