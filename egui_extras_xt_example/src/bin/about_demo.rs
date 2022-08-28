use eframe::egui::{self, Style, Visuals};
use eframe::emath::vec2;

use egui_extras_xt::show_about_window;

struct AboutDemoApp {
    about_open: bool,
}

impl Default for AboutDemoApp {
    fn default() -> Self {
        Self { about_open: false }
    }
}

impl eframe::App for AboutDemoApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            if ui.button("About").clicked() {
                self.about_open = true;
            }
        });

        show_about_window!(ctx, &mut self.about_open);
    }
}

fn main() {
    let options = eframe::NativeOptions {
        initial_window_size: Some(vec2(640.0, 480.0)),
        ..Default::default()
    };

    eframe::run_native(
        "About demo",
        options,
        Box::new(|cc| {
            cc.egui_ctx.set_style(Style {
                visuals: Visuals::dark(),
                ..Style::default()
            });

            Box::new(AboutDemoApp::default())
        }),
    );
}