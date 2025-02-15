use chrono::{DateTime, Utc};
use commit_log::commits;
use egui::{text::LayoutJob, ScrollArea, TextStyle, Ui};
use rich_text_md::rich_text_md;

#[derive(Clone)]
struct CommitBody {
    text: LayoutJob,
    shown: bool,
}

#[derive(Clone)]
struct Commit {
    date_time: DateTime<Utc>,
    summary: LayoutJob,
    body: Option<CommitBody>,
}

#[derive(Clone)]
pub struct TabData {
    commits: Vec<Commit>,
}

pub fn show(ui: &mut Ui, tab_data: &mut Option<TabData>) {
    if tab_data.is_none() {
        *tab_data = Some(TabData {
            commits: commits!(),
        });
    }

    let TabData { commits } = tab_data.as_mut().unwrap();

    ScrollArea::both().auto_shrink(false).show_rows(
        ui,
        ui.text_style_height(&TextStyle::Body),
        commits.len(),
        |ui, row_range| {
            for i in row_range {
                let Commit {
                    date_time,
                    summary,
                    body,
                } = &mut commits[i];

                ui.horizontal(|ui| {
                    ui.label(rich_text_md!("~{date_time}~"));
                    ui.vertical(|ui| {
                        ui.horizontal(|ui| {
                            ui.label(summary.clone());

                            if let Some(CommitBody { text: _, shown }) = body {
                                ui.toggle_value(shown, "...");
                            }
                        });

                        if let Some(CommitBody { text, shown }) = body {
                            if *shown {
                                ui.label(text.clone());
                            }
                        }
                    });
                });
            }
        },
    );
}
