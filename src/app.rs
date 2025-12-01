//#![allow(unused)]
use makepad_widgets::*;
use crate::components::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    use crate::components::tab_overview::*;

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
                caption_label = { label = {text: "Makepad UI Zoo - 中文"} }
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
                        ],
                        selected: 0,
                    }

                    tab_overview = Tab { name: "介绍", template: PermanentTab, kind: TabOverView }
                    TabOverView = <UIZooTab> { <OverView> {} }

                }
            }
        }
    }
}

#[derive(Live, LiveHook)]
pub struct App {
    #[live] ui: WidgetRef,
    #[rust] counter: usize
}

impl LiveRegister for App {
    fn live_register(cx: &mut Cx) {
        makepad_widgets::live_design(cx);
        crate::components::live_design(cx);
    }
}

impl MatchEvent for App{
    fn handle_actions(&mut self, cx: &mut Cx, actions:&Actions){
    }
}

impl AppMain for App {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        self.match_event(cx, event);
        self.ui.handle_event(cx, event, &mut Scope::empty());
    }
}
app_main!(App);

