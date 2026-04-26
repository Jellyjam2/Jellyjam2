use eframe::{egui, App, Frame};
use egui::{Color32, Pos2, RichText, Stroke, Vec2};
use rand::Rng;

// 1. Data Structure (Added 'console_log' and 'warp_factor')
struct LuminaApp {
    stars: Vec<Star>,
    system_active: bool,
    command_buffer: String,
    console_log: Vec<String>, // Stores the history of text
    warp_factor: f32,         // Controls star speed
}

struct Star {
    pos: Pos2,
    speed: f32,
    size: f32,
}

impl LuminaApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            stars: (0..100).map(|_| Star::random()).collect(),
            system_active: false,
            command_buffer: String::new(),
            console_log: vec!["SYSTEM READY.".to_string(), "AWAITING INPUT...".to_string()],
            warp_factor: 1.0,
        }
    }

    // THE BRAIN: Decides what to do with text
    fn process_command(&mut self) {
        let input = self.command_buffer.trim().to_lowercase();
        
        if input.is_empty() { return; }

        // Echo the user's command
        self.console_log.push(format!("> {}", input));

        // Match the command to an action
        match input.as_str() {
            "warp" => {
                self.warp_factor = 5.0;
                self.console_log.push(">> WARP DRIVE ENGAGED".to_string());
            }
            "steady" => {
                self.warp_factor = 1.0;
                self.console_log.push(">> ENGINES STABILIZED".to_string());
            }
            "halt" => {
                self.warp_factor = 0.0;
                self.console_log.push(">> ALL STOP".to_string());
            }
            "status" => {
                self.console_log.push(">> SYSTEM: OPTIMAL".to_string());
                self.console_log.push(">> SHIELDS: ACTIVE".to_string());
            }
            "clear" => {
                self.console_log.clear();
            }
            _ => {
                self.console_log.push(">> UNKNOWN COMMAND".to_string());
            }
        }

        // Keep log short (Last 6 lines only)
        if self.console_log.len() > 6 {
            self.console_log.remove(0);
        }

        // Clear the input box
        self.command_buffer.clear();
    }
}

impl App for LuminaApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        // A. VISUALS
        let mut visuals = egui::Visuals::dark();
        visuals.window_fill = Color32::from_rgb(10, 12, 16); 
        ctx.set_visuals(visuals);

        // B. ANIMATION (Now multiplied by warp_factor)
        let painter = ctx.layer_painter(egui::LayerId::background());
        let screen_rect = ctx.screen_rect();
        
        for star in &mut self.stars {
            // Apply WARP SPEED
            star.pos.y += star.speed * self.warp_factor;
            
            if star.pos.y > screen_rect.height() {
                star.pos.y = 0.0;
                star.pos.x = rand::thread_rng().gen_range(0.0..screen_rect.width());
            }
            painter.circle_filled(
                star.pos,
                star.size,
                Color32::from_rgba_premultiplied(0, 255, 255, (100.0 * star.speed) as u8),
            );
        }
        ctx.request_repaint();

        // C. INTERFACE
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(100.0);
                
                // The System Button
                let btn = egui::Button::new(RichText::new("SYSTEM").size(20.0).strong())
                    .min_size(Vec2::new(150.0, 60.0))
                    .fill(if self.system_active { Color32::from_rgb(0, 100, 100) } else { Color32::TRANSPARENT })
                    .stroke(Stroke::new(2.0, Color32::from_rgb(0, 255, 255)));

                if ui.add(btn).clicked() {
                    self.system_active = !self.system_active;
                }

                ui.add_space(50.0);

                // === NEW: CONSOLE LOG DISPLAY ===
                for line in &self.console_log {
                    ui.label(RichText::new(line).color(Color32::from_rgb(0, 200, 0)).monospace());
                }

                ui.add_space(10.0);
                ui.label(RichText::new("NEURAL LINK: ESTABLISHED").color(Color32::from_rgb(0, 255, 255)));
                
                // === NEW: INPUT PARSER ===
                let response = ui.add(egui::TextEdit::singleline(&mut self.command_buffer)
                    .desired_width(300.0)
                    .lock_focus(true)); // Keep focus so you can type fast

                // Detect ENTER Key
                if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                    self.process_command();
                    response.request_focus(); // Snap focus back to box
                }
            });
        });

        // D. DIAGNOSTICS HUD
        if self.system_active {
            egui::Window::new("CORE DIAGNOSTICS")
                .default_pos([50.0, 50.0])
                .show(ctx, |ui| {
                    ui.heading("STATUS: ONLINE");
                    ui.label(format!("Warp Factor: {:.1}x", self.warp_factor)); // Show speed
                    ui.label(format!("Frame Time: {:.2}ms", ctx.input(|i| i.stable_dt) * 1000.0));
                });
        }
    }
}

impl Star {
    fn random() -> Self {
        let mut rng = rand::thread_rng();
        Self {
            pos: Pos2::new(rng.gen_range(0.0..2000.0), rng.gen_range(0.0..1000.0)),
            speed: rng.gen_range(0.5..3.0),
            size: rng.gen_range(1.0..3.0),
        }
    }
}

// WEB ENTRY POINT
#[cfg(target_arch = "wasm32")]
fn main() {
    eframe::WebLogger::init(log::LevelFilter::Debug).ok();
    let web_options = eframe::WebOptions::default();
    wasm_bindgen_futures::spawn_local(async {
        eframe::WebRunner::new()
            .start("the_canvas_id", web_options, Box::new(|cc| Box::new(LuminaApp::new(cc))))
            .await.expect("failed to start eframe");
    });
}
#[cfg(not(target_arch = "wasm32"))]
fn main() {}
