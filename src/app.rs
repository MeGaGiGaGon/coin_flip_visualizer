use std::f64::consts::PI;

use egui::{Color32, DragValue, Pos2};
use num_complex::Complex64;

trait ComplexExtension {
    fn to_pos2(self) -> Pos2;
}

impl ComplexExtension for Complex64 {
    fn to_pos2(self) -> Pos2 {
        Pos2::new(self.re as f32, self.im as f32)
    }
}

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct CoinFlipVisualizer {
    roots: u8,
}

impl Default for CoinFlipVisualizer {
    fn default() -> Self {
        Self {
            roots: 3,
        }
    }
}

impl CoinFlipVisualizer {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for CoinFlipVisualizer {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::right("control_panel").show(ctx, |ui| {
            ui.label("Roots");
            ui.add(DragValue::new(&mut self.roots));
        });
        
        let central_panel_response = egui::CentralPanel::default().show(ctx, |_| {
        }).response;
        let center = central_panel_response.rect.center();
        let available_circle_size = central_panel_response.rect.size().min_elem();
        let painter = ctx.layer_painter(central_panel_response.layer_id);
        let z = Complex64::from_polar(1.0, PI/(self.roots as f64));
        let roots = (0..self.roots).map(|i| z.powf(2.0*i as f64)).collect::<Vec<_>>();
        for root in roots {
            painter.circle_filled(root.scale(available_circle_size as f64 / 2.0).to_pos2() + center.to_vec2(), 1.0, Color32::YELLOW);
        }
    }
}