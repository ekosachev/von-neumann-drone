use bevy_ecs::prelude::Resource;

#[derive(Resource)]
pub struct RCurrentTab(pub AppTab);

#[derive(Clone, Copy)]
pub enum AppTab {
    GalaxyMap,
    SystemMap,
    Research,
    Units,
    Statistics,
}

impl AppTab {
    pub fn next(&self) -> Self {
        match self {
            Self::GalaxyMap => Self::SystemMap,
            Self::SystemMap => Self::Research,
            Self::Research => Self::Units,
            Self::Units => Self::Statistics,
            Self::Statistics => Self::GalaxyMap,
        }
    }

    pub fn prev(&self) -> Self {
        match self {
            Self::GalaxyMap => Self::Statistics,
            Self::SystemMap => Self::GalaxyMap,
            Self::Research => Self::SystemMap,
            Self::Units => Self::Research,
            Self::Statistics => Self::Units,
        }
    }
}

impl RCurrentTab {
    pub fn switch_next(&mut self) {
        self.0 = self.0.next();
    }

    pub fn switch_prev(&mut self) {
        self.0 = self.0.prev();
    }
}

impl From<AppTab> for usize {
    fn from(value: AppTab) -> Self {
        match value {
            AppTab::GalaxyMap => 0,
            AppTab::SystemMap => 1,
            AppTab::Research => 2,
            AppTab::Units => 3,
            AppTab::Statistics => 4,
        }
    }
}
