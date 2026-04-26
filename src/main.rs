use eframe::{egui, App, Frame};
use egui::{Color32, Pos2, RichText, Stroke, Vec2};
use rand::Rng;

// 1. Data Structure
struct LuminaApp {
    stars: Vec<Star>,
    system_active: bool,
    command_buffer: String,
    console_log: Vec<String>, 
    warp_factor: f32,         
}

struct Star {
    pos: Pos2,
    origin: Pos2, // Remember where it started (for resetting)
    speed: f32,
    size: f32,
}

impl LuminaApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            stars: (0..150).map(|_| Star::random()).collect(), // Increased star count to 150
            system_active: false,
            command_buffer: String::new(),
            console_log: vec![
                "PHYSICS ENGINE: ONLINE".to_string(), 
                "REPULSOR FIELD: ACTIVE".to_string()
            ],
            warp_factor: 1.0,
        }
    }

    fn process_command(&mut self) {
        let input = self.command_buffer.trim().to_lowercase();
        if input.is_empty() { return; }

        self.console_log.push(format!("> {}", input));

        match input.as_str() {
            "warp" => {
                self.warp_factor = 10.0; 
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
            "clear" => {
                self.console_log.clear();
            }
            "status" => {
                self.console_log.push(format!(">> SPEED: {:.1}x", self.warp_factor));
                self.console_log.push(">> FIELD: REACTIVE".to_string());
            }
            _ => {
                self.console_log.push(">> UNKNOWN COMMAND".to_string());
            }
        }

        if self.console_log.len() > 8 {
            self.console_log.remove(0);
        }
        self.command_buffer.clear();
    }
}

impl App for LuminaApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        let mut visuals = egui::Visuals::dark();
        visuals.window_fill = Color32::from_rgb(5, 8, 12); // Slightly darker space
        ctx.set_visuals(visuals);

        let painter = ctx.layer_painter(egui::LayerId::background());
        let screen_rect = ctx.screen_rect();
        
        // === THE PHYSICS ENGINE ===
        // 1. Get Mouse Position (if inside window)
        let mouse_pos = ctx.input(|i| i.pointer.hover_pos());

        for star in &mut self.stars {
            // A. Standard Movement
            star.pos.y += star.speed * self.warp_factor;

            // B. Gravity Well Logic (Repulsor)
            if let Some(mouse) = mouse_pos {
                let dist_x = star.pos.x - mouse.x;
                let dist_y = star.pos.y - mouse.y;
                let dist = (dist_x * dist_x + dist_y * dist_y).sqrt();

                // If star is within 200 pixels of mouse...
                if dist < 200.0 {
                    // Push it away!
                    let force = (200.0 - dist) * 0.05; // Stronger force when closer
                    star.pos.x += (dist_x / dist) * force * 5.0;
                    star.pos.y += (dist_y / dist) * force * 5.0;
                }
            }

            // C. Screen Wrap (Reset to top)
            if star.pos.y > screen_rect.height() {
                star.pos.y = 0.0;
                star.pos.x = rand::thread_rng().gen_range(0.0..screen_rect.width());
            }
            // Also wrap sides (if pushed off screen by mouse)
            if star.pos.x > screen_rect.width() { star.pos.x = 0.0; }
            if star.pos.x < 0.0 { star.pos.x = screen_rect.width(); }

            // D. Draw
            let brightness = if self.warp_factor > 2.0 { 255 } else { 150 };
            painter.circle_filled(
                star.pos,
                star.size,
                Color32::from_rgba_premultiplied(0, 255, 255, brightness as u8),
            );
        }
        ctx.request_repaint();

        // === INTERFACE ===
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(100.0);
                
                let btn = egui::Button::new(RichText::new("SYSTEM").size(20.0).strong())
                    .min_size(Vec2::new(150.0, 60.0))
                    .fill(if self.system_active { Color32::from_rgb(0, 100, 100) } else { Color32::TRANSPARENT })
                    .stroke(Stroke::new(2.0, Color32::from_rgb(0, 255, 255)));

                if ui.add(btn).clicked() {
                    self.system_active = !self.system_active;
                }

                ui.add_space(50.0);

                for line in &self.console_log {
                    ui.label(RichText::new(line).color(Color32::from_rgb(0, 200, 0)).monospace());
                }

                ui.add_space(10.0);
                ui.label(RichText::new("NEURAL LINK: ESTABLISHED").color(Color32::from_rgb(0, 255, 255)));
                
                ui.horizontal(|ui| {
                    let width = 380.0; 
                    ui.add_space((ui.available_width() - width) / 2.0);

                    let response = ui.add(egui::TextEdit::singleline(&mut self.command_buffer).desired_width(300.0));
                    if ui.button("RUN").clicked() { self.process_command(); response.request_focus(); }
                    if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                        self.process_command();
                        response.request_focus();
                    }
                });
            });
        });

        if self.system_active {
            egui::Window::new("CORE DIAGNOSTICS")
                .default_pos([50.0, 50.0])
                .show(ctx, |ui| {
                    ui.heading("STATUS: ONLINE");
                    ui.label(format!("Warp Factor: {:.1}x", self.warp_factor));
                    ui.label("Physics Engine: ACTIVE");
                });
        }
    }
}

impl Star {
    fn random() -> Self {
        let mut rng = rand::thread_rng();
        let pos = Pos2::new(rng.gen_range(0.0..2000.0), rng.gen_range(0.0..1000.0));
        Self {
            pos,
            origin: pos,
            speed: rng.gen_range(0.5..3.0),
            size: rng.gen_range(1.0..3.0),
        }
    }
}

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
