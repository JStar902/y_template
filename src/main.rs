use std::io::{self, Write};
use std::{fs};
use std::path::{Path, PathBuf};
use chrono::Local;

/*
Purpose: This function creates the folder name by getting the date and project name from the user
Args: N/A
Return: folder_name (String)
*/
fn get_folder_name() -> String {
    // Gets date from chrono
    let date = Local::now().format("%Y-%m-%d").to_string(); // Gets date

    // Gathers and merges input folder name with date
    print!("Enter folder name: ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input = input.trim().to_string();
    let folder_name = date + "_" + &input;
    return folder_name;
}

/*
Purpose: Get the desired base file path for the new project folder to be placed
Args: N/A
Return: base_dir (Path)
*/
fn get_base_dir(start: &Path) -> Option<PathBuf> {
    let entries = fs::read_dir(start).ok()?;

    for entry in entries {
        let entry = entry.ok()?;
        let path = entry.path();

        if path.is_dir() {
            if let Some(name) = path.file_name() {
                if name == "Youtube" {
                    return Some(path);
                }
            }

            // if let Some(found) = get_base_dir(&path){
            //     return Some(found);
            // }
        }
    }

    None
}

/*
Purpose: Creates a new folder in your desired folder with subfolders for dividing up your work flow.
Args: base_dir (String) - file location for desired folder
      folder_name (String) - inputted date_name string for folder name
Return: Error message if failed
*/
fn create_directory(base_dir: &Path, folder_name: &str) -> io::Result<()> {
    let main_folder_path = base_dir.join(folder_name);
    if main_folder_path.exists() {
        return Err(io::Error::new(io::ErrorKind::AlreadyExists, "Folder already exists"));
    }

    fs::create_dir(&main_folder_path)?;
    let subfolders = ["A-roll", "B-roll", "Save", "Photoshop"];

    for folder in subfolders {
        fs::create_dir(main_folder_path.join(folder))?;
    }
    Ok(())

}
fn main() {

    // Sets the base directory for youtube (Later want this to scan for the folder named "youtube")
    let folder_name = get_folder_name();
    let start_path = Path::new("C:/");
    let base_dir = match get_base_dir(start_path) {
        Some(path) => path,
        None => {
            eprintln!("Could not find Youtube folder");
            return;
        }
    };
    match create_directory(&base_dir, &folder_name) {
        Ok(_) => println!("Folder structure created successfully"),
        Err(e) => eprintln!("Failed to create folders: {}", e),
    }
}
