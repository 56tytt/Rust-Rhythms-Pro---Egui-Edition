// src/ui.rs

use eframe::egui;
use std::path::Path;

use crate::audio::{AudioEngine, PlayerState};
use crate::equalizer::{EqPreset, Equalizer};
use crate::playlist::Playlist;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Tab {
    Library,
    Player,
    Equalizer,
}

pub struct MusicPlayerApp {
    pub engine: AudioEngine,
    pub playlist: Playlist,
    pub progress: f32,
    pub duration: f64,
    pub track_name: String,
    pub current_tab: Tab,
    pub volume: f32,
    pub eq: Equalizer,
    pub tex_play: egui::TextureHandle,
    pub tex_pause: egui::TextureHandle,
    pub tex_next: egui::TextureHandle,
    pub tex_prev: egui::TextureHandle,
    pub show_about_window: bool,
    pub show_theme_window: bool,
    pub accent_color: egui::Color32, // צבע הדגשה (לדים, סליידרים)
    pub bg_color: egui::Color32,     // רקע ראשי
    pub panel_color: egui::Color32,  // רקע פאנלים ואקולייזר
    pub text_color: egui::Color32,   // צבע טקסט ראשי
    pub border_color: egui::Color32, // צבע מסגרות
}

impl MusicPlayerApp {
    pub fn new(ctx: &egui::Context) -> Self {
        let engine = AudioEngine::new_headless();
        let mut playlist = Playlist::new();
        // Load the icons from the assets folder
        // Make sure you have these files in your project directory: assets/play.png, etc.
        let tex_play = load_icon(ctx, "play", include_bytes!("../assets/play.png"));
        let tex_pause = load_icon(ctx, "pause", include_bytes!("../assets/pause.png"));
        let tex_next = load_icon(ctx, "next", include_bytes!("../assets/next.png"));
        let tex_prev = load_icon(ctx, "prev", include_bytes!("../assets/prev.png"));

        // --- טעינת הפונט המותאם אישית ---
        let mut fonts = egui::FontDefinitions::default();
        // טוענים את הקובץ מהתיקייה assets
        fonts.font_data.insert(
            "my_custom_font".to_owned(),
            egui::FontData::from_static(include_bytes!("../assets/rb.ttf")).into(),
        );
        // מגדירים אותו כפונט הראשון שירוץ (גם לטקסט רגיל וגם לקוד)
        fonts
            .families
            .entry(egui::FontFamily::Proportional)
            .or_default()
            .insert(0, "my_custom_font".to_owned());
        fonts
            .families
            .entry(egui::FontFamily::Monospace)
            .or_default()
            .insert(0, "my_custom_font".to_owned());
        ctx.set_fonts(fonts);
        // -------------------------------

        let mut app = Self {
            engine,
            playlist,
            progress: 0.0,
            duration: 0.0,
            track_name: String::from("No track loaded"),
            current_tab: Tab::Player,
            volume: 1.0,
            eq: Equalizer::new(),
            accent_color: egui::Color32::from_rgb(0, 150, 255),
            bg_color: egui::Color32::from_rgb(12, 12, 15),
            panel_color: egui::Color32::from_rgb(20, 20, 25),
            text_color: egui::Color32::from_gray(220),
            border_color: egui::Color32::from_gray(50),
            tex_play,
            tex_pause,
            tex_next,
            tex_prev,
            show_about_window: false,
            show_theme_window: false,
        };

        app.load_theme(); // עכשיו זה בחוץ ובמקום הנכון!
        app
    }

    // Helper method to scan folders recursively
    fn scan_folder_recursive(&mut self, path: &Path) {
        if let Ok(entries) = std::fs::read_dir(path) {
            for entry in entries.flatten() {
                let p = entry.path();
                if p.is_dir() {
                    self.scan_folder_recursive(&p);
                } else if let Some(ext) = p.extension() {
                    let ext_str = ext.to_string_lossy().to_lowercase();
                    if ["mp3", "wav", "ogg", "flac", "m4a"].contains(&ext_str.as_str()) {
                        let path_str = p.to_string_lossy().to_string();
                        if !self.playlist.items.contains(&path_str) {
                            self.playlist.add(path_str);
                        }
                    }
                }
            }
        }
    }

