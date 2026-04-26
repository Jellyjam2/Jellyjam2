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
    speed: f32,
    size: f32,
}

impl LuminaApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let mut rng = rand::thread_rng();
        Self {
            // Spawn stars closer to the center so they are instantly visible
            stars: (0..120).map(|_| Star {
                pos: Pos2::new(rng.gen_range(0.0..1000.0), rng.gen_range(0.0..800.0)),
                speed: rng.gen_range(0.5..2.5),
                size: rng.gen_range(1.5..3.0),
            }).collect(),
            system_active: false, // Menu starts closed (Clean look)
            command_buffer: String::new(),
            console_log: vec![
                "VISUAL CORE: RESTORED".to_string(), 
                "NEURAL GEOMETRY: ONLINE".to_string()
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
                self.warp_factor = 6.0; 
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
            _ => {
                self.console_log.push(">> UNKNOWN COMMAND".to_string());
            }
        }

        if self.console_log.len() > 6 { self.console_log.remove(0); }
        self.command_buffer.clear();
    }
}

impl App for LuminaApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        // A. VISUALS: Deep Space Background
        let mut visuals = egui::Visuals::dark();
        visuals.window_fill = Color32::from_rgb(5, 10, 15); 
        ctx.set_visuals(visuals);

        let painter = ctx.layer_painter(egui::LayerId::background());
        let screen_rect = ctx.screen_rect();
        
        // B. GEOMETRIC ENGINE (The "Cool Stuff")
        let star_count = self.stars.len();
        
        // 1. Move Stars
        for star in &mut self.stars {
            star.pos.y += star.speed * self.warp_factor;
            
            // Screen Wrap
            if star.pos.y > screen_rect.height() {
                star.pos.y = 0.0;
                star.pos.x = rand::thread_rng().gen_range(0.0..screen_rect.width());
            }
        }

        // 2. Draw The Neural Web (Connections)
        // We check distance between stars to draw lines
        for i in 0..star_count {
            let p1 = self.stars[i].pos;
            
            // Connect to other stars nearby
            for j in (i + 1)..star_count {
                let p2 = self.stars[j].pos;
                let dist = p1.distance(p2);

                // If close enough, draw the Geometric Line
                if dist < 120.0 {
                    // Line gets dimmer as it gets longer
                    let opacity = (1.0 - (dist / 120.0)) * 150.0;
                    painter.line_segment(
                        [p1, p2], 
                        Stroke::new(1.0, Color32::from_rgba_premultiplied(0, 255, 200, opacity as u8))
                    );
                }
            }

            // Draw the Star Dot
            painter.circle_filled(
                p1,
                self.stars[i].size,
                Color32::from_rgba_premultiplied(0, 255, 255, 200), // Bright Cyan
            );
        }
        
        ctx.request_repaint();

        // C. INTERFACE
        egui::CentralPanel::default().show(ctx, |ui| {
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
                
                // Input Row
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
