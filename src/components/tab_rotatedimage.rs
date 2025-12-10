use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    use crate::components::layout_templates::*;

    pub DemoRotatedImage = <UIZooTabLayout_B> {
        desc = {
            <Markdown> { body: dep("crate://self/resources/rotatedimage.md") }
        } // -- desc
        demos = {
            <RotatedImage> {
                width: Fill, height: Fill
                draw_bg: {
                    scale: 1.
                    rotation: 45.
                    opacity: 0.25
                }
                source: dep("crate://self/resources/ducky.png")
            }
        } // -- demos
    }
}

