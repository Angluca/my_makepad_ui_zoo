use makepad_widgets::*;
use crate::app::App;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    use crate::components::layout_templates::*;
    FAVORIT = dep("crate://self/resources/Icon_Favorite.svg")

    pub DemoRadioButton = <UIZooTabLayout_B> {
        desc = {
            <Markdown> { body: dep("crate://self/resources/radiobutton.md") }
        } // -- desc
        demos = {
            <H4> { text: "Default" }
            <UIZooRowH> {
                radios_demo_1 = <View> {
                    spacing: (THEME_SPACE_2)
                    width: Fit, height: Fit
                    radio1 = <RadioButton> { text: "Option 1" }
                    radio2 = <RadioButton> { text: "Option 2" }
                    radio3 = <RadioButton> { text: "Option 3" }
                    radio4 = <RadioButton> {
                        text: "Option 4, disabled"
                        animator: { disabled = { default: on } }
                    }
                }
            }

            <Hr> {}
            <H4> { text: "RadioButtonFlat"}
            <UIZooRowH> {
                radios_demo_2 = <View> {
                    spacing: (THEME_SPACE_2)
                    width: Fit, height: Fit,
                    radio1 = <RadioButtonFlat> { text: "Option 1" }
                    radio2 = <RadioButtonFlat> { text: "Option 2" }
                    radio3 = <RadioButtonFlat> { text: "Option 3" }
                    radio4 = <RadioButtonFlat> { text: "Option 4" }
                }
            }

            <Hr> {}
            <H4> { text: "RadioButtonFlatter"}
            <UIZooRowH> {
                radios_demo_3 = <View> {
                    spacing: (THEME_SPACE_2)
                    width: Fit, height: Fit,
                    radio1 = <RadioButtonFlatter> { text: "Option 1" }
                    radio2 = <RadioButtonFlatter> { text: "Option 2" }
                    radio3 = <RadioButtonFlatter> { text: "Option 3" }
                    radio4 = <RadioButtonFlatter> { text: "Option 4" }
                }
            }

            <Hr> {}
            <H4> { text: "RadioButtonGradientX"}
            <UIZooRowH> {
                radios_demo_4 = <View> {
                    spacing: (THEME_SPACE_2)
                    width: Fit, height: Fit,
                    radio1 = <RadioButtonGradientX> {
                        text: "Option 1"
                        draw_bg: {
                            color: #f00
                            color_hover: #ff0
                            color_focus: #ff0
                            color_active: #0f0
                        }
                    }
                    radio2 = <RadioButtonGradientX> { text: "Option 2" }
                    radio3 = <RadioButtonGradientX> { text: "Option 3" }
                    radio4 = <RadioButtonGradientX> { text: "Option 4" }
                }
            }

            <Hr> {}
            <H4> { text: "RadioButtonGradientY"}
            <UIZooRowH> {
                radios_demo_5 = <View> {
                    spacing: (THEME_SPACE_2)
                    width: Fit, height: Fit,
                    radio1 = <RadioButtonGradientY> {
                        text: "Option 1"
                        draw_bg: {
                            color: #f00
                            color_hover: #ff0
                            color_focus: #ff0
                            color_active: #0f0
                        }
                    }
                    radio2 = <RadioButtonGradientY> { text: "Option 2" }
                    radio3 = <RadioButtonGradientY> { text: "Option 3" }
                    radio4 = <RadioButtonGradientY> { text: "Option 4" }
                }
            }

            <H4> { text: "Custom styled marker"}
            radios_demo_6 = <UIZooRowH> {
                radio1 = <RadioButtonCustom> {
                    text: "Option 1"
                    padding: { left: 20. }
                    align: { x: 0., y: 0.5 }

                    label_walk: { margin: { left: 5. } }
                    label_align: { x: 0., y: 0. }

                    icon_walk: { width: 12.5, height: Fit, margin: { left: 0. } }
                    draw_icon: {
                        svg_file: (FAVORIT)
                        color: #A00
                        color_active: #f00
                        color_disabled: #4

                        color_2: #0
                        color_2_active: #f00
                        color_2_disabled: #4
                    }
                }
                radio2 = <RadioButtonCustom> {
                    text: "Option 1"
                    padding: { left: 20. }
                    align: { x: 0., y: 0.5 }

                    label_walk: { margin: { left: 5. } }
                    label_align: { x: 0., y: 0. }

                    icon_walk: { width: 12.5, height: Fit, margin: { left: 0. } }
                    draw_icon: {
                        svg_file: (FAVORIT)
                        color: #A00
                        color_active: #f00
                        color_disabled: #4

                        color_2: #0
                        color_2_active: #f00
                        color_2_disabled: #4
                    }
                }
            }

            <Hr> {}
            <H4> { text: "Button Group"}
            radios_demo_7 = <ButtonGroup> {
                radio1 = <RadioButtonTab> { text: "Option 1" }
                radio2 = <RadioButtonTab> { text: "Option 2" }
                radio3 = <RadioButtonTab> { text: "Option 3" }
                radio4 = <RadioButtonTab> {
                    text: "Option 4, disabled"
                    animator: {
                        disabled = { default: on }
                    }
                }
            }

            <Hr> {}
            <H4> { text: "Button Group Flat"}
            radios_demo_8 = <ButtonGroup> {
                radio1 = <RadioButtonTabFlat> { text: "Option 1" }
                radio2 = <RadioButtonTabFlat> { text: "Option 2" }
                radio3 = <RadioButtonTabFlat> { text: "Option 3" }
                radio4 = <RadioButtonTabFlat> { text: "Option 4" }
            }

            <Hr> {}
            <H4> { text: "Button Group GradientX & Y" }
            radios_demo_9 = <ButtonGroup> {
                width: Fit, height: Fit,
                radio1 = <RadioButtonTabGradientX> {
                    text: "Option 1"
                    draw_bg: { color: #f00 }
                }
                radio2 = <RadioButtonTabGradientX> { text: "Option 2" }
                radio3 = <RadioButtonTabGradientY> {
                    text: "Option 3"
                    draw_bg: { color: #f00 }
                }
                radio4 = <RadioButtonTabGradientY> { text: "Option 4" }
            }

            <Hr> {}
            <H4> { text: "Button Group GradientY styled" }
            radios_demo_10 = <ButtonGroup> {
                radio1 = <RadioButtonTabGradientY> {
                    text: "Option 1"
                    draw_text: {
                        color: #A
                        color_hover: #c
                        color_active: #f
                    }

                    draw_bg: {
                        border_size: (THEME_BEVELING)
                        border_radius: 6.

                        color_dither: 1.0

                        color: #F00
                        color_hover: #F44
                        color_active: #300

                        border_color: #0
                        border_color_hover: #F
                        border_color_active: #8

                        border_color_2: #0
                        border_color_2_hover: #F
                        border_color_2_active: #8
                    }
                }
                radio2 = <RadioButtonTabGradientY> {
                    text: "Option 2"

                    draw_text: {
                        color: #A
                        color_hover: #C
                        color_active: #F
                    }

                    draw_bg: {
                        border_size: (THEME_BEVELING)
                        border_radius: 6.

                        color_dither: 1.0

                        color: #F00
                        color_hover: #F44
                        color_active: #300

                        border_color: #0
                        border_color_hover: #F
                        border_color_active: #8

                        border_color_2: #0
                        border_color_2_hover: #F
                        border_color_2_active: #8
                    }
                }
                radio3 = <RadioButtonTabGradientY> {
                    text: "Option 3"

                    draw_text: {
                        color: #A
                        color_hover: #C
                        color_active: #F
                    }

                    draw_bg: {
                        border_size: (THEME_BEVELING)
                        border_radius: 6.

                        color_dither: 1.0

                        color: #F00
                        color_hover: #F44
                        color_active: #300

                        border_color: #0
                        border_color_hover: #F
                        border_color_active: #8

                        border_color_2: #0
                        border_color_2_hover: #F
                        border_color_2_active: #8
                    }
                }
                radio4 = <RadioButtonTabGradientY> {
                    text: "Option 4"

                    draw_text: {
                        color: #A
                        color_hover: #C
                        color_active: #F
                    }

                    draw_bg: {
                        border_size: (THEME_BEVELING)
                        border_radius: 6.

                        color_dither: 1.0

                        color: #F00
                        color_hover: #F44
                        color_active: #300

                        border_color: #0
                        border_color_hover: #F
                        border_color_active: #8

                        border_color_2: #0
                        border_color_2_hover: #F
                        border_color_2_active: #8
                    }
                }
            }

            <Hr> {}
            <H4> { text: "Button Group GradientX" }
            radios_demo_11 = <ButtonGroup> {
                radio1 = <RadioButtonTabGradientX> { text: "Option 1" }
                radio2 = <RadioButtonTabGradientX> { text: "Option 2" }
                radio3 = <RadioButtonTabGradientX> { text: "Option 3" }
                radio4 = <RadioButtonTabGradientX> { text: "Option 4" }
            }

            <Hr> {}
            <H4> { text: "Button Group GradientX Styled" }
            radios_demo_12 = <ButtonGroup> {
                radio1 = <RadioButtonTabGradientX> {
                    text: "Option 1"

                    draw_text: {
                        color: #A
                        color_hover: #C
                        color_active: #F
                    }

                    draw_bg: {
                        border_size: (THEME_BEVELING)

                        color_dither: 10.0

                        color: #F00
                        color_hover: #F44
                        color_active: #300

                        border_color: #0
                        border_color_hover: #F
                        border_color_active: #8

                        border_color_2: #0
                        border_color_2_hover: #F
                        border_color_2_active: #8
                    }
                }
                radio2 = <RadioButtonTabGradientX> {
                    text: "Option 2"

                    draw_text: {
                        color: #A
                        color_hover: #C
                        color_active: #F
                    }

                    draw_bg: {
                        border_size: (THEME_BEVELING)

                        color_dither: 1.0

                        color: #F00
                        color_hover: #F44
                        color_active: #300

                        border_color: #0
                        border_color_hover: #F
                        border_color_active: #8

                        border_color_2: #0
                        border_color_2_hover: #F
                        border_color_2_active: #8
                    }
                }
                radio3 = <RadioButtonTabGradientX> {
                    text: "Option 3"

                    draw_text: {
                        color: #A
                        color_hover: #C
                        color_active: #F
                    }

                    draw_bg: {
                        border_size: (THEME_BEVELING)

                        color_dither: 1.0

                        color: #F00
                        color_hover: #F44
                        color_active: #300

                        border_color: #0
                        border_color_hover: #F
                        border_color_active: #8

                        border_color_2: #0
                        border_color_2_hover: #F
                        border_color_2_active: #8
                    }
                }
                radio4 = <RadioButtonTabGradientX> {
                    text: "Option 4"

                    draw_text: {
                        color: #A
                        color_hover: #C
                        color_active: #F
                    }

                    draw_bg: {
                        border_size: (THEME_BEVELING)

                        color_dither: 1.0

                        color: #F00
                        color_hover: #F44
                        color_active: #300

                        border_color: #0
                        border_color_hover: #F
                        border_color_active: #8

                        border_color_2: #0
                        border_color_2_hover: #F
                        border_color_2_active: #8
                    }
                }
            }

            <Hr> {}
            <H4> { text: "Styling Attributes Reference" }
            <UIZooRowH> {
                radios_demo_13 = <View> {
                    spacing: (THEME_SPACE_2)
                    width: Fit, height: Fit,
                    radio1 = <RadioButton> {
                        text: "Option 1"

                        width: Fit, height: Fit,
                        align: { x: 0., y: 0. }
                        padding: <THEME_MSPACE_V_2> { left: (THEME_SPACE_2)}

                        icon_walk: { margin: { left: 20. } }

                        label_walk: {
                            width: Fit, height: Fit,
                            margin: <THEME_MSPACE_H_1> { left: 13. }
                        }
                        label_align: { y: 0.0 }

                        draw_bg: {
                            size: 15.0,

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

                            mark_color: (THEME_COLOR_MARK_OFF)
                            mark_color_active: (THEME_COLOR_MARK_ACTIVE)
                            mark_color_disabled: (THEME_COLOR_MARK_DISABLED)
                        }

                        draw_text: {
                            color: (THEME_COLOR_LABEL_OUTER)
                            color_hover: (THEME_COLOR_LABEL_OUTER_HOVER)
                            color_down: (THEME_COLOR_LABEL_OUTER_DOWN)
                            color_active: (THEME_COLOR_LABEL_OUTER_ACTIVE)
                            color_focus: (THEME_COLOR_LABEL_OUTER_FOCUS)
                            color_disabled: (THEME_COLOR_LABEL_OUTER_DISABLED)

                            text_style: {
                                font_size: (THEME_FONT_SIZE_P)
                                line_spacing: 1.2
                            }
                        }

                        draw_icon: {
                            color_dither: 1.0
                            color: (THEME_COLOR_LABEL_OUTER)
                            color_active: (THEME_COLOR_LABEL_OUTER_ACTIVE)
                            color_disabled: (THEME_COLOR_LABEL_OUTER_DISABLED)

                            color_2: (THEME_COLOR_LABEL_OUTER)
                            color_2_active: (THEME_COLOR_LABEL_OUTER_ACTIVE)
                            color_2_disabled: (THEME_COLOR_LABEL_OUTER_DISABLED)
                        }
                    }
                    radio2 = <RadioButton> { text: "Option 2" }
                    radio3 = <RadioButton> { text: "Option 3" }
                    radio4 = <RadioButton> {
                        text: "Option 4, disabled"
                        animator: {
                            disabled = {
                                default: on
                            }
                        }
                    }
                }
            }
        } // -- demos
    }
}

pub fn handle_actions(app: &mut App, cx: &mut Cx, actions:&Actions) {
    let ref ui = app.ui;
    ui.radio_button_set(ids_array!(radios_demo_1.radio1, radios_demo_1.radio2, radios_demo_1.radio3, radios_demo_1.radio4)).selected(cx, actions);
            ui.radio_button_set(ids_array!(radios_demo_2.radio1, radios_demo_2.radio2, radios_demo_2.radio3, radios_demo_2.radio4)).selected(cx, actions);
            ui.radio_button_set(ids_array!(radios_demo_3.radio1, radios_demo_3.radio2, radios_demo_3.radio3, radios_demo_3.radio4)).selected(cx, actions);
            ui.radio_button_set(ids_array!(radios_demo_4.radio1, radios_demo_4.radio2, radios_demo_4.radio3, radios_demo_4.radio4)).selected(cx, actions);
            ui.radio_button_set(ids_array!(radios_demo_5.radio1, radios_demo_5.radio2, radios_demo_5.radio3, radios_demo_5.radio4)).selected(cx, actions);
            ui.radio_button_set(ids_array!(radios_demo_6.radio1, radios_demo_6.radio2, radios_demo_6.radio3, radios_demo_6.radio4)).selected(cx, actions);
            ui.radio_button_set(ids_array!(radios_demo_7.radio1, radios_demo_7.radio2, radios_demo_7.radio3, radios_demo_7.radio4)).selected(cx, actions);
            ui.radio_button_set(ids_array!(radios_demo_8.radio1, radios_demo_8.radio2, radios_demo_8.radio3, radios_demo_8.radio4)).selected(cx, actions);
            ui.radio_button_set(ids_array!(radios_demo_9.radio1, radios_demo_9.radio2, radios_demo_9.radio3, radios_demo_9.radio4)).selected(cx, actions);
            ui.radio_button_set(ids_array!(radios_demo_10.radio1, radios_demo_10.radio2, radios_demo_10.radio3, radios_demo_10.radio4)).selected(cx, actions);
            ui.radio_button_set(ids_array!(radios_demo_11.radio1, radios_demo_11.radio2, radios_demo_11.radio3, radios_demo_11.radio4)).selected(cx, actions);
            ui.radio_button_set(ids_array!(radios_demo_12.radio1, radios_demo_12.radio2, radios_demo_12.radio3, radios_demo_12.radio4)).selected(cx, actions);
            ui.radio_button_set(ids_array!(radios_demo_13.radio1, radios_demo_13.radio2, radios_demo_13.radio3, radios_demo_13.radio4)).selected(cx, actions);
}
