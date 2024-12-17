use bevy_ecs::prelude::Event;
use ratatui::crossterm::event::KeyCode;

#[derive(Event)]
pub struct EKeyPress(pub KeyCode);
