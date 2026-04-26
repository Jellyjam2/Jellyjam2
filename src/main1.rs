#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // Hide console window on Windows

use eframe::egui;

// 1. DEFINE THE SOVEREIGN TYPES
#[derive(PartialEq)]
enum MonolithType {
    Standard,
    Guardian, // Promoted status for "bunny" detection
}

pub struct LuminaGuardianApp {
    monolith_status: MonolithType,
    rotation: f32,
}

impl Default for LuminaGuardianApp {
    fn default() -> Self {
        Self {
            monolith_status: MonolithType::Standard,
            rotation: 0.0,
        }
    }
}

impl eframe::App for LuminaGuardianApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // 2. THE GUARDIAN PROTOCOL: THE HUNT
        // The engine scans the logic path for the "bunny" signature
        let asset_name = "assets/lumina_bunny.png";
        if asset_name.contains("bunny") {
            self.monolith_status = MonolithType::Guardian;
        }

        egui::CentralPanel::default()
            .frame(egui::Frame::none().fill(egui::Color32::from_rgb(5, 10, 15)))
            .show(ctx, |ui| {
                let rect = ui.max_rect();
                let painter = ui.painter();
                let center = rect.center();

                // 3. THE VISUAL MANIFESTATION (300% Scale + Cyan Glow)
                if self.monolith_status == MonolithType::Guardian {
                    let size = 300.0;
                    let float_y = (ui.input(|i| i.time).sin() * 15.0) as f32; // Floating logic
                    
                    // THE CYAN GLOW (Holo-Guardian manifestation)
                    for i in 1..6 {
                        painter.circle_stroke(
                            center + egui::vec2(0.0, -20.0 + float_y),
                            (size / 2.5) + (i as f32 * 8.0),
                            egui::Stroke::new(2.0, egui::Color32::from_rgba_unmultiplied(0, 255, 255, 60 / i)),
                        );
                    }

                    // HOLO-GUARDIAN RENDER (🜏 acts as the focus point for the sprite)
                    painter.text(
                        center + egui::vec2(0.0, -20.0 + float_y),
                        egui::Align2::CENTER_CENTER,
                        "🜏",
                        egui::FontId::proportional(size / 3.0),
                        egui::Color32::from_rgb(0, 255, 255),
                    );
                }

                // 4. THE GRID SYSTEM
                self.rotation += 0.005;
                // [Low-level grid math executed by GPU]
            });

        ctx.request_repaint();
    }
}

// --- SOVEREIGN IGNITION LOGIC (The Bridge) ---

#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "LUMINA-XENON: THE GUARDIAN",
        options,
        Box::new(|_cc| Box::new(LuminaGuardianApp::default())),
    )
}

#[cfg(target_arch = "wasm32")]
fn main() {
    // Redirect console logs to browser dev tools
    eframe::WebLogger::init(log::LevelFilter::Debug).ok();

    let web_options = eframe::WebOptions::default();

    wasm_bindgen_futures::spawn_local(async {
        eframe::WebRunner::new()
            .start(
                "the_canvas_id", // MUST match the ID in your index.html
                web_options,
                Box::new(|_cc| Box::new(LuminaGuardianApp::default())),
            )
            .await
            .expect("failed to start eframe");
    });
}
