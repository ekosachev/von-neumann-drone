use bevy_ecs::prelude::Component;

#[derive(Component)]
pub struct CTabMenu {
    pub tab_names: Vec<String>,
}

impl Default for CTabMenu {
    fn default() -> Self {
        Self {
            tab_names: vec![
                String::from("Galaxy map"),
                String::from("System map"),
                String::from("Research"),
                String::from("Units"),
                String::from("Statistics"),
            ],
        }
    }
}
