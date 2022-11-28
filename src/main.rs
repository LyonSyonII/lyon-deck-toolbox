use eframe::{egui::{self as ui, RichText}, epaint::Vec2};
use steam_deck_tools::StyleHelper;

#[allow(clippy::field_reassign_with_default)]
fn main() {
    let mut native_options = eframe::NativeOptions::default();
    native_options.follow_system_theme = true;
    native_options.initial_window_size = Some(Vec2::new(300., 300.));
    eframe::run_native(
        "Steam Deck Tools",
        native_options,
        Box::new(|cc| Box::new(App::new(cc))),
    )
}

#[derive(Default)]
struct App {
    rwfus: bool
}

impl App {
    fn new(cc: &eframe::CreationContext) -> Self {
        cc.egui_ctx.set_style(ui::Style::default());
        cc.egui_ctx.set_heading_font_style(20., eframe::epaint::FontFamily::Proportional);
        cc.egui_ctx.set_small_font_style(8., eframe::epaint::FontFamily::Proportional); // TODO! Bug!!!!
        cc.egui_ctx.set_visuals(ui::Visuals::light());
        Self::default()
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &eframe::egui::Context, _: &mut eframe::Frame) {
        ui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading("Steam Deck Tools");
                ui.label(RichText::new("Select the tools you want to install, or click 'Install All'.").size(5.));
            });
            
            ui.group(|ui| {
                ui.vertical(|ui| {
                    ui.checkbox(&mut self.rwfus, "Rwfus");
                    ui.small("Creates an overlay over the Root filesystem that allows <code>pacman</code> to install packages with SteamOS readonly enabled.");
                });
            });
        });
    }
}
