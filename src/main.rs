use eframe::{egui, App, Frame};
use egui::{Color32, Pos2, RichText, Stroke, Vec2};
use rand::Rng;

struct LuminaApp {
    stars: Vec<Star>,
    system_active: bool,
    command_buffer: String,
    console_log: Vec<String>, 
    warp_factor: f32,
    time: f32,
}

struct Star {
    angle: f32,
    dist: f32,
    speed: f32,
    size: f32,
    // Removed 'z' to fix the warning
}

impl LuminaApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            stars: (0..200).map(|_| Star::random()).collect(),
            system_active: false,
            command_buffer: String::new(),
            console_log: vec![
                "VISUALS: RE-ROUTED".to_string(), 
                "VORTEX: VISIBLE".to_string()
            ],
            warp_factor: 1.0,
            time: 0.0,
        }
    }

    fn process_command(&mut self) {
        let input = self.command_buffer.trim().to_lowercase();
        if input.is_empty() { return; }
        self.console_log.push(format!("> {}", input));

        match input.as_str() {
            "warp" => {
                self.warp_factor = 10.0; 
                self.console_log.push(">> HYPER-SPIN ENGAGED".to_string());
            }
            "steady" => {
                self.warp_factor = 1.0;
                self.console_log.push(">> ORBIT STABILIZED".to_string());
            }
            "halt" => {
                self.warp_factor = 0.0;
                self.console_log.push(">> ROTATION LOCKED".to_string());
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
        self.time += 0.002 * self.warp_factor;

        // 1. GLOBAL STYLE
        let mut visuals = egui::Visuals::dark();
        visuals.window_fill = Color32::BLACK; 
        ctx.set_visuals(visuals);

        // 2. THE BACKGROUND LAYER (The Stars live here now)
        // This guarantees they are drawn BEHIND the buttons but ON TOP of the black void.
        let painter = ctx.layer_painter(egui::LayerId::background());
        let screen_rect = ctx.screen_rect();
        let center = screen_rect.center();

        for star in &mut self.stars {
            // Physics: Expand Outwards
            star.dist += star.speed * (self.warp_factor * 0.5);
            
            // Reset if too far
            if star.dist > screen_rect.width() {
                star.dist = rand::thread_rng().gen_range(10.0..50.0); // Spawn in center
            }

            // Rotation Math
            let current_angle = star.angle + self.time; 
            let x = center.x + current_angle.cos() * star.dist;
            let y = center.y + current_angle.sin() * star.dist;
            let pos = Pos2::new(x, y);

            // Draw Star (Bright in center, fade at edge)
            let alpha = (1.0 - (star.dist / 1000.0)) * 255.0;
            painter.circle_filled(
                pos,
                star.size,
                Color32::from_rgba_premultiplied(0, 255, 255, alpha as u8),
            );

            // Draw Neural Connections (Spinning Web)
            if star.dist < 300.0 {
                painter.line_segment(
                    [center, pos],
                    Stroke::new(1.0, Color32::from_rgba_premultiplied(0, 100, 255, (alpha * 0.5) as u8))
                );
            }
        }
        
        ctx.request_repaint(); 

        // 3. THE UI LAYER (Transparent Container)
        egui::CentralPanel::default()
            .frame(egui::Frame::none()) // Invisible Frame
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.add_space(150.0);
                    
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
                        
                        // RUN BUTTON
                        if ui.button("RUN").clicked() { 
                            self.process_command(); 
                            response.request_focus(); 
                        }
                        // ENTER KEY
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
    fn random() -> Self {
        let mut rng = rand::thread_rng();
        Self {
            angle: rng.gen_range(0.0..std::f32::consts::TAU),
            dist: rng.gen_range(10.0..800.0),
            speed: rng.gen_range(0.5..1.5),
            size: rng.gen_range(1.5..3.0),
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
