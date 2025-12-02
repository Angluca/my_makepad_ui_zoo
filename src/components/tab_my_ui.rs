use makepad_widgets::*;
use crate::app::App;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    use crate::components::layout_templates::*;

    pub MyUI = <UIZooTabLayout_B> {
        desc = {
            //<Markdown> { body: dep("crate://self/resources/icon.md") }
            <H1> { text: "Test desc"}
        }
        demos = {
            <H4> { text: "Test demo"}
            my_ui_btn = <Button> { text: "Click me Click me !" }
        }

    }
}

pub fn handle_actions(app: &mut App, _cx: &mut Cx, actions:&Actions) {
    if app.ui.button(ids!(my_ui_btn)).clicked(&actions) {
        log!("My UI button test: Hello World!");
    }
}
