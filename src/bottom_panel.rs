// src/bottom_panel.rs
use crate::audio::{AudioEngine, PlayerState};
use eframe::egui;

pub fn draw_bottom_controls(
    ctx: &egui::Context,
    engine: &mut AudioEngine,
    accent_color: egui::Color32,
    duration: f64,
    progress: f32,
    volume: &mut f32,
    tex_play: &egui::TextureHandle,
    tex_pause: &egui::TextureHandle,
    tex_next: &egui::TextureHandle,
    tex_prev: &egui::TextureHandle,
) -> Option<String> {
    let mut action = None;
    // --- 1. פס הסטטוס התחתון (Sleek & Thin) עם לדים תגובתיים לשמע! ---
    egui::TopBottomPanel::bottom("pro_status_strip")
        .exact_height(24.0)
        .frame(
            egui::Frame::NONE
                .fill(egui::Color32::from_rgb(8, 8, 12))
                .inner_margin(egui::Margin::symmetric(15, 4)),
        )
        .show(ctx, |ui| {
            ui.horizontal_centered(|ui| {
                let status_color = match engine.current_state {
                    PlayerState::Playing => egui::Color32::from_rgb(50, 255, 100),
                    PlayerState::Paused => egui::Color32::from_rgb(255, 200, 50),
                    _ => egui::Color32::from_gray(150),
                };

                ui.label(
                    egui::RichText::new(format!("{:?}", engine.current_state).to_uppercase())
                        .color(status_color)
                        .size(11.0)
                        .strong(),
                );
                ui.add_space(10.0);
                ui.label(
                    egui::RichText::new("|")
                        .color(egui::Color32::from_gray(50))
                        .size(11.0),
                );
                ui.add_space(10.0);
                ui.label(
                    egui::RichText::new("DOLBY ATMOS")
                        .color(egui::Color32::from_rgb(200, 50, 255))
                        .size(11.0),
                );
                ui.add_space(10.0);
                ui.label(
                    egui::RichText::new("325.1kHz")
                        .color(egui::Color32::from_rgb(50, 200, 255))
                        .size(11.0),
                );
                ui.add_space(10.0);
                ui.label(
                    egui::RichText::new("BASS BOOST")
                        .color(egui::Color32::from_rgb(50, 255, 100))
                        .size(11.0),
                );

                // --- משיכת נתוני האודיו האמיתיים מהמנוע עבור הלדים ---
                // --- משיכת נתוני האודיו האמיתיים מהמנוע עבור הלדים ---
                let mut led_levels = vec![0.25; 12]; // ברירת מחדל: 25% כדי שלא ייעלמו לעולם
                if engine.current_state == PlayerState::Playing {
                    if let Ok(data) = engine.spectrum_data.lock() {
                        if data.len() >= 12 {
                            for i in 0..12 {
                                let db_value = data[i + 2];
                                // רגישות משופרת - מינימום 0.25, מקסימום 1.0
                                let factor = ((db_value + 55.0) / 55.0).clamp(0.25, 1.0);
                                led_levels[i] = factor;
                            }
                        }
                    }
                }

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(
                        egui::RichText::new("Rust Rhythms Pro")
                            .color(egui::Color32::from_rgb(255, 180, 50))
                            .size(11.0),
                    );
                    ui.add_space(20.0);

                    // --- ציור פס הלדים התגובתי (PRO GLOW) ---
                    let led_count = 12;
                    let led_radius = 2.5;
                    let led_spacing = 8.0;
                    let total_width = (led_count as f32) * (led_radius * 2.0 + led_spacing);

                    let (rect, _response) = ui.allocate_exact_size(
                        egui::vec2(total_width, led_radius * 2.0),
                        egui::Sense::hover(),
                    );

                    for i in 0..led_count {
                        let center_x =
                            rect.min.x + (i as f32) * (led_radius * 2.0 + led_spacing) + led_radius;
                        let center = egui::pos2(center_x, rect.center().y);

                        let intensity = led_levels[i];

                        // 1. הילה (Bloom): גדולה יותר (2.8x) ואטימות גבוהה יותר (0.6 במקום 0.4)
                        let bloom_color = accent_color.linear_multiply(intensity * 0.6);
                        ui.painter()
                            .circle_filled(center, led_radius * 2.8, bloom_color);

                        // 2. צבע הליבה: חזק ולא נעלם (מינימום 0.4)
                        let core_color = accent_color.linear_multiply(intensity.max(0.4));
                        ui.painter().circle_filled(center, led_radius, core_color);

                        // 3. "Hot Core": כשהבס פוגע חזק, המרכז נדלק בלבן זוהר!
                        let hotness = ((intensity - 0.7) / 0.3).clamp(0.0, 1.0);
                        if hotness > 0.0 {
                            let white_glow =
                                egui::Color32::from_white_alpha((200.0 * hotness) as u8);
                            ui.painter()
                                .circle_filled(center, led_radius * 0.6, white_glow);
                        }
                    }
                });
            });
        });
    // --- 1. פס הסטטוס התחתון (Sleek & Thin) ---

    // --- 2. פאנל הנגן המרכזי (Player Controls) ---
    egui::TopBottomPanel::bottom("unified_bottom_panel")
        .exact_height(90.0) // גובה קבוע שלא יקפוץ
        .frame(
            egui::Frame::none()
                .fill(egui::Color32::from_rgb(18, 18, 24))
                .inner_margin(egui::Margin::symmetric(25, 10)),
        ) // נותן לזה "לנשום" מהצדדים
        .show(ctx, |ui| {
            // שורה 1: פס ההתקדמות המטורף
            ui.vertical_centered_justified(|ui| {
                ui.horizontal(|ui| {
                    let current_time_str = format_time(engine.current_position);
                    let total_time_str = format_time(duration);

                    // טקסט זמן אפור ועדין
                    ui.label(
                        egui::RichText::new(current_time_str)
                            .size(12.0)
                            .color(egui::Color32::from_gray(180)),
                    );

                    // ה"פטיש": מכריחים את הסליידר להיות בדיוק ברוחב המסך פחות 50 פיקסלים
                    let mut style = (*ctx.style()).clone();
                    style.spacing.slider_width = ui.available_width() - 50.0;
                    style.visuals.selection.bg_fill = accent_color;
                    style.spacing.interact_size.y = 12.0; // פס דק ואלגנטי
                    ui.set_style(style);

                    let mut temp_progress = progress;
                    let slider = egui::Slider::new(&mut temp_progress, 0.0..=100.0)
                        .show_value(false)
                        .trailing_fill(true);

                    if ui.add(slider).changed() {
                        engine.seek(temp_progress);
                    }

                    // זמן סופי בצד ימין
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.label(
                            egui::RichText::new(total_time_str)
                                .size(12.0)
                                .color(egui::Color32::from_gray(180)),
                        );
                    });
                });
            });

            ui.add_space(15.0);

            // שורה 2: כפתורי ניגון וווליום
            ui.horizontal(|ui| {
                // חישוב למירכוז מושלם
                let center_offset = ui.available_width() / 2.0 - 75.0;
                ui.add_space(center_offset);

                let btn_size = egui::Vec2::new(35.0, 35.0); // גודל אחיד לכל הכפתורים

                // PREV (לבן נקי)
                if ui
                    .add(
                        egui::ImageButton::new(
                            egui::Image::new(tex_prev)
                                .fit_to_exact_size(btn_size)
                                .tint(egui::Color32::WHITE),
                        )
                        .frame(false),
                    )
                    .clicked()
                {
                    action = Some("PREV".to_string());
                }

                ui.add_space(20.0);

                // PLAY/PAUSE (הפכנו גם אותו ללבן נקי!)
                let is_playing = engine.current_state == PlayerState::Playing;
                let play_tex = if is_playing { tex_pause } else { tex_play };
                if ui
                    .add(
                        egui::ImageButton::new(
                            egui::Image::new(play_tex)
                                .fit_to_exact_size(btn_size)
                                .tint(egui::Color32::WHITE),
                        )
                        .frame(false),
                    )
                    .clicked()
                {
                    if is_playing {
                        engine.pause();
                    } else {
                        engine.play();
                    }
                }

                ui.add_space(20.0);

                // NEXT (לבן נקי)
                if ui
                    .add(
                        egui::ImageButton::new(
                            egui::Image::new(tex_next)
                                .fit_to_exact_size(btn_size)
                                .tint(egui::Color32::WHITE),
                        )
                        .frame(false),
                    )
                    .clicked()
                {
                    action = Some("NEXT".to_string());
                }

                // סליידר ווליום מצד ימין (קטן ומינימליסטי)
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    let mut style = (*ctx.style()).clone();
                    style.spacing.slider_width = 80.0; // סליידר ווליום קצר
                    style.spacing.interact_size.y = 8.0;
                    ui.set_style(style);

                    if ui
                        .add(
                            egui::Slider::new(volume, 0.0..=1.0)
                                .show_value(false)
                                .trailing_fill(true),
                        )
                        .changed()
                    {
                        engine.set_volume(*volume);
                    }
                    ui.label(
                        egui::RichText::new("🔊")
                            .size(14.0)
                            .color(egui::Color32::from_gray(180)),
                    );
                });
            });
        });

    action
}

fn format_time(seconds: f64) -> String {
    let mins = (seconds / 60.0).floor() as u32;
    let secs = (seconds % 60.0) as u32;
    format!("{:02}:{:02}", mins, secs)
}
