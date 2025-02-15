use egui::Ui;

mod commit_log;
mod home;
mod void;

#[derive(Clone)]
#[repr(u8)]
pub enum Tab<'a> {
    Home(Option<home::TabData<'a>>),
    TheVoid,
    CommitLog(Option<commit_log::TabData<'a>>),
}

impl PartialEq for Tab<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.discriminant() == other.discriminant()
    }
}

impl From<u8> for Tab<'_> {
    fn from(value: u8) -> Self {
        match value {
            0 => Tab::Home(None),
            1 => Tab::TheVoid,
            2 => Tab::CommitLog(None),
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
            Tab::Home(tab_data) => home::show(ui, tab_data),
            Tab::TheVoid => void::show(ui),
            Tab::CommitLog(tab_data) => commit_log::show(ui, tab_data),
        }
    }
}
