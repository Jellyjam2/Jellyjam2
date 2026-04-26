use eframe::{egui, App, Frame};
use egui::{Color32, Pos2, RichText, Stroke, Vec2};
use rand::Rng;

// 1. The Data Structure
struct LuminaApp {
    stars: Vec<Star>,
    system_active: bool,
    command_buffer: String,
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
        }
    }
}

impl App for LuminaApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        // A. VISUALS: Force Dark Mode
        let mut visuals = egui::Visuals::dark();
        visuals.window_fill = Color32::from_rgb(10, 12, 16); 
        ctx.set_visuals(visuals);

        // B. ANIMATION: Starfield
        let painter = ctx.layer_painter(egui::LayerId::background());
        let screen_rect = ctx.screen_rect();
        
        for star in &mut self.stars {
            star.pos.y += star.speed;
            if star.pos.y > screen_rect.height() {
                star.pos.y = 0.0;
                star.pos.x = rand::thread_rng().gen_range(0.0..screen_rect.width());
            }
            painter.circle_filled(
                star.pos,
                star.size,
                Color32::from_rgba_premultiplied(0, 255, 255, (150.0 * star.speed) as u8),
            );
        }
        ctx.request_repaint();

        // C. INTERFACE: Central Button
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(150.0);
                let btn = egui::Button::new(RichText::new("SYSTEM").size(20.0).strong())
                    .min_size(Vec2::new(150.0, 60.0))
                    .fill(if self.system_active { Color32::from_rgb(0, 100, 100) } else { Color32::TRANSPARENT })
                    .stroke(Stroke::new(2.0, Color32::from_rgb(0, 255, 255)));

                if ui.add(btn).clicked() {
                    self.system_active = !self.system_active;
                }

                ui.add_space(20.0);
                ui.label(RichText::new("NEURAL LINK: ESTABLISHED").color(Color32::from_rgb(0, 255, 255)));
                ui.add(egui::TextEdit::singleline(&mut self.command_buffer).desired_width(300.0));
            });
        });

        // D. LOGIC: Diagnostics Window
        if self.system_active {
            egui::Window::new("CORE DIAGNOSTICS")
                .default_pos([50.0, 50.0])
                .show(ctx, |ui| {
                    ui.heading("STATUS: ONLINE");
                    ui.label(format!("Frame Time: {:.2}ms", ctx.input(|i| i.stable_dt) * 1000.0));
                    ui.label("Memory Integrity: 100%");
                    if ui.button("PURGE CACHE").clicked() {
                        self.command_buffer.clear();
                    }
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

// 2. THE WEB ENTRY POINT (This fixes the errors)
#[cfg(target_arch = "wasm32")]
fn main() {
    // Redirect logs to console
    eframe::WebLogger::init(log::LevelFilter::Debug).ok();

    let web_options = eframe::WebOptions::default();

    wasm_bindgen_futures::spawn_local(async {
        eframe::WebRunner::new()
            .start(
                "the_canvas_id", // CONNECTS TO YOUR NEW INDEX.HTML
                web_options,
                Box::new(|cc| Box::new(LuminaApp::new(cc))),
            )
            .await
            .expect("failed to start eframe");
    });
}

// Desktop Fallback (Just in case)
#[cfg(not(target_arch = "wasm32"))]
fn main() {
    // Empty for now, we are focused on Web
}
