mod bundles;
mod components;
mod resources;
mod systems;

use std::io;

use bevy_ecs::{schedule::Schedule, world::World};
use ratatui::{
    crossterm::event::{self, KeyCode, KeyEventKind},
    DefaultTerminal,
};
use resources::terminal::RTerminal;
use systems::ui::test_widget::s_draw_test_widget;

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    terminal.clear()?;
    let app_result = run(terminal);

    ratatui::restore();

    app_result
}

fn run(mut terminal: DefaultTerminal) -> io::Result<()> {
    let mut world = World::new();

    world.insert_resource(RTerminal(terminal));

    let mut ui_shedule = Schedule::default();

    ui_shedule.add_systems(s_draw_test_widget);

    loop {
        ui_shedule.run(&mut world);
        if let event::Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                return Ok(());
            }
        }
    }
}
