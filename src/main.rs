use std::io::{self, Write};
use std::fs;
use std::path::Path;
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

    print!("Enter folder name: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input = input.trim().to_string();

    // Sets the base directory for youtube (Later want this to scan for the folder named "youtube")
    let folder_name = date + "_" + &input;
    let base_dir = "C:/Youtube";
    let main_folder_path = Path::new(base_dir).join(&folder_name);

    println!("Folder name: {}", folder_name);
    println!("Folder location: {}", main_folder_path.display());

    // Creates the folder path and internal folders
    fs::create_dir(&main_folder_path).expect("Failed to create main folder");
    let subfolders = ["A-roll", "Save", "Photoshop"];

    for folder in subfolders {
        let sub_path = main_folder_path.join(folder);
        fs::create_dir(&sub_path).expect("Failed to crate subfolder");
    }

    println!("Folder structure created successfully");
}
