#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

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

                // 3. THE NEURAL MANIPULATOR (Interactive Manifestation)
                // Get the mouse/touch position, defaulting to center if no input
                let pointer = ctx.input(|i| i.pointer.hover_pos().unwrap_or(center));
                
                // Calculate the "Tilt" - The Guardian leans toward your mouse
                // We use a factor of 0.15 to make the movement smooth and weighty
                let tilt = (pointer - center) * 0.15; 
                
                // Check for click/touch engagement
                let is_clicking = ctx.input(|i| i.pointer.any_down());
                
                // SCALE & GLOW LOGIC: The entity reacts to your physical touch
                let scale = if is_clicking { 350.0 } else { 300.0 };
                let glow_alpha = if is_clicking { 180 } else { 80 };

                if self.monolith_status == MonolithType::Guardian {
                    // THE CYAN MANIFOLD (Multiple layers create the 'Holo' depth)
                    for i in 1..8 {
                        painter.rect_stroke(
                            egui::Rect::from_center_size(
                                center + tilt, 
                                egui::vec2(scale, scale)
                            ),
                            (i as f32 * 2.5).min(30.0), // Procedural rounding
                            egui::Stroke::new(2.0, egui::Color32::from_rgba_unmultiplied(0, 255, 255, (glow_alpha / i) as u8)),
                        );
                    }

                    // THE SOVEREIGN FOCUS (The 🜏 Manifestation)
                    painter.text(
                        center + tilt,
                        egui::Align2::CENTER_CENTER,
                        "🜏",
                        egui::FontId::proportional(scale / 3.0),
                        egui::Color32::from_rgb(0, 255, 255),
                    );
                }

                // 4. THE BACKGROUND GRID SYSTEM
                self.rotation += 0.005;
                // [Isometric math logic can be expanded here for the snow]
            });

        // Force a high-speed repaint for smooth 60fps interaction
        ctx.request_repaint();
    }
}

// --- SOVEREIGN IGNITION LOGIC (The Desktop/Web Bridge) ---

#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(1200.0, 800.0)),
        ..Default::default()
    };
    eframe::run_native(
        "LUMINA-XENON: THE GUARDIAN",
        options,
        Box::new(|_cc| Box::new(LuminaGuardianApp::default())),
    )
}

#[cfg(target_arch = "wasm32")]
fn main() {
    // Redirect console logs to browser dev tools for debugging
    eframe::WebLogger::init(log::LevelFilter::Debug).ok();

    let web_options = eframe::WebOptions::default();

    wasm_bindgen_futures::spawn_local(async {
        eframe::WebRunner::new()
            .start(
                "the_canvas_id", // ID in your index.html
                web_options,
                Box::new(|_cc| Box::new(LuminaGuardianApp::default())),
            )
            .await
            .expect("failed to start eframe");
    });
}
