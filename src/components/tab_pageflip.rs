use makepad_widgets::*;
use crate::app::App;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    use crate::components::layout_templates::*;

    Page = <View> {
        width: Fill, height: Fill
        align: { x: 0.5, y: 0.5 }
        show_bg: true
    }

    pub DemoPageFlip = <UIZooTabLayout_B> {
        desc = {
            <Markdown> { body: dep("crate://self/resources/pageflip.md") }
        } // -- desc
        demos = {
            <View> {
                width: Fill, height: Fit
                flow: Right
                spacing: (THEME_SPACE_2)
                pageflip_button_a = <Button> { text: "Page A" }
                pageflip_button_b = <Button> { text: "Page B" }
                pageflip_button_c = <Button> { text: "Page C" }
            }

            page_flip = <PageFlip> {
                width: Fill, height: Fill
                flow: Down
                active_page: page_a

                page_a = <Page> {
                    <H3> { width: Fit, text: "Page A" }
                    draw_bg: { color: #f00 }
                }

                page_b = <Page> {
                    <H3> { width: Fit, text: "Page B" }
                    draw_bg: { color: #080 }
                }

                page_c = <Page> {
                    <H3> { width: Fit, text: "Page C" }
                    draw_bg: { color: #008 }
                }
            }
        } // -- demos
    }
}

pub fn handle_actions(app: &mut App, cx: &mut Cx, actions:&Actions) {
    if app.ui.button(ids!(pageflip_button_a)).clicked(&actions) {
        app.ui.page_flip(ids!(page_flip)).set_active_page(cx, live_id!(page_a));
    }
    else if app.ui.button(ids!(pageflip_button_b)).clicked(&actions) {
        app.ui.page_flip(ids!(page_flip)).set_active_page(cx, live_id!(page_b));
    }
    else if app.ui.button(ids!(pageflip_button_c)).clicked(&actions) {
        app.ui.page_flip(ids!(page_flip)).set_active_page(cx, live_id!(page_c));
    }
}

