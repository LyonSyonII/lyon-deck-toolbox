use std::ops::Deref;

use eframe::{
    egui::{
        Style,
        TextStyle::{self, Body, Button, Heading, Monospace, Small},
    },
    epaint::{FontFamily, FontId},
};

type FontStyle = (f32, FontFamily);
#[derive(Default, Clone, Debug)]
pub struct FontStyles {
    small: (f32, FontFamily),
    body: (f32, FontFamily),
    monospace: (f32, FontFamily),
    button: (f32, FontFamily),
    heading: (f32, FontFamily),
}

impl FontStyles {
    pub fn new(
        small: FontStyle,
        body: FontStyle,
        monospace: FontStyle,
        button: FontStyle,
        heading: FontStyle,
    ) -> FontStyles {
        FontStyles {
            small,
            body,
            monospace,
            button,
            heading,
        }
    }

    pub fn with_small_style(mut self, size: f32, family: FontFamily) -> FontStyles {
        self.small = (size, family);
        self
    }

    pub fn with_body_style(mut self, size: f32, family: FontFamily) -> FontStyles {
        self.body = (size, family);
        self
    }

    pub fn with_monospace_style(mut self, size: f32, family: FontFamily) -> FontStyles {
        self.monospace = (size, family);
        self
    }

    pub fn with_button_style(mut self, size: f32, family: FontFamily) -> FontStyles {
        self.heading = (size, family);
        self
    }

    pub fn with_heading_style(mut self, size: f32, family: FontFamily) -> FontStyles {
        self.small = (size, family);
        self
    }
}

pub trait StyleHelper {
    fn set_small_font_style(&self, size: f32, family: FontFamily);
    fn set_body_font_style(&self, size: f32, family: FontFamily);
    fn set_monospace_font_style(&self, size: f32, family: FontFamily);
    fn set_button_font_style(&self, size: f32, family: FontFamily);
    fn set_heading_font_style(&self, size: f32, family: FontFamily);
    fn set_font_styles(&self, styles: FontStyles);

    fn with_small_font_style(self, size: f32, family: FontFamily) -> Self;
    fn with_body_font_style(self, size: f32, family: FontFamily) -> Self;
    fn with_monospace_font_style(self, size: f32, family: FontFamily) -> Self;
    fn with_button_font_style(self, size: f32, family: FontFamily) -> Self;
    fn with_heading_font_style(self, size: f32, family: FontFamily) -> Self;
    fn with_font_styles(self, styles: FontStyles) -> Self;
}

impl StyleHelper for eframe::egui::Context {
    fn set_small_font_style(&self, size: f32, family: FontFamily) {
        let mut style = self.style().deref().clone();
        *style.text_styles.get_mut(&TextStyle::Small).unwrap() = FontId::new(size, family);
        self.set_style(style);
    }

    fn set_body_font_style(&self, size: f32, family: FontFamily) {
        let mut style = self.style().deref().clone();
        *style.text_styles.get_mut(&TextStyle::Body).unwrap() = FontId::new(size, family);
        self.set_style(style);
    }

    fn set_monospace_font_style(&self, size: f32, family: FontFamily) {
        let mut style = self.style().deref().clone();
        *style.text_styles.get_mut(&TextStyle::Monospace).unwrap() = FontId::new(size, family);
        self.set_style(style);
    }

    fn set_button_font_style(&self, size: f32, family: FontFamily) {
        let mut style = self.style().deref().clone();
        *style.text_styles.get_mut(&TextStyle::Button).unwrap() = FontId::new(size, family);
        self.set_style(style);
    }

    fn set_heading_font_style(&self, size: f32, family: FontFamily) {
        let mut style = self.style().deref().clone();
        *style.text_styles.get_mut(&TextStyle::Heading).unwrap() = FontId::new(size, family);
        self.set_style(style);
    }

    fn set_font_styles(&self, styles: FontStyles) {
        self.set_small_font_style(styles.small.0, styles.small.1);
        self.set_body_font_style(styles.body.0, styles.body.1);
        self.set_monospace_font_style(styles.monospace.0, styles.monospace.1);
        self.set_button_font_style(styles.button.0, styles.button.1);
        self.set_heading_font_style(styles.heading.0, styles.heading.1);
    }

    fn with_small_font_style(self, size: f32, family: FontFamily) -> Self {
        self.set_small_font_style(size, family);
        self
    }

    fn with_body_font_style(self, size: f32, family: FontFamily) -> Self {
        self.set_body_font_style(size, family);
        self
    }

    fn with_monospace_font_style(self, size: f32, family: FontFamily) -> Self {
        self.set_monospace_font_style(size, family);
        self
    }

    fn with_button_font_style(self, size: f32, family: FontFamily) -> Self {
        self.set_button_font_style(size, family);
        self
    }

    fn with_heading_font_style(self, size: f32, family: FontFamily) -> Self {
        self.set_heading_font_style(size, family);
        self
    }

    fn with_font_styles(self, styles: FontStyles) -> Self {
        self.set_font_styles(styles);
        self
    }
}
