mod fonts;
mod tabs;
mod windows;

use build_timestamp::build_timestamp;
use chrono::DateTime;
use eframe::{App, CreationContext};
use egui::{
    text::LayoutJob, warn_if_debug_build, Align, CentralPanel, Color32, FontSelection, Hyperlink,
    Layout, Response, RichText, TopBottomPanel, Ui,
};
use egui_extras::install_image_loaders;
use fonts::{font_definitions, RichTextExt as _};
use rich_text_md::rich_text_md;
use tabs::Tab;
use windows::{about::AboutWindow, fonts::FontsWindow, music_player::MusicPlayerWindow};

pub struct NeocitiesSiteApp<'a> {
    tab: Tab<'a>,
    music_player_window: MusicPlayerWindow<'a>,
    about_window: AboutWindow,
    fonts_window: FontsWindow,
}

impl NeocitiesSiteApp<'_> {
    pub fn new(cc: &CreationContext<'_>) -> anyhow::Result<Self> {
        cc.egui_ctx.set_fonts(font_definitions());
        install_image_loaders(&cc.egui_ctx);

        Ok(Self {
            tab: Tab::default(),
            music_player_window: MusicPlayerWindow::new(cc)?,
            about_window: AboutWindow::new(),
            fonts_window: FontsWindow::new(),
        })
    }
}

impl App for NeocitiesSiteApp<'_> {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.about_window.show(ctx);
        self.fonts_window.show(ctx);
        self.music_player_window.show(ctx);
        TopBottomPanel::top("navbar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                self.tab.all_nav_buttons(ui);
            });
        });
        TopBottomPanel::bottom("footer").show(ctx, |ui| {
            ui.columns_const(|[col_1, col_2, col_3]| {
                col_1.horizontal_wrapped(|ui| {
                    let date_and_time = DateTime::from_timestamp(build_timestamp!(), 0).unwrap();

                    ui.label(rich_text_md!("Last updated **{date_and_time}**"));
                    warn_if_debug_build(ui);
                });
                col_2.centered_and_justified(made_with_love_and_rust);
                col_3.with_layout(
                    Layout::right_to_left(Align::Min).with_main_wrap(true),
                    |ui| {
                        ui.toggle_value(&mut self.about_window.open, "About");
                        ui.toggle_value(&mut self.fonts_window.open, "Fonts");
                        ui.add(
                            Hyperlink::from_label_and_url(
                                "Source Code",
                                "https://github.com/valentinegb/neocities-site",
                            )
                            .open_in_new_tab(true),
                        );
                    },
                );
            });
        });
        CentralPanel::default().show(ctx, |ui| self.tab.show(ui));
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
