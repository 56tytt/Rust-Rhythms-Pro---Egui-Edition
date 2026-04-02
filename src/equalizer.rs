// src/equalizer.rs
use crate::audio::AudioEngine;
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

    pub fn show(
        &mut self,
        ui: &mut egui::Ui,
        engine: &mut AudioEngine,
        _volume: &mut f32,
        accent_color: egui::Color32,
        panel_bg: egui::Color32,
    ) {
        let mut eq_changed = false;

        // כאן היה ה-vertical_centered שעשה את הבלאגן. חזרנו ל-vertical הבטוח!
        ui.vertical(|ui| {
            ui.add_space(20.0);

            // --- תפריט הפריסטים העליון ---
            ui.horizontal(|ui| {
                ui.label(
                    egui::RichText::new("EQ PRESETS:")
                        .strong()
                        .color(egui::Color32::from_gray(150)),
                );
                ui.add_space(10.0);

                let presets = [
                    (EqPreset::Flat, "FLAT"),
                    (EqPreset::BassBoost, "BASS BOOST"),
                    (EqPreset::VocalClear, "VOCAL CLEAR"),
                    (EqPreset::Electronic, "ELECTRONIC"),
                ];

                for (preset, label) in presets {
                    let is_selected = self.active_preset == preset;
                    let (bg_color, text_color) = if is_selected {
                        (egui::Color32::from_rgb(0, 200, 255), egui::Color32::BLACK)
                    } else {
                        (
                            egui::Color32::from_rgb(30, 30, 35),
                            egui::Color32::from_gray(180),
                        )
                    };

                    if ui
                        .add(
                            egui::Button::new(
                                egui::RichText::new(label).color(text_color).strong(),
                            )
                            .fill(bg_color),
                        )
                        .clicked()
                    {
                        self.apply_preset(preset);
                        eq_changed = true;
                    }
                }
            });

            ui.add_space(30.0);

            // --- הפאנל המרכזי (חומרת אולפן) ---
            // חזרנו להשתמש ב-Frame::group המובנה שהוא הכי יציב, רק דרסנו לו את הצבעים!
            let mut main_frame = egui::Frame::group(ui.style());
            main_frame.fill = panel_bg;
            main_frame.fill = egui::Color32::from_rgb(18, 18, 20); // רקע שחור עמוק
            main_frame.stroke = egui::Stroke::new(1.0, egui::Color32::from_gray(40)); // מסגרת מתכתית עדינה
            main_frame.inner_margin = egui::Margin::symmetric(20, 20);

            main_frame.show(ui, |ui| {
                ui.label(
                    egui::RichText::new("YMH-PRO 10-BAND STEREO GRAPHIC EQUALIZER")
                        .strong()
                        .size(12.0)
                        .color(egui::Color32::from_gray(100)),
                );
                ui.add_space(25.0);

                let freqs = [
                    "32", "64", "125", "250", "500", "1K", "2K", "4K", "8K", "16K",
                ];

                ui.columns(10, |columns| {
                    for i in 0..10 {
                        columns[i].vertical_centered(|ui| {
                            let val = self.bands[i];

                            // 1. לד חיווי (LED)
                            let led_intensity = ((val + 12.0) / 24.0).clamp(0.1, 1.0);
                            let led_color = if val > 6.0 {
                                egui::Color32::from_rgb(255, 60, 60) // קליפינג - אדום
                            } else {
                                egui::Color32::from_rgb(50, 255, 200).linear_multiply(led_intensity) // תקין - ירוק/תכלת
                            };

                            let (led_rect, _) =
                                ui.allocate_exact_size(egui::vec2(14.0, 4.0), egui::Sense::hover());
                            ui.painter().rect_filled(
                                led_rect,
                                egui::CornerRadius::same(1),
                                led_color,
                            );
                            ui.add_space(10.0);

                            // 2. העיצוב של הסליידר ע"י Scope כדי למנוע שגיאות Borrow Checker
                            ui.scope(|ui| {
                                ui.style_mut().spacing.slider_width = 150.0; // גובה הסליידר
                                ui.style_mut().visuals.widgets.inactive.bg_fill =
                                    egui::Color32::from_rgb(8, 8, 10);
                                ui.style_mut().visuals.widgets.inactive.fg_stroke.color =
                                    egui::Color32::from_gray(200);
                                ui.style_mut().visuals.selection.bg_fill =
                                    self.bands_color(i).linear_multiply(0.4);

                                let slider = egui::Slider::new(&mut self.bands[i], -12.0..=12.0)
                                    .vertical()
                                    .show_value(false);

                                if ui.add(slider).changed() {
                                    self.active_preset = EqPreset::Custom;
                                    eq_changed = true;
                                }
                            });

                            // 3. תצוגת דציבלים ותדר
                            ui.add_space(10.0);
                            let db_text = if val > 0.0 {
                                format!("+{:.1}", val)
                            } else {
                                format!("{:.1}", val)
                            };
                            ui.label(
                                egui::RichText::new(db_text)
                                    .size(11.0)
                                    .color(self.bands_color(i)),
                            );
                            ui.add_space(5.0);
                            ui.label(
                                egui::RichText::new(freqs[i])
                                    .size(12.0)
                                    .strong()
                                    .color(egui::Color32::from_gray(160)),
                            );
                        });
                    }
                });
            });

            ui.add_space(30.0);

            // --- הגדרות מאסטר בתחתית הפאנל ---
            let mut bottom_frame = egui::Frame::group(ui.style());
            bottom_frame.fill = egui::Color32::from_rgb(25, 25, 28);
            bottom_frame.stroke = egui::Stroke::new(1.0, egui::Color32::from_gray(40));
            bottom_frame.inner_margin = egui::Margin::symmetric(20, 15);

            bottom_frame.show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label(
                        egui::RichText::new("BASS BOOST:")
                            .strong()
                            .color(egui::Color32::from_gray(180)),
                    );
                    ui.add_space(10.0);

                    ui.scope(|ui| {
                        ui.style_mut().visuals.selection.bg_fill =
                            egui::Color32::from_rgb(255, 100, 50); // כתום
                        if ui
                            .add(egui::Slider::new(&mut self.bass_boost, 0.0..=1.0).text(""))
                            .changed()
                        {
                            self.active_preset = EqPreset::Custom;
                            eq_changed = true;
                        }
                    });

                    ui.add_space(50.0);

                    // כפתור אקטיבי / לא אקטיבי לאודיו מרחבי
                    let (btn_color, btn_text) = if self.spatial_audio {
                        (egui::Color32::from_rgb(0, 200, 255), "🎧 3D SPATIAL: ON")
                    } else {
                        (egui::Color32::from_rgb(50, 50, 55), "🎧 3D SPATIAL: OFF")
                    };

                    if ui
                        .add(
                            egui::Button::new(
                                egui::RichText::new(btn_text)
                                    .strong()
                                    .color(egui::Color32::WHITE),
                            )
                            .fill(btn_color),
                        )
                        .clicked()
                    {
                        self.spatial_audio = !self.spatial_audio;
                        engine.set_spatial_audio(self.spatial_audio);
                    }
                });
            });
        });

        if eq_changed {
            engine.set_equalizer(&self.bands, self.bass_boost);
        }
    }

    fn bands_color(&self, index: usize) -> egui::Color32 {
        if index < 4 {
            egui::Color32::from_rgb(0, 255, 255) // בס - תכלת
        } else if index < 7 {
            egui::Color32::from_rgb(255, 200, 0) // מיד - צהוב
        } else {
            egui::Color32::from_rgb(255, 50, 100) // טרבל - ורוד
        }
    }
}
