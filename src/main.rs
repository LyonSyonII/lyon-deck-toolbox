use eframe::{egui::{self as ui, RichText, Ui, ScrollArea, Layout, Button, Hyperlink}, epaint::Vec2, emath::Align};
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
        cc.egui_ctx.set_body_font_style(3., eframe::epaint::FontFamily::Proportional);
        cc.egui_ctx.set_visuals(ui::Visuals::light());
        Self::default()
    }
}

fn tool(ui: &mut Ui, title: &str, description: &str, repo: &str, checked: &mut bool) {
    ui.vertical(|ui| {
        ui.horizontal(|ui| {
            ui.checkbox(checked,RichText::from(title).size(9.));
            ui.add(Hyperlink::from_label_and_url(RichText::new("Repo").size(5.), repo));
        });
        ui.label(RichText::from(description).size(7.));
    });
}

impl eframe::App for App {
    fn update(&mut self, ctx: &eframe::egui::Context, _: &mut eframe::Frame) {
        ui::TopBottomPanel::bottom("Bottom").show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.button(RichText::new("Install Selected").size(8.)).clicked() {
                        
                }
                if ui.button(RichText::new("Install All").size(8.)).clicked() {
                
                }
            });
        });
        
        ui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.label(RichText::new("Steam Deck Tools").underline().heading());
                ui.label(RichText::new("Select the tools you want to install, or click 'Install All'.").size(5.));
            });

            ui.group(|ui| {
                ScrollArea::vertical().show(ui, |ui| {
                    tool(ui, "Rwfus", "Like a vinyl couch cover for your filesystem, Rwfus covers your Deck's /usr/ directory (and some others) allowing you to initialize and use pacman (the Arch Linux package manager) on the Steam Deck without losing packages when the next update comes out.", "https://github.com/ValShaped/rwfus", &mut self.rwfus);
                    tool(ui, "Rwfus", "Like a vinyl couch cover for your filesystem, Rwfus covers your Deck's /usr/ directory (and some others) allowing you to initialize and use pacman (the Arch Linux package manager) on the Steam Deck without losing packages when the next update comes out.", "https://github.com/ValShaped/rwfus", &mut self.rwfus);
                    tool(ui, "Rwfus", "Like a vinyl couch cover for your filesystem, Rwfus covers your Deck's /usr/ directory (and some others) allowing you to initialize and use pacman (the Arch Linux package manager) on the Steam Deck without losing packages when the next update comes out.", "https://github.com/ValShaped/rwfus", &mut self.rwfus);
                    tool(ui, "Rwfus", "Like a vinyl couch cover for your filesystem, Rwfus covers your Deck's /usr/ directory (and some others) allowing you to initialize and use pacman (the Arch Linux package manager) on the Steam Deck without losing packages when the next update comes out.", "https://github.com/ValShaped/rwfus", &mut self.rwfus);
                    tool(ui, "Rwfus", "Like a vinyl couch cover for your filesystem, Rwfus covers your Deck's /usr/ directory (and some others) allowing you to initialize and use pacman (the Arch Linux package manager) on the Steam Deck without losing packages when the next update comes out.", "https://github.com/ValShaped/rwfus", &mut self.rwfus);
                });
            });
        });
    }
}
