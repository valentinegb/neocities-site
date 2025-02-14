use commit_log::commits;
use egui::{ScrollArea, TextStyle, Ui};
use rich_text_md::rich_text_md;

pub fn show(ui: &mut Ui) {
    let commits = commits!();

    ScrollArea::vertical().auto_shrink(false).show_rows(
        ui,
        ui.text_style_height(&TextStyle::Body),
        commits.len(),
        |ui, row_range| {
            for i in row_range {
                let (date_time, message) = &commits[i];

                ui.horizontal(|ui| {
                    ui.label(rich_text_md!("~{date_time}~"));
                    ui.label(message.clone());
                });
            }
        },
    );
}
