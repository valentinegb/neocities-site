use std::sync::Arc;

use build_timestamp::build_timestamp;
use chrono::DateTime;
use eframe::{App, CreationContext};
use egui::{
    text::LayoutJob, warn_if_debug_build, Align, CentralPanel, Color32, FontData, FontDefinitions,
    FontFamily, FontSelection, Layout, RichText, TopBottomPanel,
};

trait RichTextExt {
    fn proportional_thin(self) -> Self;
    fn proportional_bold(self) -> Self;
    fn proportional_italics(self) -> Self;
}

impl RichTextExt for RichText {
    fn proportional_thin(self) -> Self {
        self.family(FontFamily::Name("proportional-thin".into()))
    }
    fn proportional_bold(self) -> Self {
        self.family(FontFamily::Name("proportional-bold".into()))
    }
    fn proportional_italics(self) -> Self {
        self.family(FontFamily::Name("proportional-italic".into()))
    }
}

pub struct NeocitiesSiteApp {
    about_window_open: bool,
    fonts_window_open: bool,
}

impl NeocitiesSiteApp {
    pub fn new(cc: &CreationContext) -> Self {
        let mut font_definitions = FontDefinitions::default();

        font_definitions.font_data.insert(
            "IBMPlexSans-Regular".to_owned(),
            Arc::new(FontData::from_static(include_bytes!(
                "../assets/fonts/IBMPlexSans-Regular.ttf"
            ))),
        );
        font_definitions.font_data.insert(
            "IBMPlexSans-Thin".to_owned(),
            Arc::new(FontData::from_static(include_bytes!(
                "../assets/fonts/IBMPlexSans-Thin.ttf"
            ))),
        );
        font_definitions.font_data.insert(
            "IBMPlexSans-Bold".to_owned(),
            Arc::new(FontData::from_static(include_bytes!(
                "../assets/fonts/IBMPlexSans-Bold.ttf"
            ))),
        );
        font_definitions.font_data.insert(
            "IBMPlexSans-Italic".to_owned(),
            Arc::new(FontData::from_static(include_bytes!(
                "../assets/fonts/IBMPlexSans-Italic.ttf"
            ))),
        );
        font_definitions.font_data.insert(
            "NotoEmoji-Light".to_owned(),
            Arc::new(FontData::from_static(include_bytes!(
                "../assets/fonts/NotoEmoji-Light.ttf"
            ))),
        );
        font_definitions.font_data.insert(
            "NotoEmoji-Bold".to_owned(),
            Arc::new(FontData::from_static(include_bytes!(
                "../assets/fonts/NotoEmoji-Bold.ttf"
            ))),
        );
        font_definitions.font_data.insert(
            "IBMPlexMono-Regular".to_owned(),
            Arc::new(FontData::from_static(include_bytes!(
                "../assets/fonts/IBMPlexMono-Regular.ttf"
            ))),
        );

        font_definitions.families.insert(
            FontFamily::Proportional,
            vec![
                "IBMPlexSans-Regular".to_owned(),
                "NotoEmoji-Regular".to_owned(),
                "emoji-icon-font".to_owned(),
            ],
        );
        font_definitions.families.insert(
            FontFamily::Name("proportional-thin".into()),
            vec![
                "IBMPlexSans-Thin".to_owned(),
                "NotoEmoji-Light".to_owned(),
                "emoji-icon-font".to_owned(),
            ],
        );
        font_definitions.families.insert(
            FontFamily::Name("proportional-bold".into()),
            vec![
                "IBMPlexSans-Bold".to_owned(),
                "NotoEmoji-Bold".to_owned(),
                "emoji-icon-font".to_owned(),
            ],
        );
        font_definitions.families.insert(
            FontFamily::Name("proportional-italic".into()),
            vec![
                "IBMPlexSans-Italic".to_owned(),
                "NotoEmoji-Regular".to_owned(),
                "emoji-icon-font".to_owned(),
            ],
        );
        font_definitions.families.insert(
            FontFamily::Monospace,
            vec![
                "IBMPlexMono-Regular".to_owned(),
                "IBMPlexSans-Regular".to_owned(),
                "NotoEmoji-Regular".to_owned(),
                "emoji-icon-font".to_owned(),
            ],
        );
        cc.egui_ctx.set_fonts(font_definitions);

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
        egui::Window::new("Why does this website suck so much?").open(&mut self.about_window_open).show(ctx, |ui| {
            let mut layout_job = LayoutJob::default();

            RichText::new("I don't really like web dev, but I thought making a Neocities site would be cool. So, what, I just sucked it up and programmed in ").append_to(&mut layout_job, ui.style(), FontSelection::Default, Align::Center);
            RichText::new("*shudder* ").proportional_thin().append_to(&mut layout_job, ui.style(), FontSelection::Default, Align::Center);
            RichText::new("JavaScript? ").proportional_italics().append_to(&mut layout_job, ui.style(), FontSelection::Default, Align::Center);
            RichText::new("No, of course not! I used ").append_to(&mut layout_job, ui.style(), FontSelection::Default, Align::Center);
            RichText::new("Rust ").proportional_bold().append_to(&mut layout_job, ui.style(), FontSelection::Default, Align::Center);
            RichText::new("instead, which some may say is overengineering but I think this was ").append_to(&mut layout_job, ui.style(), FontSelection::Default, Align::Center);
            RichText::new("way ").italics().append_to(&mut layout_job, ui.style(), FontSelection::Default, Align::Center);
            RichText::new("easier than trying to decipher the nonsensical errors from a language without static typing.").append_to(&mut layout_job, ui.style(), FontSelection::Default, Align::Center);
            ui.label(layout_job);
        });
        egui::Window::new("Fonts")
            .open(&mut self.fonts_window_open)
            .show(ctx, |ui| {
                let mut layout_job = LayoutJob::default();

                RichText::new(
                    "I like fonts. It's always fun when some text catches my eye \
                    and I just have to learn about the font and consider every \
                    character of the font as if I were in a museum. That isn't \
                    necessarily to say that I have good taste in fonts, just \
                    that I enjoy fonts. Anyway, besides the fonts egui gives you \
                    by default, I'm using ",
                )
                .append_to(
                    &mut layout_job,
                    ui.style(),
                    FontSelection::Default,
                    Align::Center,
                );
                RichText::new("IBM Plex").proportional_bold().append_to(
                    &mut layout_job,
                    ui.style(),
                    FontSelection::Default,
                    Align::Center,
                );
                RichText::new(".").append_to(
                    &mut layout_job,
                    ui.style(),
                    FontSelection::Default,
                    Align::Center,
                );

                ui.label(layout_job);
            });
        TopBottomPanel::bottom("footer").show(ctx, |ui| {
            ui.columns_const(|[col_1, col_2, col_3]| {
                col_1.horizontal_wrapped(|ui| {
                    let mut layout_job = LayoutJob::default();

                    RichText::new("Last updated ").append_to(
                        &mut layout_job,
                        ui.style(),
                        FontSelection::Default,
                        Align::Center,
                    );
                    RichText::new(
                        DateTime::from_timestamp(build_timestamp!(), 0)
                            .unwrap()
                            .to_string(),
                    )
                    .proportional_bold()
                    .append_to(
                        &mut layout_job,
                        ui.style(),
                        FontSelection::Default,
                        Align::Center,
                    );
                    ui.label(layout_job);
                    warn_if_debug_build(ui);
                });
                col_2.centered_and_justified(|ui| {
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
                    ui.label(layout_job).on_hover_text("That's a bit cliche isn't it? I should think of something more original...");
                });
                col_3.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    ui.toggle_value(&mut self.about_window_open, "About");
                    ui.toggle_value(&mut self.fonts_window_open, "Fonts");
                });
            });
        });
        CentralPanel::default().show(ctx, |_ui| {});
    }
}
