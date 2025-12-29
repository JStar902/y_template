use std::{fs, io};
use std::path::{Path, PathBuf};
use chrono::Local;
use eframe::egui;

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

            // if let Some(found) = get_base_dir(&path) {
            //     return Some(found);
            // }
        }
    }
    None
}

fn create_directory(base_dir: &Path, folder_name: &str) -> io::Result<()> {
    let main = base_dir.join(folder_name);

    if main.exists() {
        return Err(io::Error::new(io::ErrorKind::AlreadyExists, "Folder exists"));
    }

    fs::create_dir(&main)?;
    for sub in ["A-roll", "B-roll", "Save", "Photoshop"] {
        fs::create_dir(main.join(sub))?;
    }

    Ok(())
}

#[derive(Default)]
struct MyApp {
    folder_name: String,
    status: String,
}

impl MyApp {
    fn create_project(&mut self) {
        if self.folder_name.trim().is_empty() {
            self.status = "Folder name cannot be empty".to_string();
            return;
        }

        let date = Local::now().format("%Y-%m-%d");
        let final_name = format!("{}_{}", date, self.folder_name.trim());

        let start = Path::new("C:/");
        let base_dir = match get_base_dir(start) {
            Some(p) => p,
            None => {
                self.status = "Could not find Youtube folder".to_string();
                return;
            }
        };

        match create_directory(&base_dir, &final_name) {
            Ok(_) => self.status = "Folder created successfully".to_string(),
            Err(e) => self.status = format!("Error: {}", e),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
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
            ui.separator();
            if !self.status.is_empty() {
               ui.label(&self.status);
            }
        });
    }
}

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Youtube Folder Creator",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    )
}
