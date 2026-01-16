#![windows_subsystem = "windows"]

use std::{fs, io}; // OS crate
use std::path::{Path, PathBuf}; // File path crate
// use std::process::Command; // Allows the opening of applications
use std::sync::{Arc, Mutex}; // Multitasking crate
use chrono::Local; // Gets local time information from computer
use eframe::egui; // Allows for GUI interface

enum ScanStatus {
    Idle,
    Scanning,
    Found(PathBuf),
    NotFound,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum ProjectType {
    None,
    Youtube,
    School,
}

/*
Purpose: Scans for desired base file path for the new project folder to be placed
Args: start (Path) - What main directory is searched
      target_path (String) - Name of folder being searched for
Return: base_dir (Path/None)
*/
fn get_base_dir(start: &Path, target_path: &str) -> Option<PathBuf> {
    let entries = fs::read_dir(start).ok()?; 

    for entry in entries {
        let entry = entry.ok()?;
        let path = entry.path();

        if path.is_dir() {
            if let Some(name) = path.file_name() {
                // If the selected folder is target_path (Not case sensitive) it is chosen as the main base folder
                if name.to_string_lossy().eq_ignore_ascii_case(target_path) {
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
      folder_name (String) - The new project's folder name in date_inputted name format
Return: Error message if failed
*/
fn create_directory(base_dir: &Path, folder_name: &str, subfolders: &[&str]) -> io::Result<PathBuf> {
    let main = base_dir.join(folder_name);

    if main.exists() {
        return Err(io::Error::new(io::ErrorKind::AlreadyExists, "Folder exists"));
    }

    // If there is no folder in the base_dir with folder_name, a new project foler is created
    fs::create_dir(&main)?;

    for sub in subfolders {
        fs::create_dir(main.join(sub))?; // Creates subfolders
    }

    if main.join("Save").exists(){
        // Creates a premiere save file
        let prproj_template = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("Assets").join("template.prproj");
        let project_name: Vec<&str> = folder_name.split("_").collect();
        let prproj_name = format!("{}.prproj", project_name[1]);
        let prproj = main.join("Save").join(prproj_name);
        fs::copy(prproj_template, prproj)?;

        // Creates a photoshop save file
        let psd_template = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("Assets").join("template.psd");
        let psd_name = format!("{}.psd", project_name[1]);
        let psd = main.join("Photoshop").join(psd_name);
        fs::copy(psd_template, psd)?;

        // Creates a photoshop save file
        let md_template = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("Assets").join("template.md");
        let prefix = project_name[1].get(..3).unwrap_or(&project_name[1]);
        let md_name = format!("{}_ideas.md", prefix);
        let md = main.join(md_name);
        fs::copy(md_template, md)?;
    }

    Ok(main)
}

#[derive(Default)]
struct MyApp {
    search_folder_name: String, // Name for base_path that will be searched for
    folder_name: String, // Name for new project folder
    status: String, // Text shown to explain what is currently happening

    base_path: Option<PathBuf>, // Path where new project folders are placed
    project_path: PathBuf, // Path to new project folder created
    project_type: ProjectType, // Indicates what type of project created

    scan_status: Arc<Mutex<ScanStatus>>, // The current state of the application
    pending_create: bool, // Indicator for if the base_path 
}

impl Default for ScanStatus {
    fn default() -> Self {
        ScanStatus::Idle
    }
}

impl Default for ProjectType {
    fn default() -> Self {
        ProjectType::None
    }
}

impl MyApp {
    // Purpose: Scans for base folder directory
    fn start_scan(&mut self) { 
        self.status = format!("Searching for {} folder",self.search_folder_name);

        let scan_status = Arc::clone(&self.scan_status);
        let target = self.search_folder_name.clone();

        *scan_status.lock().unwrap() = ScanStatus::Scanning;

        std::thread::spawn(move || {
            let found = get_base_dir(Path::new("C:/"), &target);
            
            let mut status = scan_status.lock().unwrap();
            *status = match found {
                Some(path) => ScanStatus::Found(path),
                None => ScanStatus::NotFound,
            }
        });
    }

    // Gets folder name and Project folder path
    fn create_project(&mut self) {
        if self.folder_name.trim().is_empty() {
            self.status = "Folder name cannot be empty".to_string();
            return;
        }

        if self.base_path.is_none() {
            self.pending_create = true;
            self.start_scan();
            return;
        }

        self.finish_create_project();
    }

    // Purpose: Creates new project folder
    fn finish_create_project(&mut self) {

        let subfolders: &[&str] = match self.project_type {
            ProjectType::Youtube => &["A-roll", "B-roll", "Save", "Photoshop"],
            ProjectType::School => &["HW", "Slides", "Exam"],
            ProjectType::None => &[],
        };

        let base_dir = self.base_path.as_ref().unwrap().clone();
        let date = Local::now().format("%Y-%m-%d");

        let final_name: String = if matches!(self.project_type, ProjectType::Youtube) {
            format!("{}_{}", date, self.folder_name.trim())            
        } else {
            format!("{}", self.folder_name.trim())
        };

        match create_directory(&base_dir, &final_name, subfolders) {
            Ok(created_path) =>{
                self.status = "Folder created successfully".to_string();
                self.project_path = created_path;
                self.pending_create = false;

                // let _ = Command::new(r"C:\Program Files\Adobe\Adobe Premiere Pro 2025.exe").spawn();
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

        // Sets functionallity for enter key based on if checking for base path or creating project
        if self.base_path.is_none(){
            if ctx.input(|i| i.key_pressed(egui::Key::Enter)) {
                self.start_scan();
            }
        } else {
            if ctx.input(|i| i.key_pressed(egui::Key::Enter)) {
                self.create_project();
            }
        }

        // Closes program if esc key is pressed
        if ctx.input(|i| i.key_pressed(egui::Key::Escape)) {
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
        }

        // 
        let scan_action = {
            let scan_status = self.scan_status.lock().unwrap();

            match &*scan_status {
                ScanStatus::Found(path) => Some(ScanStatus::Found(path.clone())),
                ScanStatus::NotFound => Some(ScanStatus::NotFound),
                _ => None,
            }
        };
        
        if let Some(action) = scan_action {
            match action {
                ScanStatus::Found(path) => {
                    self.base_path = Some(path);
                    self.status = "Base folder found".to_string();
                    *self.scan_status.lock().unwrap() = ScanStatus::Idle;

                    if self.pending_create {
                        self.finish_create_project();
                    }
                }

                ScanStatus::NotFound => {
                    self.status = format!("Folder '{}' not found in C:/",self.search_folder_name);
                    self.pending_create = false;
                    *self.scan_status.lock().unwrap() = ScanStatus::Idle;
                }
                _ => {}
            }
        }

        // Main GUI interface
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {

                ui.heading("Project Folder Creator");
                ui.add_space(10.0);

                
                if self.base_path.is_none() {
                    // If the base path is not selected, screen prompts to enter it
                    ui.label("Base folder name:");

                    ui.add(egui::TextEdit::singleline(&mut self.search_folder_name).hint_text("Enter base folder name"));

                    ui.add_space(10.0);

                    if ui.button("Search for Folder").clicked() {
                        if self.search_folder_name.trim().is_empty() {
                            self.status = "Search folder name cannot be empty".to_string();
                        }else{
                            self.start_scan();
                        }
                    }
                } else {
                    // Main menu prompting user to enter new project type and folder name 
                    ui.label("Project name:");

                    ui.add(egui::TextEdit::singleline(&mut self.folder_name).hint_text("Enter project name (no date needed)"));

                    ui.add_space(10.0);

                    let create_enabled = self.project_type != ProjectType::None;

                    ui.vertical_centered(|ui|{
                        // ui.horizontal(|ui| {
                        //     ui.radio_value(&mut self.project_type, ProjectType::Youtube, "Youtube");

                        //     if ui.add_enabled(create_enabled, egui::Button::new("Create Folder")).clicked() {
                        //         self.create_project();
                        //     }                            
                        // });

                        // ui.horizontal(|ui| {

                        //     ui.radio_value(&mut self.project_type, ProjectType::School, "School");
                        //     if ui.button("Reset Project Folder").clicked() {
                        //         self.project_path = PathBuf::new();
                        //         self.folder_name.clear();
                        //         self.base_path = None;
                        //         self.status = "Project folder reset".to_string();
                        //     }                            
                        // }); 

                        ui.group(|ui| {
                            // ui.horizontal_top(|ui| { 
                            //     ui.radio_value(&mut self.project_type, ProjectType::Youtube, "Youtube");

                            //     if ui.add_enabled(create_enabled, egui::Button::new("Create Folder")).clicked() {
                            //         self.create_project();
                            //     }                                
                            // });

                            ui.radio_value(&mut self.project_type, ProjectType::Youtube, "Youtube");

                            if ui.add_enabled(create_enabled, egui::Button::new("Create Folder")).clicked() {
                                self.create_project();
                            }

                            ui.end_row();

                            ui.radio_value(&mut self.project_type, ProjectType::School, "School");
                            if ui.button("Reset Project Folder").clicked() {
                                self.project_path = PathBuf::new();
                                self.folder_name.clear();
                                self.base_path = None;
                                self.status = "Project folder reset".to_string();
                            }   
                        });
                          
                    });

                    ui.add_space(10.0);

                    // ui.horizontal(|ui|{
                    //     ui.radio_value(&mut self.project_type, ProjectType::School, "School");
                    //     if ui.button("Reset Project Folder").clicked() {
                    //         self.project_path = PathBuf::new();
                    //         self.folder_name.clear();
                    //         self.base_path = None;
                    //         self.status = "Project folder reset".to_string();
                    //     }   
                    // }); 
     

                }

                if ui.button("Exit").clicked() {
                    ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                }

            });       

            ui.add_space(15.0);

            if matches!(*self.scan_status.lock().unwrap(), ScanStatus::Scanning) {
                ui.horizontal_centered(|ui| {
                    ui.spinner();
                    ui.label("Scanning for project folder...");
                });
            }

            ui.separator();

            if !self.status.is_empty() {
               ui.vertical_centered(|ui|{
                    ui.label(&self.status);
               });
            }

            ui.vertical_centered(|ui|{
                match &self.base_path {
                    Some(path) => {
                        ui.label(format!("Base path: {}", path.to_string_lossy().to_string()));
                        
                    } None => {
                        ui.label("No base path selected");
                    }
                }
            });

            if !self.project_path.to_string_lossy().to_string().is_empty() {
               ui.vertical_centered(|ui|{
                    ui.label(format!("Project path: {}", self.project_path.to_string_lossy().to_string()));
                    ctx.copy_text(self.project_path.to_string_lossy().to_string());
               });
            }
        });


    }
}

fn main() -> eframe::Result<()> {
    // let options = eframe::NativeOptions::default();
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([500.0, 250.0])
            .with_min_inner_size([500.0, 250.0])
            .with_transparent(true), // To have rounded corners we need transparency

        ..Default::default()
    };

    eframe::run_native(
        "Project Folder Creator",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    )
}
