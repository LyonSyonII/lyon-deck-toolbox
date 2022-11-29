use eframe::{
    egui::{self as ui, Button, Hyperlink, Layout, RichText, ScrollArea, Ui, CentralPanel, Frame, Style, TextStyle},
    emath::Align,
    epaint::Vec2,
};
use steam_deck_tools::StyleHelper;


#[allow(clippy::field_reassign_with_default)]
fn main() {
    let mut native_options = eframe::NativeOptions::default();
    native_options.follow_system_theme = true;
    //native_options.initial_window_size = Some(Vec2::new(300., 300.));
    eframe::run_native(
        "Steam Deck Tools",
        native_options,
        Box::new(|cc| Box::new(App::new(cc))),
    )
}

#[derive(Default)]
struct App;

impl App {
    fn new(cc: &eframe::CreationContext) -> Self {
        let pixels_per_point = cc.integration_info.native_pixels_per_point.unwrap_or(1.);
        cc.egui_ctx.set_style(ui::Style::default());
        cc.egui_ctx.set_small_font_style(16., eframe::epaint::FontFamily::Proportional);
        cc.egui_ctx.set_body_font_style(22.5, eframe::epaint::FontFamily::Proportional);
        cc.egui_ctx.set_heading_font_style(54., eframe::epaint::FontFamily::Proportional);
        cc.egui_ctx.set_button_font_style(20., eframe::epaint::FontFamily::Proportional);
        cc.egui_ctx.divide_font_sizes_by(pixels_per_point);
        cc.egui_ctx.set_visuals(ui::Visuals::light());
        App::default()
    }

    fn install_tools(&self, all: bool) {
        
    }
}

fn tool(ui: &mut Ui, title: &str, description: &str, repo: &str, callback: impl FnOnce()) {
    let heading = ui.style().text_styles.get(&TextStyle::Heading).unwrap().size;
    let body = ui.style().text_styles.get(&TextStyle::Body).unwrap().size;

    ui.vertical(|ui| {
        ui.horizontal(|ui| {
            ui.label(RichText::new(title).size(heading * 0.67));
            if ui.add(Button::new(RichText::new("Install"))).clicked() { 
                callback() 
            }
        });
        ui.label(RichText::from(description));
        ui.add(Hyperlink::from_label_and_url(
            RichText::new("Repo").small(),
            repo,
        ));
    });
}

fn tools(ui: &mut Ui) {
    ScrollArea::vertical().show(ui, |ui| {
        tool(ui, 
            "Rwfus", 
            "Like a vinyl couch cover for your filesystem, Rwfus covers your Deck's /usr/ directory (and some others) allowing you to initialize and use pacman (the Arch Linux package manager) on the Steam Deck without losing packages when the next update comes out.", 
            "https://github.com/ValShaped/rwfus", 
            || {
                let mut path = std::env::var("HOME").unwrap();
                path.push_str("/.local/share/");
                std::env::set_current_dir(path).unwrap();
                std::process::Command::new("ls").spawn().unwrap();
            }
        );
        ui.separator();
        tool(ui, 
            "CryoUtilities", 
            "Scripts and utilities to enhance the Steam Deck experience, particularly performance.\nCurrent Functionality:\n - Swap File Resizer\n - Swappiness Changer", 
            "https://github.com/CryoByte33/steam-deck-utilities", 
            || {

            }
        );
        ui.separator();
        tool(ui, 
            "Emudeck", 
            "EmuDeck is a collection of scripts that allows you to autoconfigure your Steam Deck, it creates your roms directory structure and downloads all of the needed Emulators for you along with the best configurations for each of them.", 
            "https://github.com/dragoonDorise/EmuDeck", 
            || {

            }
        );
    });
    //ui.group(|ui| {});
}

impl eframe::App for App {
    fn update(&mut self, ctx: &eframe::egui::Context, _: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.label(RichText::new("Steam Deck Tools").underline().heading());
                ui.label(RichText::new("Click the 'Install' button of each tool to install it.").small());
            });
            
            tools(ui);
        });
    }
}
