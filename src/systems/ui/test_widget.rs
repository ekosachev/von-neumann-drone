use bevy_ecs::prelude::ResMut;
use ratatui::widgets::Paragraph;

use crate::resources::terminal::RTerminal;

pub fn s_draw_test_widget(mut terminal: ResMut<RTerminal>) {
    let _ = terminal.0.draw(|frame| {
        let hello_world = Paragraph::new("Hello world!");
        frame.render_widget(hello_world, frame.area());
    });
}
