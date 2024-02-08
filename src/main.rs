use eframe::egui::{self, ComboBox};
use med::{Media, MediaType};
use strum::IntoEnumIterator;

fn main() -> Result<(), eframe::Error> {
    let mut options = eframe::NativeOptions::default();

    options.persist_window = true;

    eframe::run_native("MED", options, Box::new(|_cc| Box::<App>::default()))
}

#[derive(Default)]
struct Filter {
    media_type: Option<MediaType>,
    status: Option<med::Status>,
    state: Option<med::State>,
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

                if let Some(state) = self.filter.state {
                    if media.state != state {
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

                ComboBox::new("add_media_type", "")
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

                ComboBox::new("add_media_status", "")
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
                ui.label("State:");

                ComboBox::new("add_media_state", "")
                    .selected_text(format!("{}", self.tmp_media.state))
                    .show_ui(ui, |ui| {
                        for state in med::State::iter() {
                            ui.selectable_value(
                                &mut self.tmp_media.state,
                                state,
                                format!("{}", state),
                            );
                        }
                    });
            });

            ui.horizontal(|ui| {
                if ui.button("add").clicked() {
                    self.media.push(self.tmp_media.clone());
                    self.save_media().unwrap();
                    self.search_media();
                }

                if ui.button("reset").clicked() {
                    self.tmp_media = Media::default();
                }
            });
        });
    }
    fn search_bar(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label("Search:");

            // let text_edit = egui::TextEdit::singleline(&mut self.query);
            // ui.add_sized(ui.available_size(), text_edit);

            if ui.text_edit_singleline(&mut self.query).lost_focus() {
                self.search_media();
            };

            if ui.button("search").clicked() {
                self.search_media();
            }
        });

        // add a bit of space
        ui.add_space(2.0);

        ui.horizontal(|ui| {
            let selected_text = match self.filter.media_type {
                Some(media_type) => format!("{}", media_type),
                None => "Type".to_string(),
            };
            ComboBox::new("filter_media_type", "")
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
            ComboBox::new("filter_media_status", "")
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

            let selected_text = match self.filter.state {
                Some(state) => format!("{}", state),
                None => "State".to_string(),
            };
            ComboBox::new("filter_media_state", "")
                .selected_text(format!("{}", selected_text))
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut self.filter.state, None, "None");

                    for state in med::State::iter() {
                        ui.selectable_value(
                            &mut self.filter.state,
                            Some(state),
                            format!("{}", state),
                        );
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
            filter: Filter::default(),
        };

        app.load_media().unwrap();

        app
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            self.search_bar(ui);

            ui.separator();

            let media_list = if !self.search_results.is_empty() {
                &mut self.search_results
            } else {
                &mut self.media
            };
            let font_size = 25.0;

            egui::ScrollArea::vertical()
                // .auto_shrink(egui::Vec2b::new(true, true))
                .show(ui, |ui| {
                    ui.spacing_mut().item_spacing = egui::Vec2::new(0.0, 10.0);

                    for (idx, media) in media_list.iter_mut().enumerate() {
                        let mut frame = egui::Frame::default()
                            .rounding(6.0)
                            .inner_margin(egui::Vec2::new(10.0, 10.0))
                            .fill(egui::Color32::from_hex("#262626").unwrap())
                            .begin(ui);

                        frame
                            .content_ui
                            .label(egui::RichText::new(format!("{}", media.media_type)));

                        let title_text = egui::RichText::new(format!("{}", media.title))
                            .size(font_size)
                            .color(egui::Color32::WHITE);

                        frame.content_ui.label(title_text);

                        frame.content_ui.label(
                            egui::RichText::new(format!("Status: {}", media.status))
                                .size(font_size * 0.4),
                        );

                        // edit state and save on change
                        let combo_state = ComboBox::new(format!("media_state_{}", idx), "")
                            .selected_text(format!("{}", media.state))
                            .show_ui(&mut frame.content_ui, |ui| {
                                for state in med::State::iter() {
                                    ui.selectable_value(
                                        &mut media.state,
                                        state,
                                        format!("{}", state),
                                    );
                                }
                            });

                        if combo_state.response.changed() {
                            self.save_media().unwrap();
                        }

                        // let available_width = frame.content_ui.available_width();
                        // frame
                        //     .content_ui
                        //     .allocate_space(egui::Vec2::new(available_width * 1.0, 0.0));
                        frame.end(ui);
                    }
                });

            self.window_add_media(ctx);
        });
    }
}
