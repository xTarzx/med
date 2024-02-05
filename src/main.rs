use eframe::egui::{self, ComboBox};
use med::{Media, MediaType};
use strum::IntoEnumIterator;

fn main() -> Result<(), eframe::Error> {
    let mut options = eframe::NativeOptions::default();

    options.persist_window = true;

    eframe::run_native("MED", options, Box::new(|_cc| Box::<App>::default()))
}

struct Filter {
    media_type: Option<MediaType>,
    status: Option<med::Status>,
}

struct App {
    db_path: std::path::PathBuf,
    media: Vec<Media>,
    tmp_media: Media,

    query: String,
    search_results: Vec<Media>,
    filter: Filter,
}

impl App {
    fn load_media(&mut self) -> Result<(), std::io::Error> {
        let file = std::fs::File::open(&self.db_path);

        match file {
            Ok(file) => {
                self.media = serde_json::from_reader(file).unwrap_or_default();
                Ok(())
            }
            Err(err) => match err.kind() {
                std::io::ErrorKind::NotFound => self.save_media(),
                _ => return Err(err),
            },
        }
    }

    fn save_media(&self) -> Result<(), std::io::Error> {
        let file = std::fs::File::create(&self.db_path);

        match file {
            Ok(file) => {
                serde_json::to_writer(file, &self.media)?;
                Ok(())
            }
            Err(err) => Err(err),
        }
    }

    fn search_media(&mut self) {
        self.search_results = self
            .media
            .iter()
            .filter(|media| {
                if !self.query.is_empty() {
                    if !media.title.contains(&self.query) {
                        return false;
                    }
                }

                if let Some(media_type) = self.filter.media_type {
                    if media.media_type != media_type {
                        return false;
                    }
                }

                if let Some(status) = self.filter.status {
                    if media.status != status {
                        return false;
                    }
                }

                true
            })
            .cloned()
            .collect();
    }

    fn window_add_media(&mut self, ctx: &egui::Context) {
        egui::Window::new("Add Media").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("Title:");
                ui.text_edit_singleline(&mut self.tmp_media.title);
            });

            ui.horizontal(|ui| {
                ui.label("Type:");

                ComboBox::new("media_type", "")
                    .selected_text(format!("{}", self.tmp_media.media_type))
                    .show_ui(ui, |ui| {
                        for media_type in MediaType::iter() {
                            ui.selectable_value(
                                &mut self.tmp_media.media_type,
                                media_type,
                                format!("{}", media_type),
                            );
                        }
                    });
            });

            ui.horizontal(|ui| {
                ui.label("Release Date:");
                ui.add(egui_extras::DatePickerButton::new(
                    &mut self.tmp_media.release_date,
                ));
            });

            ui.horizontal(|ui| {
                ui.label("Status:");

                ComboBox::new("media_status", "")
                    .selected_text(format!("{}", self.tmp_media.status))
                    .show_ui(ui, |ui| {
                        for status in med::Status::iter() {
                            ui.selectable_value(
                                &mut self.tmp_media.status,
                                status,
                                format!("{}", status),
                            );
                        }
                    });
            });

            ui.horizontal(|ui| {
                if ui.button("add").clicked() {
                    self.media.push(self.tmp_media.clone());
                    self.save_media().unwrap();
                }

                if ui.button("reset").clicked() {
                    self.tmp_media = Media::default();
                }
            });
        });
    }
}

impl Default for App {
    fn default() -> Self {
        let db_path = std::env::current_exe()
            .unwrap()
            .parent()
            .unwrap()
            .join("med.db");

        let mut app = App {
            db_path,
            media: Vec::new(),
            tmp_media: Media::default(),

            query: String::new(),
            search_results: Vec::new(),
            filter: Filter {
                media_type: None,
                status: None,
            },
        };

        app.load_media().unwrap();

        app
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("Search:");
                if ui.text_edit_singleline(&mut self.query).lost_focus() {
                    self.search_media();
                };

                let selected_text = match self.filter.media_type {
                    Some(media_type) => format!("{}", media_type),
                    None => "Type".to_string(),
                };
                ComboBox::new("media_type", "")
                    .selected_text(selected_text)
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut self.filter.media_type, None, "None");

                        for media_type in MediaType::iter() {
                            ui.selectable_value(
                                &mut self.filter.media_type,
                                Some(media_type),
                                format!("{}", media_type),
                            );
                        }
                    });

                let selected_text = match self.filter.status {
                    Some(status) => format!("{}", status),
                    None => "Status".to_string(),
                };
                ComboBox::new("media_status", "")
                    .selected_text(selected_text)
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut self.filter.status, None, "None");

                        for status in med::Status::iter() {
                            ui.selectable_value(
                                &mut self.filter.status,
                                Some(status),
                                format!("{}", status),
                            );
                        }
                    });

                if ui.button("search").clicked() {
                    self.search_media();
                }
            });

            ui.separator();

            let media_list = if !self.search_results.is_empty() {
                &self.search_results
            } else {
                &self.media
            };

            for media in media_list {
                ui.horizontal(|ui| {
                    ui.label(format!("{}", media.title));
                    ui.label(format!("{}", media.media_type));
                    ui.label(format!("{}", media.release_date));
                    ui.label(format!("{}", media.status));
                });
            }

            self.window_add_media(ctx);
        });
    }
}
