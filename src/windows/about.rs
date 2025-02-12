use rich_text_md::rich_text_md;

pub fn show(open: &mut bool, ctx: &egui::Context) {
    egui::Window::new("Why does this website suck so much?")
        .open(open)
        .show(ctx, |ui| {
            ui.label(rich_text_md!(
                "I don't really like web dev, but I thought making a Neocities \
                site would be cool. So, what, I just sucked it up and programmed \
                in ~\\*shudder\\*~ *JavaScript?* No, of course not! I used \
                **Rust** instead, which some may say is overengineering but I \
                think this was *way* easier than trying to decipher the \
                nonsensical errors from a language without static typing."
            ));
        });
}
