use egui::Ui;

mod commit_log;
mod home;
mod void;

#[derive(Clone)]
#[repr(u8)]
pub enum Tab<'a> {
    Home { message: (&'a str, &'a str) },
    TheVoid,
    CommitLog,
}

impl PartialEq for Tab<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.discriminant() == other.discriminant()
    }
}

impl From<u8> for Tab<'_> {
    fn from(value: u8) -> Self {
        match value {
            0 => Tab::Home {
                message: home::random_message(),
            },
            1 => Tab::TheVoid,
            2 => Tab::CommitLog,
            _ => unreachable!(),
        }
    }
}

impl Default for Tab<'_> {
    fn default() -> Self {
        Tab::from(0)
    }
}

impl Tab<'_> {
    fn name(discriminant: &u8) -> &str {
        match *discriminant {
            0 => "Valentine's Site",
            1 => "The Void",
            2 => "Commit Log",
            _ => unreachable!(),
        }
    }

    fn discriminant(&self) -> u8 {
        unsafe { *(self as *const Self as *const u8) }
    }

    pub fn all_nav_buttons(&mut self, ui: &mut Ui) {
        for discriminant in 0..=2 {
            if ui
                .selectable_label(
                    self.discriminant() == discriminant,
                    Tab::name(&discriminant),
                )
                .clicked()
            {
                *self = Tab::from(discriminant);
            }
        }
    }

    pub fn show(&mut self, ui: &mut Ui) {
        match self {
            Tab::Home { message } => home::show(ui, message),
            Tab::TheVoid => void::show(ui),
            Tab::CommitLog => commit_log::show(ui),
        }
    }
}
