#![allow(unused)]
use makepad_widgets::*;
use crate::components::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    use crate::components::ui_options::*;
    use crate::components::tab_my_ui::*;

    use crate::components::tab_overview::*;
    use crate::components::tab_layout::*;
    use crate::components::tab_button::*;
    use crate::components::tab_checkbox::*;
    use crate::components::tab_dropdown::*;

    UIZooTab = <RectView> {
        width: Fill, height: Fill,
        flow: Down,
        padding: 0,
        spacing: 0,
    }

    App = {{App}} {
        ui: <Window> {
            width: Fill, height: Fill,
            show_bg: true,
            draw_bg: {
                fn pixel(self) -> vec4 {
                    return (THEME_COLOR_BG_APP);
                }
            }
            caption_bar = {
                visible: true,
                margin: {left: -200},
                caption_label = { label = {text: "Makepad UI Zoo"} }
            },

            body = <View> {
                width: Fill, height: Fill,
                flow: Down,
                spacing: 0.,
                margin: 0.,

                dock = <Dock> {
                    width: Fill, height: Fill,
                    root = Splitter {
                        axis: Horizontal,
                        align: FromA(0.0),
                        a: tab_set_1,
                        b: tab_set_2,
                    }

                    tab_set_1 = Tabs {
                        tabs: [tab_a],
                        selected: 0,
                    }
                    tab_set_2 = Tabs {
                        tabs: [
                            tab_overview,
                            tab_layout,
                            tab_button,
                            tab_checkbox,
                            tab_dropdown,

                            tab_my_ui,
                        ],
                        selected: 0,
                    }

                    TabOverView = <UIZooTab> { <OverView> {} }
                    tab_overview = Tab { name: "Intro", template: PermanentTab, kind: TabOverView }
                    TabLayout = <UIZooTab> { <DemoLayout> {} }
                    tab_layout = Tab { name: "Layout", template: PermanentTab, kind: TabLayout }
                    TabButton = <UIZooTab> { <DemoButton> {} }
                    tab_button = Tab { name: "Button", template: PermanentTab, kind: TabButton }
                    TabCheckBox = <UIZooTab> { <DemoCheckBox> {} }
                    tab_checkbox = Tab { name: "CheckBox", template: PermanentTab, kind: TabCheckBox }
                    TabDropDown = <UIZooTab> { <DemoDropDown> {} }
                    tab_dropdown = Tab { name: "DropDown & PopupMenu", template: PermanentTab, kind: TabDropDown }

                    // -- Tab UI template
                    TabMyUI = <UIZooTab> { <MyUI> {} }
                    tab_my_ui = Tab { name: "My UI", template: PermanentTab, kind: TabMyUI }
                } // -- dock

                <UIOptions> {}

            } // -- body
        } // -- ui
    } // -- app
}

use crate::components::tab_dropdown::DataBindingsForApp;
#[derive(Live, LiveHook)]
pub struct App {
    #[live] pub ui: WidgetRef,
    #[rust] pub counter: usize,
    #[rust(DataBindingsForApp::new(cx))]
    pub bindings: DataBindingsForApp,
}

impl LiveRegister for App {
    fn live_register(cx: &mut Cx) {
        makepad_widgets::live_design(cx);
        crate::components::live_design(cx);
    }
}

impl MatchEvent for App{
    fn handle_actions(&mut self, cx: &mut Cx, actions:&Actions) {
        crate::components::handle_actions(self, cx, actions);
    }

    fn handle_startup(&mut self, cx: &mut Cx) {
        crate::components::handle_startup(self, cx);
    }
}

impl AppMain for App {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        self.match_event(cx, event);
        self.ui.handle_event(cx, event, &mut Scope::empty());
    }
}
app_main!(App);

