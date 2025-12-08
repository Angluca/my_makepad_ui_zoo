use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    use crate::components::layout_templates::*;

    FAVORITE = dep("crate://self/resources/Icon_Favorite.svg")
    pub DemoLinkLabel = <UIZooTabLayout_B> {
        desc = {
            <Markdown> { body: dep("crate://self/resources/linklabel.md") }
        } // -- desc
        demos = {
            <H4> { text: "Standard" }
            <UIZooRowH> {
                <LinkLabel> { text: "Click me!" }
            }

            <Hr> {}
            <H4> { text: "Standard, disabled" }
            <UIZooRowH> {
                <LinkLabel> {
                    text: "Click me!"
                    animator: {
                        disabled = { default: on }
                    }
                }
            }

            <Hr> {}
            <H4> { text: "LinkLabelGradientX" }
            <UIZooRowH> {
                <LinkLabelGradientX> { text: "<LinkLabelGradientX>" }
            }

            <Hr> {}
            <H4> { text: "LinkLabelGradientY" }
            <UIZooRowH> {
                <LinkLabelGradientY> { text: "<LinkLabelGradientY>" }
            }

            <Hr> {}
            <H4> { text: "LinkLabelIcon" }
            <UIZooRowH> {
                <LinkLabelIcon> {
                    text: "Click me!"
                    draw_icon: {
                        color: #f00
                        svg_file: (FAVORITE)
                    }
                    icon_walk: {
                        width: 12.5, height: Fit
                        margin: 0.
                    }
                }
                <LinkLabelIcon> {
                    text: "Click me!"
                    draw_icon: {
                        svg_file: (FAVORITE)
                        gradient_fill_horizontal: 1.0

                        color: #f00
                        color_hover: (THEME_COLOR_LABEL_INNER_HOVER)
                        color_down: (THEME_COLOR_LABEL_INNER_DOWN)
                        color_focus: (THEME_COLOR_LABEL_INNER_FOCUS)
                        color_disabled: (THEME_COLOR_LABEL_INNER_DISABLED)

                        color_2: #0ff
                        color_2_hover: (THEME_COLOR_LABEL_INNER_HOVER)
                        color_2_down: (THEME_COLOR_LABEL_INNER_DOWN)
                        color_2_focus: (THEME_COLOR_LABEL_INNER_FOCUS)
                        color_2_disabled: (THEME_COLOR_LABEL_INNER_DISABLED)
                    }
                    icon_walk: {
                        width: 12.5, height: Fit
                        margin: 0.
                    }
                }
            }

            <Hr> {}
            <H4> { text: "Styling Attributes Reference" }
            <UIZooRowH> {
                <LinkLabel> {
                    text: "Click me!"
                    label_walk: {
                        width: Fit, height: Fit
                        margin: { left: 10. }
                    }

                    draw_text: {
                        gradient_fill_horizontal: 0.3
                        color: #A00, color_2: #00A
                        color_hover: #C
                        color_down: #8
                        color_focus: #B
                        color_disabled: #3

                        wrap: Word
                        text_style: {
                            font_size: 20.
                            line_spacing: 1.4
                        }
                    }

                    icon_walk: {
                        width: 30., height: Fit
                    }

                    draw_icon: {
                        svg_file: (FAVORITE)
                        color: #A00
                        color_hover: #C00
                        color_down: #800
                        color_focus: #B00
                        color_disabled: #300
                    }
                }
            }
        } // -- demos
    }
}

