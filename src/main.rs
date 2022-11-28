use eframe::egui as ui;
use steam_deck_tools::StyleHelper;

fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Steam Deck Tools",
        native_options,
        Box::new(|cc| Box::new(App::new(cc))),
    )
}

#[derive(Default)]
struct App;

impl App {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        cc.egui_ctx.set_heading_font_style(22., eframe::epaint::FontFamily::Proportional);
        cc.egui_ctx.set_small_font_style(14., eframe::epaint::FontFamily::Proportional); // TODO! Bug!!!!
        cc.egui_ctx.set_visuals(ui::Visuals::light());
        Self::default()
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &eframe::egui::Context, _: &mut eframe::Frame) {
        ui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading("Steam Deck Tools");
                ui.label("Select the tools you want to install, or click 'Install All'.\nHover the mouse over each entry to see a quick explanation of what each one does.").on_hover_text("Tooltip!");
            });
        });
    }
}
