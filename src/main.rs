use eframe::{egui, App, Frame};
use egui::{Color32, Pos2, RichText, Stroke, Vec2};
use rand::Rng;

struct LuminaApp {
    stars: Vec<Star>,
    system_active: bool,
    command_buffer: String,
    console_log: Vec<String>, 
    warp_factor: f32,         
}

struct Star {
    angle: f32,  // Direction from center
    dist: f32,   // Distance from center
    speed: f32,
    size: f32,
}

impl LuminaApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            // Spawn 150 stars
            stars: (0..150).map(|_| Star::random_start()).collect(),
            system_active: false,
            command_buffer: String::new(),
            console_log: vec![
                "VISUAL LAYER: MERGED".to_string(), 
                "3D GEOMETRY: ENGAGED".to_string()
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
                self.warp_factor = 8.0; 
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
            "clear" => { self.console_log.clear(); }
            _ => { self.console_log.push(">> UNKNOWN COMMAND".to_string()); }
        }
        if self.console_log.len() > 6 { self.console_log.remove(0); }
        self.command_buffer.clear();
    }
}

impl App for LuminaApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        // A. Force Dark Background
        let mut visuals = egui::Visuals::dark();
        visuals.window_fill = Color32::BLACK; 
        ctx.set_visuals(visuals);

        // B. CRITICAL FIX: Transparent Panel (The Glass Window)
        egui::CentralPanel::default()
            .frame(egui::Frame::none()) // <--- THIS REMOVES THE BLACK CURTAIN
            .show(ctx, |ui| {
                
                // C. 3D DRAWING ENGINE (Inside the UI loop now)
                let painter = ui.painter();
                let rect = ui.max_rect();
                let center = rect.center();
                let max_dist = rect.width().max(rect.height()) / 1.2;

                // Update & Draw Stars
                for star in &mut self.stars {
                    // 1. Move Outwards (Radial Physics)
                    star.dist += star.speed * self.warp_factor;

                    // 2. Reset if off screen
                    if star.dist > max_dist {
                        star.dist = rand::thread_rng().gen_range(10.0..50.0); // Respawn in center
                        star.angle = rand::thread_rng().gen_range(0.0..std::f32::consts::TAU);
                    }

                    // 3. Calculate 2D Position
                    let x = center.x + star.angle.cos() * star.dist;
                    let y = center.y + star.angle.sin() * star.dist;
                    let pos = Pos2::new(x, y);

                    // 4. Draw Star
                    let alpha = (star.dist / max_dist) * 255.0; // Fade in as they get closer
                    painter.circle_filled(
                        pos,
                        star.size * (star.dist / 200.0), // Get bigger as they get closer
                        Color32::from_rgba_premultiplied(0, 255, 255, alpha as u8),
                    );

                    // 5. Draw Geometric Connections (The Web)
                    // Connect to center if close
                    if star.dist < 150.0 {
                        painter.line_segment(
                            [center, pos],
                            Stroke::new(1.0, Color32::from_rgba_premultiplied(0, 255, 255, (255.0 - star.dist) as u8))
                        );
                    }
                }
                
                ctx.request_repaint(); // Animate

                // D. UI INTERFACE (Draws ON TOP of stars)
                ui.vertical_centered(|ui| {
                    ui.add_space(100.0);
                    
                    let btn = egui::Button::new(RichText::new("SYSTEM").size(20.0).strong())
                        .min_size(Vec2::new(150.0, 60.0))
                        .fill(if self.system_active { Color32::from_rgb(0, 100, 100) } else { Color32::TRANSPARENT })
                        .stroke(Stroke::new(2.0, Color32::from_rgb(0, 255, 255)));

                    if ui.add(btn).clicked() { self.system_active = !self.system_active; }

                    ui.add_space(50.0);
                    for line in &self.console_log {
                        ui.label(RichText::new(line).color(Color32::from_rgb(0, 255, 128)).monospace());
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
    }
}

impl Star {
    fn random_start() -> Self {
        let mut rng = rand::thread_rng();
        Self {
            angle: rng.gen_range(0.0..std::f32::consts::TAU), // 0 to 360 degrees
            dist: rng.gen_range(10.0..600.0),
            speed: rng.gen_range(0.5..2.5),
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
