use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    use crate::components::layout_templates::*;

    pub DemoSpinner = <UIZooTabLayout_B> {
        desc = {
            //<Markdown> { body: dep("crate://self/resources/icon.md") }
        } // -- desc
        demos = {
            <H4> { text: "LoadingSpinner" }
            <LoadingSpinner> {
                draw_bg: {
                    rotation_speed: 0.8
                    border_size: 10.0
                    max_gap_ratio: 0.92
                    min_gap_ratio: 0.12
                    stroke_width: 80.0
                }
            }
        } // -- demos
    }
}

