use bevy_ecs::{observer::Trigger, system::ResMut};
use ratatui::crossterm::event::KeyCode;

use crate::{events::key_press::EKeyPress, resources::app_state::RCurrentTab};

pub fn o_general_input_handler(
    trigger: Trigger<EKeyPress>,
    // world: &mut World,
    mut current_tab: ResMut<RCurrentTab>,
) {
    match trigger.event().0 {
        KeyCode::Tab => current_tab.switch_next(),
        KeyCode::BackTab => current_tab.switch_prev(),
        _ => {}
    }
}
