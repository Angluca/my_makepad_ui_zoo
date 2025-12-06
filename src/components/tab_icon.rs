use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    use crate::components::layout_templates::*;
    FAVORITE = dep("crate://self/resources/Icon_favorite.svg")

    pub DemoIcon = <UIZooTabLayout_B> {
        desc = {
            <Markdown> { body: dep("crate://self/resources/icon.md") }
        } // -- desc
        demos = {
            <H4> { text: "Standard" }
            <Icon> {
                draw_icon: { svg_file: (FAVORITE) }
            }

            <Hr> {}
            <H4> { text: "IconGradientX" }
            <IconGradientX> {
                icon_walk: { width: 50. }
                draw_icon: { svg_file: (FAVORITE) }
            }

            <Hr> {}
            <H4> { text: "IconGradientY" }
            <IconGradientY> {
                icon_walk: { width: 50. }
                draw_icon: { svg_file: (FAVORITE) }
            }

            <H4> { text: "Styling Attributes Reference" }
            <Icon> {
                width: Fit, height: Fit

                icon_walk: {
                    width: 200, height: 100
                }

                draw_bg: { color: #22f }
                draw_icon: {
                    svg_file: (FAVORITE)
                    color: #f00
                    color_2: #f0f
                }
            }
        } // -- demos
    }
}

