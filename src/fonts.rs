use std::sync::Arc;

use egui::{FontData, FontDefinitions, FontFamily, RichText};

pub trait RichTextExt {
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

pub fn font_definitions() -> FontDefinitions {
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

    font_definitions
}
