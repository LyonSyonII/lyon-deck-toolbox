use eframe::{egui::{self as ui, RichText, Ui, ScrollArea, Layout}, epaint::Vec2, emath::Align};
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
        cc.egui_ctx.set_visuals(ui::Visuals::light());
        Self::default()
    }
}

fn tool(ui: &mut Ui, title: &str, description: &str, checked: &mut bool) {
    ui.vertical(|ui| {
        ui.checkbox(checked,RichText::from(title).size(9.));
        ui.label(RichText::from(description).size(7.));
    });
}

impl eframe::App for App {
    fn update(&mut self, ctx: &eframe::egui::Context, _: &mut eframe::Frame) {
        ui::TopBottomPanel::bottom("Bottom").show(ctx, |ui| {
            let mut layout = Layout::left_to_right(Align::Center);
            layout.vertical_align();

            ui.with_layout(layout, |ui| {
                if ui.button(RichText::new("Install Selected").size(8.)).clicked() {
                        
                }
                if ui.button("Install All").clicked() {
                
                }
            });
        });
        
        ui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading("Steam Deck Tools");
                ui.label(RichText::new("Select the tools you want to install, or click 'Install All'.").size(5.));
            });

            ui.group(|ui| {
                ScrollArea::vertical().show(ui, |ui| {
                    tool(ui, "Rwfus", "Creates an overlay over the Root filesystem that allows <code>pacman</code> to install packages with SteamOS readonly enabled.", &mut self.rwfus);
                    tool(ui, "Rwfus", "Creates an overlay over the Root filesystem that allows <code>pacman</code> to install packages with SteamOS readonly enabled.", &mut self.rwfus);
                    tool(ui, "Rwfus", "Creates an overlay over the Root filesystem that allows <code>pacman</code> to install packages with SteamOS readonly enabled.", &mut self.rwfus);
                    tool(ui, "Rwfus", "Creates an overlay over the Root filesystem that allows <code>pacman</code> to install packages with SteamOS readonly enabled.", &mut self.rwfus);
                    tool(ui, "Rwfus", "Creates an overlay over the Root filesystem that allows <code>pacman</code> to install packages with SteamOS readonly enabled.", &mut self.rwfus);
                    tool(ui, "Rwfus", "Creates an overlay over the Root filesystem that allows <code>pacman</code> to install packages with SteamOS readonly enabled.", &mut self.rwfus);
                });
            });

        });
    }
}
