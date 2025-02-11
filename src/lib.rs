use build_timestamp::build_timestamp;
use chrono::DateTime;
use eframe::App;
use egui::{
    text::LayoutJob, warn_if_debug_build, Align, CentralPanel, FontSelection, Layout, RichText,
    TopBottomPanel,
};

pub struct NeocitiesSiteApp {
    about_window_open: bool,
}

impl Default for NeocitiesSiteApp {
    fn default() -> Self {
        Self {
            about_window_open: true,
        }
    }
}

impl App for NeocitiesSiteApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::Window::new("Why does this website suck so much?").open(&mut self.about_window_open).show(ctx, |ui| {
            let mut layout_job = LayoutJob::default();

            RichText::new("I don't really like web dev, but I thought making a Neocities site would be cool. So, what, I just sucked it up and programmed in ").append_to(&mut layout_job, ui.style(), FontSelection::Default, Align::Min);
            RichText::new("*shudder* ").weak().append_to(&mut layout_job, ui.style(), FontSelection::Default, Align::Min);
            RichText::new("JavaScript? ").italics().append_to(&mut layout_job, ui.style(), FontSelection::Default, Align::Min);
            RichText::new("No, of course not! I used ").append_to(&mut layout_job, ui.style(), FontSelection::Default, Align::Min);
            RichText::new("Rust ").strong().append_to(&mut layout_job, ui.style(), FontSelection::Default, Align::Min);
            RichText::new("instead, which some may say is overengineering but I think this was ").append_to(&mut layout_job, ui.style(), FontSelection::Default, Align::Min);
            RichText::new("way ").italics().append_to(&mut layout_job, ui.style(), FontSelection::Default, Align::Min);
            RichText::new("easier than trying to decipher the nonsensical errors from a language without static typing.").append_to(&mut layout_job, ui.style(), FontSelection::Default, Align::Min);
            ui.label(layout_job);
        });
        TopBottomPanel::bottom("footer").show(ctx, |ui| {
            ui.horizontal(|ui| {
                let mut layout_job = LayoutJob::default();

                RichText::new("Last updated ").append_to(
                    &mut layout_job,
                    ui.style(),
                    FontSelection::Default,
                    Align::Min,
                );
                RichText::new(
                    DateTime::from_timestamp(build_timestamp!(), 0)
                        .unwrap()
                        .to_string(),
                )
                .strong()
                .append_to(
                    &mut layout_job,
                    ui.style(),
                    FontSelection::Default,
                    Align::Min,
                );
                ui.label(layout_job);
                warn_if_debug_build(ui);
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    ui.toggle_value(&mut self.about_window_open, "About");
                });
            });
        });
        CentralPanel::default().show(ctx, |_ui| {});
    }
}
