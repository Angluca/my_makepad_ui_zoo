use makepad_widgets::*;
use crate::app::App;

pub mod layout_templates;
pub mod ui_options;

pub mod tab_overview;
pub mod tab_my_ui;
pub mod tab_layout;

pub fn live_design(cx: &mut Cx) {
    layout_templates::live_design(cx);
    ui_options::live_design(cx);

    tab_overview::live_design(cx);
    tab_my_ui::live_design(cx);
    tab_layout::live_design(cx);
}

pub fn handle_actions(app: &mut App, cx: &mut Cx, actions:&Actions) {
    ui_options::handle_actions(app, cx, actions);
    tab_my_ui::handle_actions(app, cx, actions);
}

