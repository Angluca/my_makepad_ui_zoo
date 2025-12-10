use makepad_widgets::*;
use crate::app::App;

pub mod layout_templates;
pub mod demofiletree;

pub mod ui_options;
pub mod tab_my_ui;

pub mod tab_overview;
pub mod tab_layout;
pub mod tab_button;
pub mod tab_checkbox;
pub mod tab_dropdown;
pub mod tab_filetree;
pub mod tab_spinner;
pub mod tab_html;
pub mod tab_icon;
pub mod tab_iconset;
pub mod tab_image;
pub mod tab_imageblend;
pub mod tab_label;
pub mod tab_linklabel;
pub mod tab_shader;
pub mod tab_markdown;
pub mod tab_pageflip;
pub mod tab_portallist;
pub mod tab_radiobutton;
pub mod tab_rotary;

pub fn live_design(cx: &mut Cx) {
    layout_templates::live_design(cx);
    demofiletree::live_design(cx);

    ui_options::live_design(cx);
    tab_my_ui::live_design(cx);

    tab_overview::live_design(cx);
    tab_layout::live_design(cx);
    tab_button::live_design(cx);
    tab_checkbox::live_design(cx);
    tab_dropdown::live_design(cx);
    tab_filetree::live_design(cx);
    tab_spinner::live_design(cx);
    tab_html::live_design(cx);
    tab_icon::live_design(cx);
    tab_iconset::live_design(cx);
    tab_image::live_design(cx);
    tab_imageblend::live_design(cx);
    tab_label::live_design(cx);
    tab_linklabel::live_design(cx);
    tab_shader::live_design(cx);
    tab_markdown::live_design(cx);
    tab_pageflip::live_design(cx);
    tab_portallist::live_design(cx);
    tab_radiobutton::live_design(cx);
    tab_rotary::live_design(cx);
}

pub fn handle_actions(app: &mut App, cx: &mut Cx, actions:&Actions) {
    ui_options::handle_actions(app, cx, actions);
    tab_my_ui::handle_actions(app, cx, actions);

    tab_button::handle_actions(app, cx, actions);
    tab_checkbox::handle_actions(app, cx, actions);
    tab_dropdown::handle_actions(app, cx, actions);
    tab_imageblend::handle_actions(app, cx, actions);
    tab_shader::handle_actions(app, cx, actions);
    tab_pageflip::handle_actions(app, cx, actions);
    tab_radiobutton::handle_actions(app, cx, actions);
}

