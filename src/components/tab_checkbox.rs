use makepad_widgets::*;
use crate::app::App;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    use crate::components::layout_templates::*;

    pub DemoCheckBox = <UIZooTabLayout_B> {
        desc = {
            <Markdown> { body: dep("crate://self/resources/checkbox.md") }
        } // -- desc
        demos = {
            <H4> { text: "CheckBox"}
            <CheckBox> { text: "<CheckBox>" }
            <Hr> {}
            <H4> { text: "CheckBox, disabled" }
            <CheckBox> {
                text: "<CheckBox>",
                animator: {
                    disabled = { default: on }
                }
            }

            <Hr> {}
            <H4> { text: "CheckBoxFlat" }
            <CheckBoxFlat> { text: "<CheckBoxFlat>" }

            <Hr> {}
            <H4> { text: "CheckBoxGradientX" }
            <CheckBoxGradientX> {
                text: "<CheckBoxGradientX>",
            }

            <Hr> {}
            <H4> { text: "CheckBoxGradientY" }
            <CheckBoxGradientY> {
                text: "<CheckBoxGradientY>"
                draw_bg: {
                    color: #f00,
                    color_2: #00f
                }
            }

            <Hr> {}
            <H4> { text: "Toggle" }
            <Toggle> { text: "<Toggle>" }

            <Hr> {}
            <H4> { text: "ToggleFlat" }
            <ToggleFlat> { text: "<ToggleFlat>" }

            <Hr> {}
            <H4> { text: "ToggleGradientX" }
            <ToggleGradientX> {
                text: "<ToggleGradientX>"
                draw_bg: {
                    color: #f00,
                    color_2: #00f
                }
            }

            <Hr> {}
            <H4> { text: "ToggleGradientY" }
            <ToggleGradientY> {
                text: "<ToggleGradientY>"
                draw_bg: {
                    color: #f00,
                    color_2: #00f
                }
            }

            <Hr> {}
            <H4> { text: "Output Demo" }
            <UIZooRowH> {
                height: Fit,
                flow: Right,
                align: { x: 0., y: 0.5 }
                simple_checkbox = <CheckBox> { text: "<CheckBox>" }
                simple_check_output = <Label> { text: "what?" }
            }
            <Hr> {}
            <H4> { text: "Custom Checkbox" }
            <UIZooRowH> {
                <CheckBoxCustom> {
                    text: "<CheckBoxCustom>",
                    align: { x: 0., y: 0.5 }
                    padding: 0.,
                    margin: 0.,

                    label_walk: {
                        width: Fit, height: Fit,
                        margin: <THEME_MSPACE_1> { left: 5.5 }
                    }

                    draw_icon: {
                        //gradient_border_horizontal: 1.0;
                        gradient_fill_horizontal: 1.0;
                        color: #2,
                        color_2: #B,
                        color_active:#f00,
                        color_2_active: #00f
                        //color_disabled: #6,
                        svg_file: dep("crate://self/resources/Icon_Favorite.svg")
                    },
                    icon_walk: {
                        width: 20.0, height:Fit
                    }
                }
            }

            <Hr> {}
            <H4> { text: "Styling Attributes Reference"}
            <UIZooRowH> {
                <CheckBox> {
                    text: "<CheckBox>中文\nHello World! 你好 世界!",
                    width: Fit, height: Fit,
                    padding: <THEME_MSPACE_2> {}
                    align: { x: 0, y: 0 }

                    label_walk: {
                        width: Fit, height: Fit
                        margin: <THEME_MSPACE_H_1> { left: 30. }
                    }

                    draw_bg: {
                        check_type: Check
                        size: 30.

                        border_size: (THEME_BEVELING)
                        border_radius: (THEME_CORNER_RADIUS)

                        color_dither: 1.0
                        color: (THEME_COLOR_INSET)
                        color_hover: (THEME_COLOR_INSET_HOVER)
                        color_down: (THEME_COLOR_INSET_DOWN)
                        color_active: (THEME_COLOR_INSET_ACTIVE)
                        color_focus: (THEME_COLOR_INSET_FOCUS)
                        color_disabled: (THEME_COLOR_INSET_DISABLED)

                        border_color: (THEME_COLOR_BEVEL_INSET_2)
                        border_color_hover: (THEME_COLOR_BEVEL_INSET_2_HOVER)
                        border_color_down: (THEME_COLOR_BEVEL_INSET_2_DOWN)
                        border_color_active: (THEME_COLOR_BEVEL_INSET_2_ACTIVE)
                        border_color_focus: (THEME_COLOR_BEVEL_INSET_2_FOCUS)
                        border_color_disabled: (THEME_COLOR_BEVEL_INSET_2_DISABLED)

                        border_color_2: (THEME_COLOR_BEVEL_INSET_1)
                        border_color_2_hover: (THEME_COLOR_BEVEL_INSET_1_HOVER)
                        border_color_2_down: (THEME_COLOR_BEVEL_INSET_1_DOWN)
                        border_color_2_active: (THEME_COLOR_BEVEL_INSET_1_ACTIVE)
                        border_color_2_focus: (THEME_COLOR_BEVEL_INSET_1_FOCUS)
                        border_color_2_disabled: (THEME_COLOR_BEVEL_INSET_1_DISABLED)

                        mark_size: 0.65
                        mark_color: (THEME_COLOR_U_HIDDEN)
                        mark_color_hover: (THEME_COLOR_U_HIDDEN)
                        mark_color_down: (THEME_COLOR_U_HIDDEN)
                        mark_color_active: (THEME_COLOR_MARK_ACTIVE)
                        mark_color_active_hover: (THEME_COLOR_MARK_ACTIVE_HOVER)
                        mark_color_focus: (THEME_COLOR_MARK_FOCUS)
                        mark_color_disabled: (THEME_COLOR_MARK_DISABLED)
                    }

                    draw_text: {
                        color: (THEME_COLOR_LABEL_OUTER)
                        color_hover: (THEME_COLOR_LABEL_OUTER_HOVER)
                        color_down: (THEME_COLOR_LABEL_OUTER_DOWN)
                        color_focus: (THEME_COLOR_LABEL_OUTER_FOCUS)
                        color_active: (THEME_COLOR_LABEL_OUTER_ACTIVE)
                        color_disabled: (THEME_COLOR_LABEL_OUTER_DISABLED)

                        text_style: {
                            font_size: (THEME_FONT_SIZE_P)
                            line_spacing: 1.2
                        }
                    }
                }
            }
        } // -- demos
    }
}

pub fn handle_actions(app: &mut App, cx: &mut Cx, actions:&Actions) {
    if let Some(check) = app.ui.check_box(ids!(simple_checkbox)).changed(actions) {
        log!("CHECK BUTTON CLICKED {} {}", app.counter, check);
        app.counter += 1;
        app.ui.label(ids!(simple_check_output))
            .set_text(cx, &format!("{} {}" , app.counter, check));
    }
}
