mod fonts;
mod windows;

use build_timestamp::build_timestamp;
use chrono::DateTime;
use eframe::{App, CreationContext};
use egui::{
    text::LayoutJob, warn_if_debug_build, Align, CentralPanel, Color32, FontSelection, Hyperlink,
    Layout, Response, RichText, TopBottomPanel, Ui,
};
use fonts::{font_definitions, RichTextExt as _};
use rich_text_md::rich_text_md;

pub struct NeocitiesSiteApp {
    about_window_open: bool,
    fonts_window_open: bool,
}

impl NeocitiesSiteApp {
    pub fn new(cc: &CreationContext) -> Self {
        cc.egui_ctx.set_fonts(font_definitions());

        Self::default()
    }
}

impl Default for NeocitiesSiteApp {
    fn default() -> Self {
        Self {
            about_window_open: true,
            fonts_window_open: false,
        }
    }
}

impl App for NeocitiesSiteApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        windows::about::show(&mut self.about_window_open, ctx);
        windows::fonts::show(&mut self.fonts_window_open, ctx);
        TopBottomPanel::bottom("footer").show(ctx, |ui| {
            ui.columns_const(|[col_1, col_2, col_3]| {
                col_1.horizontal_wrapped(|ui| {
                    let date_and_time = DateTime::from_timestamp(build_timestamp!(), 0).unwrap();

                    ui.label(rich_text_md!("Last updated **{date_and_time}**"));
                    warn_if_debug_build(ui);
                });
                col_2.centered_and_justified(made_with_love_and_rust);
                col_3.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    ui.toggle_value(&mut self.about_window_open, "About");
                    ui.toggle_value(&mut self.fonts_window_open, "Fonts");
                    ui.add(
                        Hyperlink::from_label_and_url(
                            "Source Code",
                            "https://github.com/valentinegb/neocities-site",
                        )
                        .open_in_new_tab(true),
                    );
                });
            });
        });
        CentralPanel::default().show(ctx, |_ui| ());
    }
}

fn made_with_love_and_rust(ui: &mut Ui) -> Response {
    let mut layout_job = LayoutJob::default();

    RichText::new("Made with ").append_to(
        &mut layout_job,
        ui.style(),
        FontSelection::Default,
        Align::Center,
    );
    RichText::new("♥")
        .proportional_bold()
        .color(Color32::RED)
        .append_to(
            &mut layout_job,
            ui.style(),
            FontSelection::Default,
            Align::Center,
        );
    RichText::new(" and ").append_to(
        &mut layout_job,
        ui.style(),
        FontSelection::Default,
        Align::Center,
    );
    RichText::new("Rust")
        .proportional_bold()
        .color(Color32::BROWN)
        .append_to(
            &mut layout_job,
            ui.style(),
            FontSelection::Default,
            Align::Center,
        );

    ui.label(layout_job)
        .on_hover_text("That's a bit cliche isn't it? I should think of something more original...")
}
