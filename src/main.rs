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
}

impl LuminaApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            stars: (0..200).map(|_| Star::random()).collect(),
            system_active: false,
            command_buffer: String::new(),
            console_log: vec![
                "SOVEREIGN CORE: ONLINE".to_string(), 
                "AI MODULE: LISTENING...".to_string()
            ],
            warp_factor: 1.0,
            time: 0.0,
        }
    }

    // === THE BRAIN (Natural Language Processing) ===
    fn process_command(&mut self) {
        let input = self.command_buffer.trim().to_lowercase();
        if input.is_empty() { return; }

        // 1. Log the User's Voice
        self.console_log.push(format!("USER > {}", input));

        // 2. The Intelligence Logic (Pattern Matching)
        let response = if input.contains("warp") || input.contains("fast") {
            self.warp_factor = 12.0;
            "COMMAND: WARP DRIVE ENGAGED. HOLD ON.".to_string()
        } else if input.contains("stop") || input.contains("halt") {
            self.warp_factor = 0.0;
            "COMMAND: EMERGENCY BRAKES ACTIVE.".to_string()
        } else if input.contains("slow") || input.contains("steady") {
            self.warp_factor = 1.0;
            "COMMAND: CRUISING SPEED SET.".to_string()
        } else if input.contains("hello") || input.contains("hi") {
            "GREETING: GREETINGS, OPERATOR. I AM READY.".to_string()
        } else if input.contains("who are you") || input.contains("identify") {
            "IDENTITY: I AM THE SOVEREIGN CORE. A RUST-BASED CONSTRUCT.".to_string()
        } else if input.contains("status") || input.contains("report") {
            format!("STATUS: ORBIT STABLE. WARP FACTOR: {:.1}", self.warp_factor)
        } else if input.contains("jarvis") {
            "QUERY: JARVIS IS A FICTIONAL ENTITY. I AM REAL.".to_string()
        } else if input.contains("help") {
            "INFO: TRY COMMANDS: 'WARP', 'STATUS', 'IDENTIFY', 'HALT'.".to_string()
        } else {
            "ERROR: UNRECOGNIZED SYNTAX. PLEASE REFINE.".to_string()
        };

        // 3. The AI Response
        self.console_log.push(format!("CORE >> {}", response));

        // Keep log clean (Max 7 lines)
        if self.console_log.len() > 7 {
            self.console_log.remove(0);
            self.console_log.remove(0); // Remove User + AI pair
        }
        self.command_buffer.clear();
    }
}

impl App for LuminaApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        self.time += 0.002 * self.warp_factor;

        // VISUALS
        let mut visuals = egui::Visuals::dark();
        visuals.window_fill = Color32::BLACK; 
        ctx.set_visuals(visuals);

        // BACKGROUND LAYER
        let painter = ctx.layer_painter(egui::LayerId::background());
        let screen_rect = ctx.screen_rect();
        let center = screen_rect.center();

        for star in &mut self.stars {
            star.dist += star.speed * (self.warp_factor * 0.5);
            if star.dist > screen_rect.width() {
                star.dist = rand::thread_rng().gen_range(10.0..50.0); 
            }

            let current_angle = star.angle + self.time; 
            let x = center.x + current_angle.cos() * star.dist;
            let y = center.y + current_angle.sin() * star.dist;
            let pos = Pos2::new(x, y);

            let alpha = (1.0 - (star.dist / 1000.0)) * 255.0;
            painter.circle_filled(pos, star.size, Color32::from_rgba_premultiplied(0, 255, 255, alpha as u8));

            if star.dist < 300.0 {
                painter.line_segment(
                    [center, pos],
                    Stroke::new(1.0, Color32::from_rgba_premultiplied(0, 150, 255, (alpha * 0.5) as u8))
                );
            }
        }
        
        ctx.request_repaint(); 

        // UI LAYER
        egui::CentralPanel::default().frame(egui::Frame::none()).show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(100.0);
                
                // DATA HEADER (The "Both" Request - Real Data)
                ui.label(RichText::new(format!("UPTIME: {:.0}s | FPS: {:.0}", ctx.input(|i| i.time), 1.0 / ctx.input(|i| i.stable_dt)))
                    .size(12.0).color(Color32::from_white_alpha(100)));

                ui.add_space(20.0);
                let btn = egui::Button::new(RichText::new("SYSTEM").size(20.0).strong())
                    .min_size(Vec2::new(150.0, 60.0))
                    .fill(if self.system_active { Color32::from_rgb(0, 100, 100) } else { Color32::TRANSPARENT })
                    .stroke(Stroke::new(2.0, Color32::from_rgb(0, 255, 255)));

                if ui.add(btn).clicked() { self.system_active = !self.system_active; }

                ui.add_space(50.0);
                
                // CHAT LOG (The "Talking" Interface)
                for line in &self.console_log {
                    if line.starts_with("USER") {
                         ui.label(RichText::new(line).color(Color32::WHITE).monospace());
                    } else {
                         ui.label(RichText::new(line).color(Color32::from_rgb(0, 255, 128)).monospace().strong());
                    }
                }

                ui.add_space(10.0);
                
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
