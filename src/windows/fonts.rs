use rich_text_md::rich_text_md;

pub fn show(open: &mut bool, ctx: &egui::Context) {
    egui::Window::new("Fonts").open(open).show(ctx, |ui| {
        ui.label(rich_text_md!(
            "I like fonts. It's always fun when some text catches my eye and I \
            just have to learn about the font and consider every character of \
            the font as if I were in a museum. That isn't necessarily to say \
            that I have good taste in fonts, just that I enjoy fonts. Anyway, \
            besides the fonts egui gives you by default, I'm using **IBM Plex**."
        ));
    });
}
