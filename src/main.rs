use eframe::{egui, App, Frame};
use egui::{Color32, Pos2, RichText, Stroke, Vec2};
use rand::Rng; 
use wasm_bindgen::prelude::*;

// 1. BINDINGS: Rust <-> JS Bridge
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "sovereign"], js_name = processInput)]
    fn process_input_js(text: &str);
}

struct LuminaApp {
    stars: Vec<Star>,
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
}

impl LuminaApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            // FIXED: updated to rand 0.9 syntax
            stars: (0..200).map(|_| Star::random()).collect(),
            command_buffer: String::new(),
            console_log: vec![
                "SYSTEM: TITAN RANK".to_string(), 
                "IDENTITY BRIDGE: ACTIVE".to_string()
            ],
            warp_factor: 1.0,
            time: 0.0,
        }
    }

    fn process_command(&mut self) {
        let input = self.command_buffer.trim().to_string();
        if input.is_empty() { return; }

        self.console_log.push(format!("ENERICO > {}", input));

        // FIXED: Removed unnecessary unsafe block
        process_input_js(&input);

        self.command_buffer.clear();
        
        if self.console_log.len() > 8 {
            self.console_log.remove(0);
        }
    }
}

impl App for LuminaApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        self.time += 0.002 * self.warp_factor;
        
        // FIXED: Frame::none() is deprecated -> Frame::NONE
        let mut visuals = egui::Visuals::dark();
        visuals.window_fill = Color32::BLACK; 
        ctx.set_visuals(visuals);

        let painter = ctx.layer_painter(egui::LayerId::background());
        let screen_rect = ctx.screen_rect();
        let center = screen_rect.center();

        // DRAW VORTEX
        // FIXED: rand 0.9 syntax (random_range)
        let mut rng = rand::rng(); 
        
        for star in &mut self.stars {
            star.dist += star.speed * (self.warp_factor * 0.5);
            if star.dist > screen_rect.width() {
                star.dist = rng.random_range(10.0..50.0); 
            }
            let current_angle = star.angle + self.time; 
            let x = center.x + current_angle.cos() * star.dist;
            let y = center.y + current_angle.sin() * star.dist;
            let pos = Pos2::new(x, y);

            let alpha = (1.0 - (star.dist / 1000.0)) * 255.0;
            painter.circle_filled(pos, star.size, Color32::from_rgba_premultiplied(0, 255, 255, alpha as u8));
        }
        ctx.request_repaint();

        // UI LAYER
        egui::CentralPanel::default().frame(egui::Frame::NONE).show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(150.0);
                ui.label(RichText::new("LUMINA // IDENTITY: ENERICO").size(16.0).color(Color32::from_rgb(0, 255, 255)));
                ui.add_space(50.0);
                
                for line in &self.console_log {
                     ui.label(RichText::new(line).color(Color32::GREEN).monospace());
                }

                ui.add_space(20.0);
                ui.horizontal(|ui| {
                    let width = 380.0; 
                    ui.add_space((ui.available_width() - width) / 2.0);
                    let response = ui.add(egui::TextEdit::singleline(&mut self.command_buffer).desired_width(300.0));
                    
                    if ui.button("SEND").clicked() { self.process_command(); response.request_focus(); }
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
        // FIXED: Updated to rand 0.9 syntax
        let mut rng = rand::rng();
        Self {
            angle: rng.random_range(0.0..std::f32::consts::TAU),
            dist: rng.random_range(10.0..800.0),
            speed: rng.random_range(0.5..1.5),
            size: rng.random_range(1.5..3.0),
        }
    }
}

#[cfg(target_arch = "wasm32")]
fn main() {
    use eframe::wasm_bindgen::JsCast; // Required for Type Casting

    eframe::WebLogger::init(log::LevelFilter::Debug).ok();
    let web_options = eframe::WebOptions::default();

    wasm_bindgen_futures::spawn_local(async {
        // FIXED: eframe 0.31 requires the actual Canvas Element, not just the ID string
        let document = web_sys::window()
            .expect("No window")
            .document()
            .expect("No document");

        let canvas = document
            .get_element_by_id("the_canvas_id")
            .expect("Failed to find canvas")
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .expect("Not a canvas");

        // Start the engine with the corrected signature
        eframe::WebRunner::new()
            .start(
                canvas, 
                web_options, 
                Box::new(|cc| Ok(Box::new(LuminaApp::new(cc)))) // Wrapped in Ok()
            )
            .await
            .expect("failed to start eframe");
    });
}
#[cfg(not(target_arch = "wasm32"))]
fn main() {}
