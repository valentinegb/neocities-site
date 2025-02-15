use chrono::{DateTime, Utc};
use commit_log::commits;
use egui::{text::LayoutJob, Hyperlink, RichText, ScrollArea, TextStyle, Ui};
use rich_text_md::rich_text_md;

#[derive(Clone)]
struct CommitBody<'a> {
    text: &'a str,
    shown: bool,
}

#[derive(Clone)]
struct Commit<'a> {
    date_time: DateTime<Utc>,
    id: &'a str,
    summary: LayoutJob,
    body: Option<CommitBody<'a>>,
}

#[derive(Clone)]
pub struct TabData<'a> {
    commits: Vec<Commit<'a>>,
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
                    id,
                    summary,
                    body,
                } = &mut commits[i];

                ui.horizontal(|ui| {
                    ui.label(rich_text_md!("~{date_time}~"));
                    ui.vertical(|ui| {
                        ui.horizontal(|ui| {
                            ui.label(summary.clone());

                            if let Some(CommitBody { text: _, shown }) = body {
                                ui.toggle_value(shown, "…");
                            }

                            ui.add(
                                Hyperlink::from_label_and_url(
                                    "↗",
                                    format!(
                                        "https://github.com/valentinegb/neocities-site/commit/{id}"
                                    ),
                                )
                                .open_in_new_tab(true),
                            )
                            .on_hover_text("Show Commit on GitHub");
                        });

                        if let Some(CommitBody { text, shown }) = body {
                            if *shown {
                                ui.label(RichText::new(*text).monospace());
                            }
                        }
                    });
                });
            }
        },
    );
}
