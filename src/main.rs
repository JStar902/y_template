#![windows_subsystem = "windows"]

use std::{fs, io}; // OS crate
use std::path::{Path, PathBuf}; // File path crate
//use std::process::Command; // Allows the opening of applications
use std::sync::{Arc, Mutex}; // Multitasking crate
use chrono::Local; // Gets local time information from computer
use eframe::egui; // Allows for GUI interface

/*
Purpose: Scans for desired base file path for the new project folder to be placed
Args: N/A
Return: base_dir (Path/None)
*/
fn get_base_dir(start: &Path) -> Option<PathBuf> {
    let entries = fs::read_dir(start).ok()?;

    for entry in entries {
        let entry = entry.ok()?;
        let path = entry.path();

        if path.is_dir() {
            if let Some(name) = path.file_name() {
                if name.to_string_lossy().eq_ignore_ascii_case("youtube") {
                    return Some(path);
                }
            }
        }
    }
    None
}

/*
Purpose: Creates a new project folder in a desired location with subfolders for dividing up your work flow.
Args: base_dir (Path) - file location for desired folder
      folder_name (String) - inputted date_name string for folder name
Return: Error message if failed
*/
fn create_directory(base_dir: &Path, folder_name: &str) -> io::Result<PathBuf> {
    let main = base_dir.join(folder_name);

    if main.exists() {
        return Err(io::Error::new(io::ErrorKind::AlreadyExists, "Folder exists"));
    }

    fs::create_dir(&main)?;
    for sub in ["A-roll", "B-roll", "Save", "Photoshop"] {
        fs::create_dir(main.join(sub))?;
    }

    Ok(main)
}

#[derive(Default)]
struct MyApp {
    folder_name: String,
    status: String,

    youtube_path: Option<PathBuf>,
    string_youtube_path: String,

    is_scanning: bool,
    scan_result: Arc<Mutex<Option<PathBuf>>>,
    pending_create: bool,
}

impl MyApp {
    // Purpose: Scans for Youtube project folder
    fn start_scan(&mut self) { 
        self.is_scanning = true;
        self.status = "Searching for project folder".to_string();

        let result = Arc::clone(&self.scan_result);

        std::thread::spawn(move || {
            let found = get_base_dir(Path::new("C:/"));
            *result.lock().unwrap() = found;
        });
    }

    // Gets folder name and Project folder path
    fn create_project(&mut self) {
        if self.folder_name.trim().is_empty() {
            self.status = "Folder name cannot be empty".to_string();
            return;
        }

        if self.youtube_path.is_none() {
            self.pending_create = true;
            self.start_scan();
            return;
        }

        self.finish_create_project();
    }

    // Purpose: Creates new project folder
    fn finish_create_project(&mut self) {

        let base_dir = self.youtube_path.as_ref().unwrap().clone();
        let date = Local::now().format("%Y-%m-%d");
        let final_name = format!("{}_{}", date, self.folder_name.trim());

        match create_directory(&base_dir, &final_name) {
            Ok(created_path) =>{
                self.status = "Folder created successfully".to_string();
                self.pending_create = false;
                self.string_youtube_path = created_path.to_string_lossy().to_string();

                //let _ = Command::new("explorer").arg("/select").arg(created_path.to_string_lossy().as_ref()).spawn();
                // let _ = Command::new(r"C:\Program Files\Adobe\Adobe Premiere Pro.exe").spawn();
            } 
            Err(e) => {
                self.status = format!("Error: {}", e);
                self.pending_create = false;
            }
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if ctx.input(|i| i.key_pressed(egui::Key::Enter)) {
            self.create_project();
        }

        if ctx.input(|i| i.key_pressed(egui::Key::Escape)) {
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
        }

        if self.is_scanning {
            let scan_result = {
                self.scan_result.lock().unwrap().take()
            };
            
            if let Some(found) = scan_result {
                self.youtube_path = Some(found);
                self.is_scanning = false;
                self.status = "Project folder found".to_string();

                if self.pending_create {
                    self.finish_create_project();
                }
            }
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| { ui.heading("Youtube Folder Creator"); ui.add_space(10.0);});

            ui.vertical_centered(|ui| {
                ui.label("Project name:");

                ui.add(egui::TextEdit::singleline(&mut self.folder_name).hint_text("Enter project name (no date needed)"));

                ui.add_space(10.0);

                if ui.button("Create Folder").clicked() {
                    self.create_project();
                }

                if ui.button("Exit").clicked() {
                    ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                } 
            });       

            ui.add_space(15.0);

            if self.is_scanning {
                ui.horizontal_centered(|ui| {
                    ui.spinner();
                    ui.label("Scanning for project folder");
                });
            }

            ui.separator();

            if !self.status.is_empty() {
               ui.vertical_centered(|ui|{
                    ui.label(&self.status);
                    ui.label(&self.string_youtube_path);
               });
            }
        });
    }
}

fn main() -> eframe::Result<()> {
    //let options = eframe::NativeOptions::default();
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([500.0, 250.0])
            .with_min_inner_size([500.0, 250.0])
            .with_transparent(true), // To have rounded corners we need transparency

        ..Default::default()
    };

    eframe::run_native(
        "Youtube Folder Creator",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    )
}