    // Helper method to load a track
    fn load_and_play(&mut self, index: usize) {
        if let Some(song_path) = self.playlist.select(index) {
            self.engine.load(&song_path);
            self.engine.play();
            if let Some(name) = Path::new(&song_path).file_stem() {
                self.track_name = name.to_string_lossy().to_string();
            }
        }
    }

    // --- פונקציות שמירה וטעינה של העיצוב ---
    pub fn save_theme(&self) {
        let content = format!(
            "{},{},{}\n{},{},{}\n{},{},{}\n{},{},{}\n{},{},{}",
            self.bg_color.r(),
            self.bg_color.g(),
            self.bg_color.b(),
            self.panel_color.r(),
            self.panel_color.g(),
            self.panel_color.b(),
            self.accent_color.r(),
            self.accent_color.g(),
            self.accent_color.b(),
            self.text_color.r(),
            self.text_color.g(),
            self.text_color.b(),
            self.border_color.r(),
            self.border_color.g(),
            self.border_color.b()
        );
        let _ = std::fs::write("rust_rhythms_theme.cfg", content); // שומר לקובץ מקומי קטנטן
    }

    pub fn load_theme(&mut self) {
        if let Ok(content) = std::fs::read_to_string("rust_rhythms_theme.cfg") {
            let lines: Vec<&str> = content.lines().collect();
            if lines.len() >= 5 {
                self.bg_color = parse_color(lines[0], self.bg_color);
                self.panel_color = parse_color(lines[1], self.panel_color);
                self.accent_color = parse_color(lines[2], self.accent_color);
                self.text_color = parse_color(lines[3], self.text_color);
                self.border_color = parse_color(lines[4], self.border_color);
            }
        }
    }
}

