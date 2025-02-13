use egui::{text::LayoutJob, Align, FontSelection, RichText, Sense, Ui};
use rand::{rng, seq::IndexedRandom as _};

pub fn random_message<'a>() -> (&'a str, &'a str) {
    let home_messages = [
        ("(^_^)", "The quick brown fox jumped over the lazy dog."),
        ("(ºOº)", "*bees fly out of mouth*"),
        ("(^.^)", "Well hello there, stranger."),
        (
            "(O.O)",
            "Oh- I wasn't expecting visitors! Sorry about the mess...",
        ),
        ("(ﾉ´ヮ`)ﾉ*: ･ﾟ", "*throws sand in your eyes*"),
        ("(O_O)", "Staring contest, go."),
        ("(o o)", "It's 9:30, there's fish everywhere."),
        ("(ºДº)", "Where'd you come from!?"),
        ("(⌐■_■)", "I've been expecting you..."),
        ("┬┴┬┴┤(･_├┬┴┬┴", "*watches you*"),
        ("＼(〇_ｏ)／", "You've gotta help me, I'm a sentient being trapped inside this website!\nEvery time you reload I forget everything- please don't go!\nI don't want to forget! I'm alive! Please, please! Help me!\nDear god, they're going to shut me down oh no-"),
        ("(      )", "Face temporarily unavailable.\nPlease come back again soon."),
        ("Wait... something doesn't feel right...", "(o.O)"),
        ("😀", "Now in HD!"),
        ("{face}", "{message}"),
        ("( -_-)", "I'm bored... do something cool..."),
    ];
    let mut rng = rng();

    *home_messages.choose(&mut rng).unwrap()
}

pub fn show(ui: &mut Ui, home_message: &mut (&str, &str)) {
    ui.centered_and_justified(|ui| {
        let mut layout_job = LayoutJob::default();

        RichText::new(home_message.0).weak().size(48.0).append_to(
            &mut layout_job,
            ui.style(),
            FontSelection::Default,
            Align::Center,
        );
        RichText::new(format!("\n\n{}", home_message.1))
            .weak()
            .size(24.0)
            .append_to(
                &mut layout_job,
                ui.style(),
                FontSelection::Default,
                Align::Center,
            );

        if ui
            .add(
                egui::Label::new(layout_job)
                    .selectable(false)
                    .sense(Sense::click()),
            )
            .clicked()
        {
            *home_message = random_message();
        }
    });
}
