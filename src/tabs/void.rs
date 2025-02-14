use egui::{text::LayoutJob, Ui};
use rich_text_md::rich_text_md;

struct Note<'a> {
    title: &'a str,
    date: &'a str,
    content: LayoutJob,
}

fn notes(ui: &Ui) -> Vec<Note> {
    vec![
        Note {
            title: "The Void",
            date: "2025-02-13",
            content: rich_text_md!(
                "**The Void** is what I'm calling this area of the site where I \
                plan on adding random notes and thoughts from time to time, like \
                \"shouting into the void\"."
            ),
        },
        Note {
            title: "Japanese Font",
            date: "2025-02-13",
            content: rich_text_md!(
                "I added the Japanese version of the font the site uses literally \
                just for that one message on the homepage about sand. (Click the \
                text on the homepage until you find it if you haven't seen it \
                yet.) This probably increases the size of the build (and thus \
                the time it takes for the site to load) by like ~7.2 MB. Was it \
                worth it? Probably not at the moment, but the site still loads \
                fast enough for the Neocities screenshot to not show the loading \
                screen, so that's good enough for me. I am *learning* Japanese (\
                こんにちは！　元気ですか？　私の日本人はとてもよいじゃないです。), so maybe \
                it'll be more worth it later."
            ),
        },
        Note {
            title: "Valentine's Day",
            date: "2025-02-14",
            content: rich_text_md!(
                "Happy Valentine's Day!\n\nSo, my name *is* actually Valentine. \
                But, funnily enough, I was born on a different saint's holiday. \
                Every year I get comments about how it's \"my day\", but it \
                stopped being something I cared about when people stopped giving \
                me free candy. This year though, I do have some plans with \
                somebody, which I'm pretty excited for. :)"
            ),
        },
    ]
}

fn show_note(
    ui: &Ui,
    Note {
        title,
        date,
        content,
    }: Note,
) {
    egui::Window::new(format!("{date} - {title}"))
        .default_open(false)
        .show(ui.ctx(), |ui| {
            ui.label(content);
        });
}

pub fn show(ui: &Ui) {
    for note in notes(ui) {
        show_note(ui, note);
    }
}
