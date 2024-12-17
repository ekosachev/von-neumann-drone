use bevy_ecs::prelude::{Query, Res, ResMut};
use ratatui::{
    layout::{Constraint, Layout},
    style::{palette::tailwind, Modifier, Style},
    widgets::Tabs,
};

use crate::{
    components::ui::tab_menu::CTabMenu,
    resources::{app_state::RCurrentTab, terminal::RTerminal},
};

pub fn s_draw_tab_menu(
    mut terminal: ResMut<RTerminal>,
    tab_menu: Query<&CTabMenu>,
    current_tab: Res<RCurrentTab>,
) {
    let _ = terminal.0.draw(|frame| {
        let widget = Tabs::new(tab_menu.get_single().unwrap().tab_names.clone())
            .select(usize::from(current_tab.0))
            .divider("▕▏")
            .style(
                Style::default()
                    .bg(tailwind::SLATE.c900)
                    .fg(tailwind::PURPLE.c600),
            )
            .highlight_style(
                Style::default()
                    .bg(tailwind::PURPLE.c950)
                    .fg(tailwind::PURPLE.c300)
                    .add_modifier(Modifier::BOLD),
            );
        let layout =
            Layout::vertical(vec![Constraint::Length(1), Constraint::Fill(1)]).split(frame.area());
        frame.render_widget(widget, layout[0]);
    });
}
