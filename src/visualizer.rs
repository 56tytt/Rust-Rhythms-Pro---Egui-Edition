// --- 🚀 THE REAL VISUALIZER (POWERAMP STYLE) ---
// הגדלנו קצת את הקנבס שיהיה מרשים יותר
let canvas_width = ui.available_width() * 0.85;
let canvas_height = 250.0;

let (response, painter) = ui.allocate_painter(
    egui::Vec2::new(canvas_width, canvas_height),
    egui::Sense::hover()
);

// משיכת הנתונים מהמנוע (40 ערוצי תדרים)
let data: Vec<f32> = if let Ok(data_lock) = self.engine.spectrum_data.lock() {
    data_lock.clone()
} else {
    Vec::new()
};

if !data.is_empty() {
    let rect = response.rect;
    let num_bars = data.len() as f32;
    let bar_width = rect.width() / num_bars;

    // מרווחים (Gaps) בין העמודות בשביל הלוק של החומרה האמיתית
    let gap = 4.0;
    let actual_bar_width = bar_width - gap;
    let bottom_y = rect.max.y;

    for (i, &db_value) in data.iter().enumerate() {
        // חישוב גובה העמודה (db_value נע בין -60 ל-0)
        let height_factor = ((db_value + 60.0) / 60.0).clamp(0.02, 1.0);
        let bar_height = height_factor * rect.height();

        let x = rect.min.x + (i as f32) * bar_width + gap / 2.0;
        let y = bottom_y - bar_height;

        // 1. שכבת רקע: פס חלש לכל האורך (נותן לוק של תצוגת סטודיו)
        let bg_rect = egui::Rect::from_min_max(
            egui::pos2(x, rect.min.y),
            egui::pos2(x + actual_bar_width, bottom_y)
        );
        painter.rect_filled(
            bg_rect,
            egui::CornerRadius::same(2),
            egui::Color32::from_white_alpha(5) // שקיפות כמעט מלאה
        );

        // 2. שכבת הסאונד: העמודה המרכזית! הופכת ליותר בהירה ככל שהיא גבוהה
        let bar_rect = egui::Rect::from_min_max(
            egui::pos2(x, y),
            egui::pos2(x + actual_bar_width, bottom_y)
        );
        let glow_color = self.accent_color.linear_multiply(height_factor + 0.3);
        painter.rect_filled(bar_rect, egui::CornerRadius::same(2), glow_color);

        // 3. שכבת ה-Peak: הכובע הלבן שקופץ למעלה (הסוד של Poweramp!)
        let cap_rect = egui::Rect::from_min_max(
            egui::pos2(x, y - 6.0), // מרחף 6 פיקסלים מעל העמודה
            egui::pos2(x + actual_bar_width, y - 2.0)
        );
        painter.rect_filled(cap_rect, egui::CornerRadius::same(1), egui::Color32::WHITE);
    }
} else {
    ui.label(egui::RichText::new("Waiting for audio data...").weak());
}
