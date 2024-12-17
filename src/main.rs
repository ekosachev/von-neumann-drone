mod bundles;
mod components;
mod events;
mod observers;
mod resources;
mod systems;

use std::io;

use bevy_ecs::{schedule::Schedule, world::World};
use components::ui::tab_menu::CTabMenu;
use events::key_press::EKeyPress;
use observers::input_handler::o_general_input_handler;
use ratatui::{
    crossterm::event::{self, KeyCode, KeyEventKind},
    DefaultTerminal,
};
use resources::{
    app_state::{AppTab, RCurrentTab},
    terminal::RTerminal,
};
use systems::ui::tab_menu::s_draw_tab_menu;

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
    world.insert_resource(RCurrentTab(AppTab::GalaxyMap));

    world.spawn(CTabMenu::default());

    world.add_observer(o_general_input_handler);
    // world.add_observer(o_tab_menu_input_handler);
    world.flush();

    let mut ui_shedule = Schedule::default();
    ui_shedule.add_systems(s_draw_tab_menu);

    loop {
        ui_shedule.run(&mut world);
        if let event::Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                if key.code == KeyCode::Esc {
                    return Ok(());
                }
                world.trigger(EKeyPress(key.code));
            }
        }
    }
}
