use eframe::{
    egui::{
        self as ui, style::Margin, CentralPanel, Frame, Hyperlink, RichText, ScrollArea, TextStyle,
        Ui, Style, Window,
    },
    epaint::{Rounding, Vec2, Pos2},
};
use serde::Deserialize;
use lyon_deck_toolbox::{download_from_repo, install_tool, ExpectRepo, StyleHelper, UiHelper};

#[derive(Deserialize)]
struct Tool {
    title: String,
    description: String,
    repo: String,
    needs_root: bool,
    note: Option<String>
}

#[allow(dead_code)]
struct App {
    tools: Vec<Tool>,
    show_error_popup: (bool, String)
}

impl App {
    fn new(cc: &eframe::CreationContext, tools: Vec<Tool>) -> Self {
        cc.egui_ctx.set_pixels_per_point(1.);
        let mut style = Style::default();
        style.visuals.window_rounding = Rounding::same(5.);
        cc.egui_ctx.set_style(style);
        cc.egui_ctx.set_small_font_style(16.);
        cc.egui_ctx.set_body_font_style(22.5);
        cc.egui_ctx.set_heading_font_style(54.);
        cc.egui_ctx.set_button_font_style(30.);
        cc.egui_ctx.set_visuals(ui::Visuals::light());
        App { tools, show_error_popup: (false, String::new()) }
    }

    fn tool(ui: &mut Ui, tool: &Tool) -> anyhow::Result<()> {
        // SAFETY: unwrap is safe, text_styles is guaranteed to contain TextStyle::Heading
        let heading = ui
            .style()
            .text_styles
            .get(&TextStyle::Heading)
            .unwrap()
            .size;
        // SAFETY: unwrap is safe, text_styles is guaranteed to contain TextStyle::Body
        let _body = ui.style().text_styles.get(&TextStyle::Body).unwrap().size;
        let description = tool.description.replace("\\n", "\n");

        ui.columns(2, |columns| {
            columns[0].vertical(|ui| {
                ui.label(RichText::new(&tool.title).strong().size(heading * 0.67));
                ui.label(description);
                ui.add(Hyperlink::from_label_and_url(
                    RichText::new("Repo").small(),
                    &tool.repo,
                ));
            });
            // SAFETY: 'columns' array is guaranteed to have 2 elements
            if columns[1].button_sized("Install", columns[0].min_size()).clicked() {
                install_tool(&tool.title, tool.needs_root)?;
            }
            Ok(())
        })
    }
    
    fn tools(&mut self, ui: &mut Ui) -> anyhow::Result<()> {
        // TODO: Add filter box (do not show tools that do not contain the string)
        ScrollArea::vertical().show(ui, |ui| {
            for t in &self.tools {
                ui.separator();
                ui.add_space(15.);
                Self::tool(ui, t)?;
                ui.add_space(15.);
            }
            ui.separator();
            Ok(())
        }).inner
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &eframe::egui::Context, f: &mut eframe::Frame) {
        let style = ctx.style();
        let frame = Frame {
            inner_margin: Margin {
                left: 20.,
                right: 5.,
                top: 0.,
                bottom: 5.,
            },
            outer_margin: Margin::same(0.),
            rounding: style.visuals.window_rounding,
            shadow: style.visuals.window_shadow,
            fill: style.visuals.window_fill(),
            stroke: style.visuals.window_stroke(),
        };

        CentralPanel::default().frame(frame).show(ctx, |ui| {
            if self.show_error_popup.0 {
                let size = f.info().window_info.size;
                Window::new("Error")
                    .collapsible(false)
                    .resizable(false)
                    .fixed_size(size/2.)
                    .fixed_pos(Pos2::new(size.x/4., size.y/4.))
                    .show(ctx, |ui| {
                        ui.vertical_centered(|ui| {
                            ui.label(&self.show_error_popup.1);
                            if ui.button("Ok").clicked() {
                                self.show_error_popup.0 = false;
                            }
                        });
                    });
                return;
            }
            
            ui.add_space(20.);
            ui.vertical_centered(|ui| {
                ui.label(
                    RichText::new("Lyon's Deck Toolbox")
                        .heading()
                        .underline()
                        .strong(),
                );
                ui.small("Click the 'Install' button of each tool to install it.")
            });
            ui.add_space(20.);
            
            match self.tools(ui) {
                Err(e) if !self.show_error_popup.0 => self.show_error_popup = (true, e.to_string()),
                _ => {}
            }
        });
    }
}

#[allow(clippy::field_reassign_with_default)]
fn main() -> anyhow::Result<()> {
    let input = download_from_repo("tools.yaml")?;
    println!("Parsing 'tools.yaml'");
    let tools: Vec<Tool> =
        serde_yaml::from_str(&input).repo_context("Failed parsing 'tools.yaml'")?;
    println!("Starting GUI");
    let mut native_options = eframe::NativeOptions::default();
    native_options.follow_system_theme = true;
    native_options.initial_window_size = Some(Vec2::new(1280., 800.));
    eframe::run_native(
        "Lyon's Deck Toolbox",
        native_options,
        Box::new(|cc| Box::new(App::new(cc, tools))),
    );
    Ok(())
}
