use chrono::Local;
use eframe::egui;
use eframe::App as EframeApp;

struct ClockApp;

impl EframeApp for ClockApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let time = Local::now().format("%H:%M:%S").to_string();
            ui.heading(time);
        });
        ctx.request_repaint_after(std::time::Duration::from_millis(200));
    }
}

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([200.0, 80.0])
            .with_title("Rust Clock"),
        ..Default::default()
    };
    eframe::run_native(
        "Rusty Clock",
        options,
        Box::new(|_cc| Ok(Box::new(ClockApp))),
    )
}
