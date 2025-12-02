use makepad_widgets::*;
use crate::app::App;

pub mod ui_options;
pub mod tab_overview;

pub fn live_design(cx: &mut Cx) {
    ui_options::live_design(cx);
    tab_overview::live_design(cx);
}

pub fn handle_actions(app: &mut App, cx: &mut Cx, actions:&Actions) {
    ui_options::handle_actions(app, cx, actions);
}

