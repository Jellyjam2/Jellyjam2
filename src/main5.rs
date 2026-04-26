#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::egui;
use rand::Rng;
use std::sync::Mutex;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsCast;

// --- 1. GLOBAL NEURAL STATE ---
#[derive(Clone, Copy, Debug)]
pub struct NeuralState {
    pub r: u8, pub g: u8, pub b: u8,
    pub speed: f32,
    pub reveal: bool,
}

lazy_static::lazy_static! {
    static ref NEURAL_LINK: Mutex<NeuralState> = Mutex::new(NeuralState {
        r: 0, g: 255, b: 255, // Default Cyan
        speed: 0.005,
        reveal: false,
    });
}

// This is the Rust-side handler
pub fn update_neural_state(r: u8, g: u8, b: u8, speed: f32, reveal: bool) {
    if let Ok(mut state) = NEURAL_LINK.lock() {
        state.r = r; state.g = g; state.b = b;
        state.speed = speed; state.reveal = reveal;
    }
}

// --- 2. VISUAL ENGINE ---
struct Particle { pos: [f32; 3], vel: f32, color: egui::Color32 }

pub struct LuminaSovereign {
    particles: Vec<Particle>,
    rotation: f32,
    pitch: f32,
}

impl LuminaSovereign {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let mut rng = rand::thread_rng();
        let mut particles = Vec::new();
        for _ in 0..500 {
            particles.push(Particle {
                pos: [rng.gen_range(-800.0..800.0), rng.gen_range(-500.0..500.0), rng.gen_range(-800.0..800.0)],
                vel: rng.gen_range(0.5..2.0),
                color: egui::Color32::from_white_alpha(rng.gen_range(50..200)),
            });
        }
        Self { particles, rotation: 0.0, pitch: 0.5 }
    }
}

fn project_iso(pos: [f32; 3], rot: f32, pitch: f32, center: egui::Pos2) -> egui::Pos2 {
    let (sin_r, cos_r) = rot.sin_cos();
    let (sin_p, cos_p) = pitch.sin_cos();
    let x = pos[0] * cos_r - pos[2] * sin_r;
    let z_t = pos[0] * sin_r + pos[2] * cos_r;
    let y = pos[1] * cos_p - z_t * sin_p;
    let z = pos[1] * sin_p + z_t * cos_p + 800.0;
    egui::pos2(center.x + x * (800.0 / z), center.y + y * (800.0 / z))
}

impl eframe::App for LuminaSovereign {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let state = NEURAL_LINK.lock().unwrap().clone();
        let ai_color = egui::Color32::from_rgb(state.r, state.g, state.b);

        egui::CentralPanel::default().frame(egui::Frame::none().fill(egui::Color32::from_rgb(5, 10, 15))).show(ctx, |ui| {
            let rect = ui.max_rect();
            let painter = ui.painter();
            let center = rect.center();

            self.rotation += state.speed;

            // Physics
            for p in &mut self.particles {
                p.pos[1] += p.vel;
                if p.pos[1] > 500.0 { p.pos[1] = -500.0; }
            }

            // Draw Grid
            let grid_size = 1200.0;
            for i in (-10..10).map(|x| x as f32 * 100.0) {
                let p1 = project_iso([i, 200.0, -grid_size], self.rotation, self.pitch, center);
                let p2 = project_iso([i, 200.0, grid_size], self.rotation, self.pitch, center);
                painter.line_segment([p1, p2], egui::Stroke::new(1.0, egui::Color32::from_rgb(0, 40, 40)));
            }

            // Draw Snow
            for p in &self.particles {
                let s_pos = project_iso(p.pos, self.rotation, self.pitch, center);
                if rect.contains(s_pos) { painter.circle_filled(s_pos, 1.5, p.color); }
            }

            // Draw HUD
            let pointer = ctx.input(|i| i.pointer.hover_pos().unwrap_or(center));
            let tilt = (pointer - center) * 0.1;
            
            if !state.reveal {
                let box_rect = egui::Rect::from_center_size(center + tilt, egui::vec2(140.0, 140.0));
                painter.rect_filled(box_rect, 15.0, egui::Color32::BLACK);
                painter.rect_stroke(box_rect, 15.0, egui::Stroke::new(3.0, ai_color));
                painter.text(center + tilt, egui::Align2::CENTER_CENTER, "SYSTEM", egui::FontId::proportional(24.0), ai_color);
            } else {
                let scale = 130.0;
                for i in (1..=10).rev() {
                    let depth = i as f32 * 5.0;
                    let alpha = (150 - (i * 10)).clamp(0, 255) as u8;
                    let pos_offset = tilt - egui::vec2(depth * 0.5, depth * 0.5);
                    painter.text(center + pos_offset, egui::Align2::CENTER_CENTER, "LUMINA", egui::FontId::proportional(scale), egui::Color32::from_rgba_unmultiplied(state.r, state.g, state.b, alpha));
                }
                painter.text(center + tilt, egui::Align2::CENTER_CENTER, "LUMINA", egui::FontId::proportional(scale), egui::Color32::WHITE);
            }
        });
        ctx.request_repaint();
    }
}

// --- 3. LAUNCHER ---
#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(1200.0, 800.0)),
        fullscreen: true,
        ..Default::default()
    };
    eframe::run_native("LUMINA NEURAL", options, Box::new(|cc| Box::new(LuminaSovereign::new(cc))))
}

#[cfg(target_arch = "wasm32")]
fn main() {
    eframe::WebLogger::init(log::LevelFilter::Debug).ok();

    // --- MANUAL ATTACHMENT STRATEGY ---
    // We explicitly attach the function to 'window.dispatch_neural_command'
    // This bypasses Trunk's naming issues.
    if let Some(window) = web_sys::window() {
        let closure = Closure::wrap(Box::new(move |r, g, b, speed, reveal| {
            update_neural_state(r, g, b, speed, reveal);
        }) as Box<dyn FnMut(u8, u8, u8, f32, bool)>);

        let _ = js_sys::Reflect::set(
            &window,
            &JsValue::from_str("dispatch_neural_command"),
            closure.as_ref().unchecked_ref(),
        );
        closure.forget(); // Keep memory alive
    }

    let web_options = eframe::WebOptions::default();
    wasm_bindgen_futures::spawn_local(async {
        eframe::WebRunner::new()
            .start("the_canvas_id", web_options, Box::new(|cc| Box::new(LuminaSovereign::new(cc))))
            .await
            .expect("failed to start eframe");
    });
}
