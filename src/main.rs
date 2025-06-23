use chrono::Local;
use eframe::egui;
use eframe::App as EframeApp;

struct ClockApp;

impl EframeApp for ClockApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        //TO-DO, add multiple themes
        // ctx.set_visuals(egui::Visuals::light());
        // ctx.set_visuals(egui::Visuals::dark());

        let mut visuals = egui::Visuals::dark();
        visuals.override_text_color = Some(egui::Color32::from_rgb(51, 255, 0));
        visuals.panel_fill = egui::Color32::from_rgb(75, 0, 130);
        ctx.set_visuals(visuals);

        // // TO-DO, figure out the rounded corners modules
        // let mut style = (*ctx.style()).clone();
        // style.spacing.item_spacing = egui::vec2(24.0, 18.0); // Space between widgets
        // style.visuals.window_corner_radius = egui::CornerRadius::same(10); // Rounded corners
        // ctx.set_style(style);

        // Change fonts and font sizes:
        let mut fonts = egui::FontDefinitions::default();
        fonts
            .families
            .get_mut(&egui::FontFamily::Proportional)
            .unwrap();
        //.insert(0, "Hack".to_owned()); Maybe add a custom font one day

        ctx.set_fonts(fonts);

        let mut style = (*ctx.style()).clone();
        style
            .text_styles
            .insert(egui::TextStyle::Heading, egui::FontId::monospace(36.0));
        ctx.set_style(style);

        // TO-DO, play with DPI scaling (make UI bigger/smaller)
        //ctx.set_pixels_per_point(1.0); // 1.0 is normal, >1.0 is bigger

        // Timer variable and Centering
        egui::CentralPanel::default().show(ctx, |ui| {
            let time = Local::now().format("%I:%M:%S %p\n%b %e %a, %Y").to_string();
            ui.vertical_centered(|ui| {
                //ui.label(&time);
                ui.heading(&time);
            });

            // TO-DO, makes buttons, probably use this to let user change themes
            // ui.colored_label(egui::Color32::LIGHT_BLUE, "Welcome to your custom clock!");
            // if ui
            //     .add(egui::Button::new("A round button!").rounding(12.0))
            //     .clicked()
            // {
            //     // Button pressed!
            // }
        });
        ctx.request_repaint_after(std::time::Duration::from_millis(200));
    }
}

fn main() -> eframe::Result<()> {
    // Window
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([260.0, 150.0])
            .with_title("Rusty Clock"),
        ..Default::default()
    };

    // Credits
    eframe::run_native(
        "Rusty Clock",
        options,
        Box::new(|_cc| Ok(Box::new(ClockApp))),
    )
}