// Implement the eframe::App trait
impl eframe::App for MusicPlayerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // --- 1. עדכון מנוע האודיו וחישוב התקדמות ---
        // עדכון מנוע האודיו ובדיקה אם השיר הסתיים
        let song_finished = self.engine.update();
        let mut visuals = egui::Visuals::dark();
        visuals.panel_fill = self.bg_color;
        visuals.window_fill = self.panel_color;
        visuals.override_text_color = Some(self.text_color);
        visuals.widgets.noninteractive.bg_stroke.color = self.border_color;
        ctx.set_visuals(visuals);

        // מעבר שיר אוטומטי
        if song_finished {
            if let Some(_) = self.playlist.next() {
                let idx = self.playlist.current_index.unwrap();
                self.load_and_play(idx);
            }
        }

        // עדכון משך השיר ופס ההתקדמות (חשוב כדי שהסליידר יזוז!)
        self.duration = self.engine.current_duration;

        if self.duration > 0.0 {
            self.progress = ((self.engine.current_position / self.duration) as f32) * 100.0;
        } else {
            self.progress = 0.0;
        }

        // בקשת ריענון רציפה עבור הויזואליזר ופס ההתקדמות
        ctx.request_repaint();
        // --- מקשי קיצור במקלדת (Hotkeys) ---
        // רווח (Space) = Play / Pause
        if ctx.input(|i| i.key_pressed(egui::Key::Space)) {
            if self.engine.current_state == PlayerState::Playing {
                self.engine.pause();
            } else {
                self.engine.play();
            }
        }
        // חץ למעלה = הגברת ווליום
        if ctx.input(|i| i.key_pressed(egui::Key::ArrowUp)) {
            self.volume = (self.volume + 0.05).clamp(0.0, 1.0);
            self.engine.set_volume(self.volume);
        }

        // חץ למטה = הנמכת ווליום
        if ctx.input(|i| i.key_pressed(egui::Key::ArrowDown)) {
            self.volume = (self.volume - 0.05).clamp(0.0, 1.0);
            self.engine.set_volume(self.volume);
        }

        // חץ ימינה = שיר הבא
        if ctx.input(|i| i.key_pressed(egui::Key::ArrowRight)) {
            if let Some(_) = self.playlist.next() {
                let idx = self.playlist.current_index.unwrap();
                self.load_and_play(idx);
            }
        }

        // חץ שמאלה = שיר קודם
        if ctx.input(|i| i.key_pressed(egui::Key::ArrowLeft)) {
            if let Some(_) = self.playlist.previous() {
                let idx = self.playlist.current_index.unwrap();
                self.load_and_play(idx);
            }
        }

        // --- 2. תפריט עליון (Menu Bar) ---
        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            egui::MenuBar::new().ui(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("📂 Load Playlist...").clicked() {
                        if let Some(path) = rfd::FileDialog::new()
                            .add_filter("Playlist", &["m3u"])
                            .pick_file()
                        {
                            if let Ok(new_playlist) = Playlist::load_m3u(&path) {
                                self.playlist = new_playlist;
                                ui.close();
                            }
                        }
                    }
                    if ui.button("💾 Save Playlist...").clicked() {
                        if let Some(path) = rfd::FileDialog::new()
                            .add_filter("Playlist", &["m3u"])
                            .save_file()
                        {
                            let _ = self.playlist.save_m3u(&path);
                            ui.close();
                        }
                    }
                    ui.separator();
                    if ui.button("➕ Add Folder...").clicked() {
                        if let Some(folder_path) = rfd::FileDialog::new().pick_folder() {
                            self.scan_folder_recursive(&folder_path);
                            ui.close();
                        }
                    }
                    ui.separator();
                    if ui.button("❌ Exit").clicked() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });

                ui.menu_button("View", |ui| {
                    if ui.button("Toggle Dark/Light Mode").clicked() {
                        let is_dark = ctx.style().visuals.dark_mode;
                        ctx.set_visuals(if is_dark {
                            egui::Visuals::light()
                        } else {
                            egui::Visuals::dark()
                        });
                        ui.close();
                    }

                    ui.menu_button("🎨 Theme", |ui| {
                        ui.label(
                            egui::RichText::new("Premium Presets:")
                                .strong()
                                .color(egui::Color32::from_gray(180)),
                        );
                        ui.separator();

                        // פריסטים מוכנים
                        let themes = [
                            ("Spotify Green", egui::Color32::from_rgb(30, 215, 96)),
                            ("Apple Pink", egui::Color32::from_rgb(250, 45, 85)),
                            ("Pro Blue", egui::Color32::from_rgb(0, 150, 255)),
                            ("Neon Purple", egui::Color32::from_rgb(180, 50, 255)),
                            ("Gold Premium", egui::Color32::from_rgb(255, 180, 50)),
                            ("Cyberpunk Red", egui::Color32::from_rgb(255, 0, 60)),
                        ];

                        for (name, color) in themes {
                            ui.horizontal(|ui| {
                                // מצייר ריבוע צבע קטן ליד השם
                                let (rect, _response) = ui.allocate_exact_size(
                                    egui::vec2(14.0, 14.0),
                                    egui::Sense::hover(),
                                );
                                ui.painter()
                                    .rect_filled(rect, egui::CornerRadius::same(3), color);

                                if ui.button(name).clicked() {
                                    self.accent_color = color;
                                    ui.close();
                                }
                            });
                        }

                        ui.add_space(10.0);

                        // בורר צבעים חופשי (Color Picker) מובנה של Egui
                        ui.label(
                            egui::RichText::new("Custom Color:")
                                .strong()
                                .color(egui::Color32::from_gray(180)),
                        );
                        ui.separator();

                        if ui.button("🖌 Open Color Picker...").clicked() {
                            self.show_theme_window = true;
                            ui.close(); // סוגר את התפריט בצורה מסודרת
                        }
                    });
                });

                // --- תפריט אודות (About) ---
                ui.menu_button("ℹ About", |ui| {
                    if ui.button("👨‍💻 About the Developer").clicked() {
                        self.show_about_window = true;
                        ui.close();
                    }
                });
            });
        });

        // --- 3. פאנל בקרה תחתון מאוחד (Unified Bottom Controls) ---
        // קריאה למודל החדש ב-bottom_panel.rs שמטפל גם בסטטוס וגם בכפתורים
        if let Some(action) = crate::bottom_panel::draw_bottom_controls(
            ctx,
            &mut self.engine,
            self.accent_color,
            self.duration,
            self.progress,
            &mut self.volume,
            &self.tex_play,
            &self.tex_pause,
            &self.tex_next,
            &self.tex_prev,
        ) {
            match action.as_str() {
                "NEXT" => {
                    if let Some(_) = self.playlist.next() {
                        let idx = self.playlist.current_index.unwrap();
                        self.load_and_play(idx);
                    }
                }
                "PREV" => {
                    if let Some(_) = self.playlist.previous() {
                        let idx = self.playlist.current_index.unwrap();
                        self.load_and_play(idx);
                    }
                }
                _ => {}
            }
        }

        // --- 4. פאנל מרכזי (Tabs Content) ---
        egui::CentralPanel::default().show(ctx, |ui| {
            // בחירת טאב (Tab Selector)
            ui.horizontal(|ui| {
                ui.selectable_value(&mut self.current_tab, Tab::Library, "📚 Library");
                ui.selectable_value(&mut self.current_tab, Tab::Player, "🎵 Player");
                ui.selectable_value(&mut self.current_tab, Tab::Equalizer, "🎛 Equalizer");
            });
            ui.separator();

            match self.current_tab {
                Tab::Library => {
                    ui.add_space(10.0);

                    // כותרת יוקרתית עם כמות השירים בצד ימין
                    ui.horizontal(|ui| {
                        ui.heading(egui::RichText::new("My Library").strong().size(28.0));
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            let count = self.playlist.items.len();
                            ui.label(
                                egui::RichText::new(format!("{} Tracks", count))
                                    .color(egui::Color32::from_gray(120))
                                    .size(14.0),
                            );
                        });
                    });

                    ui.add_space(15.0);
                    ui.separator();
                    ui.add_space(10.0);

                    let mut song_to_play = None;

                    if self.playlist.items.is_empty() {
                        // מצב "ריק" (Empty State) מרשים למרכז המסך
                        ui.vertical_centered(|ui| {
                            ui.add_space(80.0);
                            ui.label(egui::RichText::new("📭").size(60.0));
                            ui.add_space(20.0);
                            ui.label(
                                egui::RichText::new("Your Library is Empty")
                                    .size(22.0)
                                    .strong()
                                    .color(egui::Color32::from_gray(200)),
                            );
                            ui.add_space(10.0);
                            ui.label(
                                egui::RichText::new(
                                    "Go to File -> Add Folder to bring in the music.",
                                )
                                .size(15.0)
                                .color(egui::Color32::from_gray(150)),
                            );
                        });
                    } else {
                        // רשימת שירים מעוצבת אישית (Custom Drawn Rows)
                        egui::ScrollArea::vertical()
                            .auto_shrink([false; 2]) // מכריח את הגלילה לתפוס את כל הגובה
                            .show(ui, |ui| {
                                for (i, path_str) in self.playlist.items.iter().enumerate() {
                                    let file_name = std::path::Path::new(path_str)
                                        .file_stem()
                                        .map(|s| s.to_string_lossy().to_string())
                                        .unwrap_or_else(|| format!("Track {}", i + 1));

                                    let is_current = self.playlist.current_index == Some(i);

                                    // 1. מקצים שורה מרווחת בגובה 45 פיקסלים שחותכת את המסך
                                    let row_height = 45.0;
                                    let (rect, response) = ui.allocate_exact_size(
                                        egui::vec2(ui.available_width(), row_height),
                                        egui::Sense::click(),
                                    );

                                    // 2. רקע מותאם: אפקט Hover או הדגשת השיר הנוכחי
                                    let is_hovered = response.hovered();
                                    if is_hovered || is_current {
                                        let bg_color = if is_current {
                                            self.accent_color.linear_multiply(0.15) // רקע זוהר וחלש לשיר המתנגן
                                        } else {
                                            egui::Color32::from_white_alpha(10) // לבן שקוף מאוד במעבר עכבר
                                        };
                                        ui.painter().rect_filled(
                                            rect,
                                            egui::CornerRadius::same(6),
                                            bg_color,
                                        );
                                    }

                                    // 3. צבע הטקסט דינמי
                                    let text_color = if is_current {
                                        self.accent_color
                                    } else if is_hovered {
                                        egui::Color32::WHITE
                                    } else {
                                        egui::Color32::from_gray(180)
                                    };

                                    // 4. בניית התוכן (מספר, אייקון, שם)
                                    let icon = if is_current { "🔊" } else { "🎵" };
                                    let track_num = format!("{:02}", i + 1);

                                    let text_pos = rect.min + egui::vec2(15.0, 14.0); // פדינג פנימי מדויק

                                    // מציירים את הטקסט בעצמנו!
                                    ui.painter().text(
                                        text_pos,
                                        egui::Align2::LEFT_TOP,
                                        format!("{}    {}    {}", track_num, icon, file_name),
                                        egui::FontId::proportional(15.0),
                                        text_color,
                                    );

                                    // טיפול בלחיצה
                                    if response.clicked() {
                                        song_to_play = Some(i);
                                    }
                                }
                            });
                    }

                    // אם לחצנו על שיר, נפעיל אותו
                    if let Some(idx) = song_to_play {
                        self.load_and_play(idx);
                    }
                }
                Tab::Player => {
                    ui.vertical_centered(|ui| {
                        ui.add_space(50.0);
                        ui.heading(
                            egui::RichText::new(&self.track_name)
                                .size(32.0)
                                .strong()
                                .color(self.accent_color),
                        );
                        ui.add_space(10.0);
                        ui.label(
                            egui::RichText::new(format!("Status: {:?}", self.engine.current_state))
                                .weak(),
                        );
                        ui.add_space(60.0);

                        // ויזואליזר בסגנון Poweramp
                        let canvas_width = ui.available_width() * 0.85;
                        let canvas_height = 250.0;
                        let (response, painter) = ui.allocate_painter(
                            egui::Vec2::new(canvas_width, canvas_height),
                            egui::Sense::hover(),
                        );

                        if let Ok(data) = self.engine.spectrum_data.lock() {
                            if !data.is_empty() {
                                let rect = response.rect;
                                let num_bars = data.len() as f32;
                                let bar_width = rect.width() / num_bars;
                                let gap = 4.0;
                                let actual_bar_width = bar_width - gap;

                                for (i, &db_value) in data.iter().enumerate() {
                                    let height_factor = ((db_value + 60.0) / 60.0).clamp(0.02, 1.0);
                                    let bar_height = height_factor * rect.height();
                                    let x = rect.min.x + (i as f32) * bar_width + gap / 2.0;
                                    let y = rect.max.y - bar_height;

                                    // רקע העמודה
                                    painter.rect_filled(
                                        egui::Rect::from_min_max(
                                            egui::pos2(x, rect.min.y),
                                            egui::pos2(x + actual_bar_width, rect.max.y),
                                        ),
                                        egui::CornerRadius::same(2),
                                        egui::Color32::from_white_alpha(5),
                                    );
                                    // עמודת הסאונד
                                    painter.rect_filled(
                                        egui::Rect::from_min_max(
                                            egui::pos2(x, y),
                                            egui::pos2(x + actual_bar_width, rect.max.y),
                                        ),
                                        egui::CornerRadius::same(2),
                                        self.accent_color.linear_multiply(height_factor + 0.3),
                                    );
                                    // ה-Peak (הכובע הלבן)
                                    painter.rect_filled(
                                        egui::Rect::from_min_max(
                                            egui::pos2(x, y - 6.0),
                                            egui::pos2(x + actual_bar_width, y - 2.0),
                                        ),
                                        egui::CornerRadius::same(1),
                                        egui::Color32::WHITE,
                                    );
                                }
                            } else {
                                ui.label(egui::RichText::new("Waiting for audio data...").weak());
                            }
                        }
                    });
                }

                Tab::Equalizer => {
                    ui.heading("DSP Settings");
                    ui.add_space(20.0);
                    self.eq.show(
                        ui,
                        &mut self.engine,
                        &mut self.volume,
                        self.accent_color,
                        self.panel_color,
                    );
                }
            }
        });

        // --- ציור חלון ה"אודות" אם הוא פתוח ---
        if self.show_about_window {
            egui::Window::new("About Rust Rhythms")
                .collapsible(false)
                .resizable(false)
                .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0]) // שם את החלון בדיוק באמצע המסך
                .show(ctx, |ui| {
                    ui.add_space(10.0);
                    ui.vertical_centered(|ui| {
                        ui.heading(
                            egui::RichText::new("Rust Rhythms Pro")
                                .size(24.0)
                                .strong()
                                .color(self.accent_color),
                        );
                        ui.add_space(5.0);
                        ui.label(egui::RichText::new("Version 2.5 - Egui Edition").weak());

                        ui.add_space(20.0);

                        // כאן אתה כותב את השם שלך!
                        ui.label(
                            egui::RichText::new("Powerful, innovative audio engine®️").size(16.0),
                        );
                        ui.label("A high-performance audio engine built in Rust.");
                        ui.add_space(15.0);
                        ui.label(
                            egui::RichText::new("Shay Kadosh Software Engineering©️ Ashkelon")
                                .size(14.0),
                        );
                        ui.add_space(15.0);
                        ui.label("With my code partner,dear Jimmy Henderson ");

                        ui.add_space(20.0);
                        ui.separator();
                        ui.add_space(10.0);

                        if ui.button(egui::RichText::new("Close").size(16.0)).clicked() {
                            self.show_about_window = false;
                        }
                        ui.add_space(10.0);
                    });
                });
        }

        // --- חלון בחירת צבע חופשי (משוחרר מחסימות!) ---
        if self.show_theme_window {
            egui::Window::new("🎨 PRO Theme Studio")
                .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
                .show(ctx, |ui| {
                    ui.vertical(|ui| {
                        ui.label(egui::RichText::new("Interface Customization").strong());
                        ui.separator();

                        egui::Grid::new("colors")
                            .spacing([20.0, 10.0])
                            .show(ui, |ui| {
                                ui.label("Main Background:");
                                ui.color_edit_button_srgba(&mut self.bg_color);
                                ui.end_row();

                                ui.label("Panel Background:");
                                ui.color_edit_button_srgba(&mut self.panel_color);
                                ui.end_row();

                                ui.label("Accent Color:");
                                ui.color_edit_button_srgba(&mut self.accent_color);
                                ui.end_row();

                                ui.label("Primary Text:");
                                ui.color_edit_button_srgba(&mut self.text_color);
                                ui.end_row();

                                ui.label("Borders/Lines:");
                                ui.color_edit_button_srgba(&mut self.border_color);
                                ui.end_row();
                            });

                        ui.add_space(15.0);
                        if ui.button("Close & Save Theme").clicked() {
                            self.show_theme_window = false;
                        }
                    });
                });
        }
    }
}
// Helper formatting function-----------------------------------------------------------------------------------------
fn format_time(seconds: f64) -> String {
    let mins = (seconds / 60.0).floor() as u32;
    let secs = (seconds % 60.0) as u32;
    format!("{:02}:{:02}", mins, secs)
}

// Helper function to load PNG/JPG bytes into an EGUI TextureHandle
fn load_icon(ctx: &egui::Context, name: &str, bytes: &[u8]) -> egui::TextureHandle {
    let image = image::load_from_memory(bytes)
        .expect("Failed to load image from assets folder")
        .to_rgba8();
    let size = [image.width() as _, image.height() as _];
    let pixels = image.as_flat_samples();

    let color_image = egui::ColorImage::from_rgba_unmultiplied(size, pixels.as_slice());

    ctx.load_texture(
        name,
        color_image,
        egui::TextureOptions::LINEAR, // Smooth scaling
    )
}

// Helper function to parse colors from config file
fn parse_color(line: &str, default: egui::Color32) -> egui::Color32 {
    let parts: Vec<&str> = line.split(',').collect();
    if parts.len() == 3 {
        if let (Ok(r), Ok(g), Ok(b)) = (
            parts[0].trim().parse(),
            parts[1].trim().parse(),
            parts[2].trim().parse(),
        ) {
            return egui::Color32::from_rgb(r, g, b);
        }
    }
    default
}
