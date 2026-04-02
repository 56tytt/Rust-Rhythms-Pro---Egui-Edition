// src/equalizer.rs
use crate::audio::{ AudioEngine };
use eframe::egui;

#[derive(Debug, Clone, PartialEq)]
pub enum EqPreset {
    Custom,
    Flat,
    BassBoost,
    VocalClear,
    Electronic,
}

pub struct Equalizer {
    pub bands: [f32; 10],
    pub bass_boost: f32,
    pub spatial_audio: bool,
    pub active_preset: EqPreset,
}

impl Equalizer {
    pub fn new() -> Self {
        Self {
            bands: [4.0, 3.0, 0.0, -2.0, -1.0, 1.0, 3.0, 4.0, 4.0, 5.0],
            bass_boost: 0.3,
            spatial_audio: false,
            active_preset: EqPreset::Electronic,
        }
    }

    pub fn apply_preset(&mut self, preset: EqPreset) {
        self.active_preset = preset.clone();
        match preset {
            EqPreset::Flat => {
                self.bands = [0.0; 10];
                self.bass_boost = 0.0;
            }
            EqPreset::BassBoost => {
                self.bands = [5.0, 4.0, 2.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 2.0];
                self.bass_boost = 0.5;
            }
            EqPreset::VocalClear => {
                self.bands = [-2.0, -1.0, 0.0, 2.0, 4.0, 4.0, 2.0, 0.0, -1.0, -2.0];
                self.bass_boost = 0.0;
            }
            EqPreset::Electronic => {
                self.bands = [4.0, 3.0, 0.0, -2.0, -1.0, 1.0, 3.0, 4.0, 4.0, 5.0];
                self.bass_boost = 0.3;
            }
            EqPreset::Custom => {}
        }
    }

    pub fn show(&mut self, ui: &mut egui::Ui, engine: &mut AudioEngine, volume: &mut f32) {
        let mut eq_changed = false;

        ui.vertical(|ui| {
            ui.add_space(20.0);

            // --- Presets Section ---
            ui.horizontal(|ui| {
                ui.label(egui::RichText::new("Presets:").strong());
                ui.add_space(10.0);

                let presets = [
                    (EqPreset::Flat, "Flat"),
                    (EqPreset::BassBoost, "Bass Boost"),
                    (EqPreset::VocalClear, "Vocal Clear"),
                    (EqPreset::Electronic, "Electronic"),
                ];

                for (preset, label) in presets {
                    let is_selected = self.active_preset == preset;
                    let text_color = if is_selected {
                        egui::Color32::WHITE
                    } else {
                        ui.style().visuals.text_color()
                    };

                    let bg_color = if is_selected {
                        egui::Color32::from_rgb(0, 150, 255)
                    } else {
                        ui.style().visuals.widgets.noninteractive.bg_fill
                    };

                    let button_text = egui::RichText::new(label).color(text_color);

                    if ui.add(egui::Button::new(button_text).fill(bg_color)).clicked() {
                        self.apply_preset(preset);
                        eq_changed = true;
                    }
                }
            });

            ui.add_space(30.0);

            // --- EQ Sliders ---
            egui::Frame::group(ui.style()).show(ui, |ui| {
                ui.add_space(10.0);
                let freqs = ["32", "64", "125", "250", "500", "1K", "2K", "4K", "8K", "16K"];

                ui.columns(10, |columns| {
                    for i in 0..10 {
                        columns[i].vertical_centered(|ui| {
                            ui.style_mut().visuals.selection.bg_fill = self.bands_color(i);

                            // מחקנו את התווית הכפולה, עכשיו רק הסליידר יציג את המספר
                            let slider = egui::Slider
                                ::new(&mut self.bands[i], -12.0..=12.0)
                                .vertical()
                                .show_value(true); // עכשיו זה נכון

                            if ui.add_sized([30.0, 150.0], slider).changed() {
                                self.active_preset = EqPreset::Custom;
                                eq_changed = true;
                            }

                            ui.add_space(5.0);
                            ui.label(freqs[i]);

                            self.draw_peak_cap(ui, i);
                        });
                    }
                });
                ui.add_space(10.0);
            });

            ui.add_space(30.0);

            // --- Master Controls ---
            egui::Frame::group(ui.style()).show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.vertical(|ui| {
                        ui.label("Bass Boost:");
                        if ui.add(egui::Slider::new(&mut self.bass_boost, 0.0..=1.0)).changed() {
                            self.active_preset = EqPreset::Custom;
                            eq_changed = true;
                        }
                    });

                    ui.add_space(50.0);

                    // מחקתי מפה את ה-Master Volume כי יש לנו את הווליום בבר התחתון הגלובלי!
                    // זה מונע את הכפילות שראית בתמונה.

                    if ui.toggle_value(&mut self.spatial_audio, "🎧 3D Spatial Audio").changed() {
                        engine.set_spatial_audio(self.spatial_audio);
                    }
                });
            });

            // מחקנו לגמרי את שורת ה-draw_status_bar! הבר הגלובלי יעשה את העבודה.
        });

        if eq_changed {
            engine.set_equalizer(&self.bands, self.bass_boost);
        }
    }

    fn bands_color(&self, index: usize) -> egui::Color32 {
        if index < 5 {
            egui::Color32::from_rgb(0, 255, 255)
        } else {
            egui::Color32::from_rgb(255, 50, 50)
        }
    }

    fn draw_peak_cap(&self, ui: &mut egui::Ui, index: usize) {
        let desired_size = egui::Vec2::new(30.0, 5.0);
        let (rect, _response) = ui.allocate_at_least(desired_size, egui::Sense::hover());

        let val = self.bands[index];
        let percent = (val + 12.0) / 24.0;
        let y_pos = rect.min.y + rect.height() * (1.0 - percent);

        ui.painter().rect_filled(
            egui::Rect::from_center_size(
                egui::Pos2::new(rect.center().x, y_pos),
                egui::Vec2::new(desired_size.x * 0.8, desired_size.y * 0.6)
            ),
            egui::CornerRadius::same(1),
            egui::Color32::WHITE
        );
    }
}
