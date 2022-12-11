use eframe::{
    egui::{self as ui, Button, CentralPanel, Hyperlink, RichText, ScrollArea, TextStyle, Ui},
    epaint::Vec2,
};
use serde::Deserialize;
use steam_deck_tools::{download_from_repo, install_tool, ExpectRepo, StyleHelper};

#[derive(Deserialize)]
struct Tool {
    title: String,
    description: String,
    repo: String,
    needs_root: bool,
}

struct App {
    tools: Vec<Tool>,
}

impl App {
    fn new(cc: &eframe::CreationContext, tools: Vec<Tool>) -> Self {
        let pixels_per_point = cc.integration_info.native_pixels_per_point.unwrap_or(1.);
        cc.egui_ctx.set_style(ui::Style::default());
        cc.egui_ctx
            .set_small_font_style(16., eframe::epaint::FontFamily::Proportional);
        cc.egui_ctx
            .set_body_font_style(22.5, eframe::epaint::FontFamily::Proportional);
        cc.egui_ctx
            .set_heading_font_style(54., eframe::epaint::FontFamily::Proportional);
        cc.egui_ctx
            .set_button_font_style(30., eframe::epaint::FontFamily::Proportional);
        cc.egui_ctx.divide_font_sizes_by(pixels_per_point);
        cc.egui_ctx.set_visuals(ui::Visuals::light());
        App { tools }
    }
    
    fn tool(ui: &mut Ui, tool: &Tool) {
        // SAFETY: unwrap is safe, text_styles is guaranteed to contain TextStyle::Heading
        let heading = ui.style().text_styles.get(&TextStyle::Heading).unwrap().size;
        // SAFETY: unwrap is safe, text_styles is guaranteed to contain TextStyle::Body
        let _body = ui.style().text_styles.get(&TextStyle::Body).unwrap().size;
        let description = tool.description.replace("\\n", "\n");

        ui.horizontal(|ui| {
            // TODO! ui.add_sized([40.0, 20.0], egui::DragValue::new(&mut my_value));
            if ui.add(Button::new(RichText::new("Install"))).clicked() {
                install_tool(&tool.title, tool.needs_root);
            }
            ui.vertical(|ui| {
                ui.label(RichText::new(&tool.title).strong().size(heading * 0.67));
                ui.label(RichText::from(&description));
                ui.add(Hyperlink::from_label_and_url(
                    RichText::new("Repo").small(),
                    &tool.repo,
                ));
            });
        });
        ui.separator();
    }

    fn tools(&self, ui: &mut Ui) {
        ScrollArea::vertical().show(ui, |ui| {
            ui.separator();
            for t in &self.tools {
                Self::tool(ui, t);
            }
        });
    }
}

#[allow(clippy::field_reassign_with_default)]
fn main() {
    let input = download_from_repo("tools.yaml");
    println!("Parsing 'tools.yaml'...");
    let tools: Vec<Tool> = serde_yaml::from_str(&input).expect_repo("Failed parsing 'tools.yaml'");
    println!("Starting GUI...");
    let mut native_options = eframe::NativeOptions::default();
    native_options.follow_system_theme = true;
    native_options.initial_window_size = Some(Vec2::new(1920., 1080.));
    eframe::run_native(
        "Steam Deck Tools",
        native_options,
        Box::new(|cc| Box::new(App::new(cc, tools))),
    )
}

impl eframe::App for App {
    fn update(&mut self, ctx: &eframe::egui::Context, _: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.label(
                    RichText::new("Steam Deck Tools")
                        .underline()
                        .strong()
                        .heading(),
                );
                ui.label(
                    RichText::new("Click the 'Install' button of each tool to install it.").small(),
                );
            });

            self.tools(ui);
        });
    }
}
