use std::io::Stdout;

use bevy_ecs::prelude::Resource;
use ratatui::prelude::{CrosstermBackend, Terminal};

#[derive(Resource)]
pub struct RTerminal(pub Terminal<CrosstermBackend<Stdout>>);
