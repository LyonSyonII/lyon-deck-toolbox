use eframe::egui as ui;

fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native("Steam Deck Tools", native_options, Box::new(|cc| Box::new(App::new(cc))))
}

#[derive(Default)]
struct App;

impl App {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let mut style = ui::Style::default();
        let mut text_styles = &mut style.text_styles;
        *text_styles.get_mut(&ui::TextStyle::Heading).unwrap() = ui::FontId::new(30., eframe::epaint::FontFamily::Proportional);
        
        cc.egui_ctx.set_style(style);
        cc.egui_ctx.set_visuals(ui::Visuals::light());
        Self::default()
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {
        ui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Steam Deck Tools");
        });
    }
}