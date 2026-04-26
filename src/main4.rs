use eframe::egui;
use rand::Rng;

// --- 1. THE PARTICLES (SNOW) ---
struct Particle {
    pos: [f32; 3], 
    vel: f32,      
    color: egui::Color32,
}

// --- 2. THE APP STATE ---
pub struct LuminaSovereign {
    particles: Vec<Particle>,
    rotation: f32,  
    pitch: f32,     
    revealed: bool, 
    // Removed "tilt" from struct state, we calculate it live now
}

impl LuminaSovereign {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let mut rng = rand::thread_rng();
        let mut particles = Vec::new();
        for _ in 0..500 {
            particles.push(Particle {
                pos: [
                    rng.gen_range(-800.0..800.0),
                    rng.gen_range(-500.0..500.0),
                    rng.gen_range(-800.0..800.0),
                ],
                vel: rng.gen_range(0.5..2.0),
                color: egui::Color32::from_white_alpha(rng.gen_range(50..200)),
            });
        }

        Self {
            particles,
            rotation: 0.0,
            pitch: 0.5,
            revealed: false,
        }
    }
}

// --- 3. THE 3D MATH ENGINE ---
fn project_iso(pos: [f32; 3], rotation: f32, pitch: f32, center: egui::Pos2) -> egui::Pos2 {
    let (sin_r, cos_r) = rotation.sin_cos();
    let (sin_p, cos_p) = pitch.sin_cos();
    
    let x = pos[0] * cos_r - pos[2] * sin_r;
    let z_temp = pos[0] * sin_r + pos[2] * cos_r;
    let y = pos[1] * cos_p - z_temp * sin_p;
    let z = pos[1] * sin_p + z_temp * cos_p + 800.0; 
    
    let factor = 800.0 / z; 
    egui::pos2(center.x + x * factor, center.y + y * factor)
}

impl eframe::App for LuminaSovereign {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default()
            .frame(egui::Frame::none().fill(egui::Color32::from_rgb(5, 10, 15)))
            .show(ctx, |ui| {
                let rect = ui.max_rect();
                let painter = ui.painter();
                let center = rect.center();

                // --- A. PHYSICS ---
                self.rotation += 0.01; // SPED UP ROTATION for effect
                for p in &mut self.particles {
                    p.pos[1] += p.vel; 
                    if p.pos[1] > 500.0 { p.pos[1] = -500.0; } 
                }

                // --- B. CALCULATION: SYNCED ROTATION ---
                // Instead of just mouse tilt, we calculate a "Spin Vector"
                // based on the same rotation variable as the grid.
                
                // 1. Get Mouse Influence (Interactive)
                let pointer = ctx.input(|i| i.pointer.hover_pos().unwrap_or(center));
                let mouse_tilt = (pointer - center) * 0.05;

                // 2. Get World Influence (Automatic Spin)
                // We use Sin/Cos to make the offset travel in a circle
                let spin_radius = 20.0;
                let spin_offset = egui::vec2(
                    self.rotation.sin() * spin_radius,
                    self.rotation.cos() * spin_radius
                );

                // 3. Combine them (The total "3D Feel")
                let total_tilt = mouse_tilt + spin_offset;

                // --- C. GRID ---
                let grid_size = 1200.0;
                let step = 100.0;
                let grid_color = egui::Color32::from_rgb(0, 40, 40); 

                for i in (-10..10).map(|x| x as f32 * step) {
                    let p1 = project_iso([i, 200.0, -grid_size], self.rotation, self.pitch, center);
                    let p2 = project_iso([i, 200.0, grid_size], self.rotation, self.pitch, center);
                    painter.line_segment([p1, p2], egui::Stroke::new(1.0, grid_color));
                    
                    let p3 = project_iso([-grid_size, 200.0, i], self.rotation, self.pitch, center);
                    let p4 = project_iso([grid_size, 200.0, i], self.rotation, self.pitch, center);
                    painter.line_segment([p3, p4], egui::Stroke::new(1.0, grid_color));
                }

                // --- D. SNOW ---
                for p in &self.particles {
                    let screen_pos = project_iso(p.pos, self.rotation, self.pitch, center);
                    if rect.contains(screen_pos) {
                        painter.circle_filled(screen_pos, 1.5, p.color);
                    }
                }

                // --- E. LOGIC ---
                let is_clicked = ctx.input(|i| i.pointer.any_click());
                if is_clicked { self.revealed = !self.revealed; }

                if !self.revealed {
                    // STATE 1: THE SPINNING BOX
                    // We use total_tilt to offset the center, making it look like it's orbiting
                    let box_size = 140.0;
                    let box_rect = egui::Rect::from_center_size(center + total_tilt, egui::vec2(box_size, box_size));
                    
                    // Shadow Box (Simulates depth behind)
                    let shadow_rect = box_rect.translate(egui::vec2(10.0, 10.0));
                    painter.rect_filled(shadow_rect, 15.0, egui::Color32::from_black_alpha(100));

                    // Main Box
                    painter.rect_filled(box_rect, 15.0, egui::Color32::BLACK);
                    painter.rect_stroke(box_rect, 15.0, egui::Stroke::new(3.0, egui::Color32::WHITE));
                    painter.text(center + total_tilt, egui::Align2::CENTER_CENTER, "SYSTEM", egui::FontId::proportional(24.0), egui::Color32::WHITE);
                } else {
                    // STATE 2: THE SPINNING HOLOGRAM
                    let scale = 130.0;
                    let font_id = egui::FontId::proportional(scale);
                    
                    // 1. TRAIL (Synced to Rotation)
                    for i in (1..=10).rev() {
                        let depth = i as f32 * 5.0;
                        let alpha = (150 - (i * 10)).clamp(0, 255) as u8;
                        
                        // CRITICAL: We subtract the 'total_tilt' scaled by depth.
                        // Since 'total_tilt' contains the rotation sine wave, the trail will "whip" around.
                        let pos_offset = total_tilt - egui::vec2(depth * 0.5, depth * 0.5);
                        
                        painter.text(
                            center + (pos_offset * (i as f32 * 0.2)), // Multiply effect by layer
                            egui::Align2::CENTER_CENTER,
                            "LUMINA",
                            font_id.clone(),
                            egui::Color32::from_rgba_unmultiplied(0, 100, 100, alpha),
                        );
                    }

                    // 2. HERO TEXT
                    painter.text(
                        center + total_tilt,
                        egui::Align2::CENTER_CENTER,
                        "LUMINA",
                        font_id,
                        egui::Color32::WHITE, 
                    );
                }
            });

        ctx.request_repaint();
    }
}

// --- 4. LAUNCHER (STRICT v0.22) ---
#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(1200.0, 800.0)),
        fullscreen: true,
        ..Default::default()
    };
    eframe::run_native(
        "LUMINA SOVEREIGN",
        options,
        Box::new(|cc| Box::new(LuminaSovereign::new(cc))),
    )
}

#[cfg(target_arch = "wasm32")]
fn main() {
    eframe::WebLogger::init(log::LevelFilter::Debug).ok();
    let web_options = eframe::WebOptions::default();
    wasm_bindgen_futures::spawn_local(async {
        eframe::WebRunner::new()
            .start(
                "the_canvas_id", 
                web_options,
                Box::new(|cc| Box::new(LuminaSovereign::new(cc))),
            )
            .await
            .expect("failed to start eframe");
    });
}
