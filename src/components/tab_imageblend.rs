use makepad_widgets::*;
use crate::app::App;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    use crate::components::layout_templates::*;

    pub DemoImageBlend = <UIZooTabLayout_B> {
        desc = {
            <Markdown> { body: dep("crate://self/resources/imageblend.md") }
        } // -- desc
        demos = {
            <H4> { text: "Standard" }
            blend_button = <Button> { text: "Blend Image" }
            blend_image = <ImageBlend> {
                align: { x: 0, y: 0 }
                image_a: {
                    source: dep("crate://self/resources/ducky.png"),
                    fit: Smallest
                    width: Fill,
                    height: Fill
                }
                image_b: {
                    source: dep("crate://self/resources/ismael-jean-deGBOI6yQv4-unsplash.jpg")
                    fit: Smallest
                    width: Fill,
                    height: Fill
                }
            }
        } // -- demos
    }
}

pub fn handle_actions(app: &mut App, cx: &mut Cx, actions:&Actions) {
    if app.ui.button(ids!(blend_button)).clicked(&actions) {
        app.ui.image_blend(ids!(blend_image)).switch_image(cx);
    }
}

