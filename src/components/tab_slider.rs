use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    use crate::components::layout_templates::*;

    pub DemoSlider = <UIZooTabLayout_B> {
        desc = {
            <Markdown> { body: dep("crate://self/resources/slider.md") }
        } // -- desc
        demos = {
            <H4> { text: "Slider" }
            <Slider> { text: "Default" }

            <Slider> {
                text: "Default, disabled"
                animator: { disabled = { default: on } }
            }
            <Slider> { text: "label_align", label_align: { x: 0.5, y: 0. } }
            <Slider> { text: "min/max", min: 0., max: 100. }
            <Slider> { text: "precision", precision: 20 }
            <Slider> { text: "stepped", step: 0.1 }

            <Hr> {}
            <H4> { text: "SliderGradientY"}
            <SliderGradientY> { text: "Default", draw_bg: { color: #f0f } }
            <SliderGradientY> {
                text: "disabled"
                animator: { disabled = { default: on } }
            }
            <SliderGradientY> { text: "label_align", label_align: { x: 0.5, y: 0. } }
            <SliderGradientY> { text: "min/max", min: 0., max: 100. }
            <SliderGradientY> { text: "precision", precision: 20 }
            <SliderGradientY> { text: "stepped", step: 0.1 }

            <Hr> {}
            <H4> { text: "SliderGradientX"}
            <SliderGradientX> { text: "Default", draw_bg: { color: #f0f } }
            <SliderGradientX> {
                text: "disabled"
                animator: { disabled = { default: on } }
            }
            <SliderGradientX> { text: "label_align", label_align: { x: 0.5, y: 0. } }
            <SliderGradientX> { text: "min/max", min: 0., max: 100. }
            <SliderGradientX> { text: "precision", precision: 20 }
            <SliderGradientX> { text: "stepped", step: 0.1 }

            <Hr> {}
            <H4> { text: "SliderFlat"}
            <SliderFlat> { text: "Default" }
            <SliderFlat> {
                text: "disabled"
                animator: { disabled = { default: on } }
            }
            <SliderFlat> { text: "label_align", label_align: { x: 0.5, y: 0. } }
            <SliderFlat> { text: "min/max", min: 0., max: 100. }
            <SliderFlat> { text: "precision", precision: 20 }
            <SliderFlat> { text: "stepped", step: 0.1 }

            <Hr> {}
            <H4> { text: "SliderMinimal"}
            <SliderMinimal> { text: "Default" }
            <SliderMinimal> {
                text: "Default, disabled"
                animator: { disabled = { default: on } }
            }
            <SliderMinimal> { text: "label_align", label_align: { x: 0.5, y: 0. } }
            <SliderMinimal> { text: "min/max", min: 0., max: 100. }
            <SliderMinimal> { text: "precision", precision: 20 }
            <SliderMinimal> { text: "stepped", step: 0.1 }

            <Hr> {}
            <H4> { text: "SliderMinimalFlat"}
            <SliderMinimalFlat> { text: "Default" }
            <SliderMinimalFlat> {
                text: "disabled"
                animator: { disabled = { default: on } }
            }
            <SliderMinimalFlat> { text: "label_align", label_align: { x: 0.5, y: 0. } }
            <SliderMinimalFlat> { text: "min/max", min: 0., max: 100. }
            <SliderMinimalFlat> {
                text: "Big",
                padding: 5.
                height: 100.
                min: 0., max: 100.
                draw_bg: {
                   offset_y: 0.,
                   handle_size: 10.,
                }
            }
            <SliderMinimalFlat> { text: "precision", precision: 20 }
            <SliderMinimalFlat> { text: "stepped", step: 0.1 }

            <Hr> {}
            <H4> { text: "SliderRound"}
            <SliderRound> { text: "Default" }
            <SliderRound> {
                text: "Disabled"
                animator: { disabled = { default: on } }
            }
            <SliderRound> {
                text: "Solid",
                draw_text: { color: #0ff }
                draw_bg: {
                    val_color: #F08
                    val_color_hover: #F4A
                    val_color_focus: #C04
                    val_color_drag: #F08

                    val_color_2: #F08
                    val_color_2_hover: #F4A
                    val_color_2_focus: #C04
                    val_color_2_drag: #F08

                    handle_color: #F
                    handle_color_hover: #F
                    handle_color_focus: #F
                    handle_color_drag: #F
                }
            }
            <SliderRound> {
                text: "Solid",
                draw_bg: {
                    val_color: #6,
                    val_color_2: #6,
                    handle_color: #0,
                }
            }
            <SliderRound> { text: "min/max", min: 0., max: 100. }
            <SliderRound> { text: "precision", precision: 20 }
            <SliderRound> { text: "stepped", step: 0.1 }
            <SliderRound> {
                text: "label_size",
                draw_bg: {label_size: 150. },
            }

            <Hr> {}
            <H4> { text: "SliderRoundGradientY"}
            <SliderRoundGradientY> { text: "min/max", min: 0., max: 100. }
            <SliderRoundGradientY> {
                text: "min/max"
                animator: { disabled = { default: on } }
            }
            <SliderRoundGradientY> { text: "precision", precision: 20 }
            <SliderRoundGradientY> { text: "stepped", step: 0.1, draw_bg: { color: #f0f }}

            <Hr> {}
            <H4> { text: "SliderRoundGradientX"}
            <SliderRoundGradientX> { text: "min/max", min: 0., max: 100. }
            <SliderRoundGradientX> {
                text: "min/max"
                animator: { disabled = { default: on } }
            }
            <SliderRoundGradientX> { text: "precision", precision: 20 }
            <SliderRoundGradientX> { text: "stepped", step: 0.1, draw_bg: { color: #f0f }}

            <Hr> {}
            <H4> { text: "SliderRoundFlat"}
            <SliderRoundFlat> { text: "min/max", min: 0., max: 100. }
            <SliderRoundFlat> {
                text: "disabled"
                animator: { disabled = { default: on } }
            }
            <SliderRoundFlat> { text: "precision", precision: 20 }
            <SliderRoundFlat> { text: "stepped", step: 0.1 }

            <Hr> {}
            <H4> { text: "Styling Attributes Reference" }
            <Slider> {
                text: "Slider"
                height: 36;

                min: 0.0, max: 1.0,
                step: 0.0,
                label_align: { x: 0., y: 0. }
                margin: <THEME_MSPACE_1> { top: (THEME_SPACE_2) }
                precision: 2,

                draw_text: {
                    color: (THEME_COLOR_LABEL_OUTER)
                    color_hover: (THEME_COLOR_LABEL_OUTER_HOVER)
                    color_drag: (THEME_COLOR_LABEL_OUTER_DRAG)
                    color_focus: (THEME_COLOR_LABEL_OUTER_FOCUS)
                    color_disabled: (THEME_COLOR_LABEL_OUTER_DISABLED)
                    color_empty: (THEME_COLOR_TEXT_PLACEHOLDER)

                    text_style: {
                        line_spacing: (THEME_FONT_WDGT_LINE_SPACING),
                        font_size: (THEME_FONT_SIZE_P)
                    }
                }

                label_walk: {
                    width: Fill, height: Fit,
                    margin: { top: 0., bottom: (THEME_SPACE_1) },
                }

                text_input: <TextInput> {
                    empty_text: "0",
                    is_numeric_only: true,
                    is_read_only: false,

                    width: Fit,
                    label_align: {y: 0.},
                    margin: 0.
                    padding: 0.

                    draw_text: {
                        color: (THEME_COLOR_TEXT_VAL)
                        color_hover: (THEME_COLOR_TEXT_HOVER)
                        color_focus: (THEME_COLOR_TEXT_FOCUS)
                        color_down: (THEME_COLOR_TEXT_DOWN)
                        color_disabled: (THEME_COLOR_TEXT_DISABLED)
                        color_empty: (THEME_COLOR_TEXT_PLACEHOLDER)
                        color_empty_hover: (THEME_COLOR_TEXT_PLACEHOLDER_HOVER)
                        color_empty_focus: (THEME_COLOR_TEXT_FOCUS)
                    }

                    draw_bg: {
                        border_radius: 0.
                        border_size: 0.

                        color: (THEME_COLOR_U_HIDDEN)
                        color_hover: (THEME_COLOR_U_HIDDEN)
                        color_focus: (THEME_COLOR_U_HIDDEN)
                        color_disabled: (THEME_COLOR_U_HIDDEN)
                        color_empty: (THEME_COLOR_U_HIDDEN)

                        border_color: (THEME_COLOR_U_HIDDEN)
                        border_color_hover: (THEME_COLOR_U_HIDDEN)
                        border_color_empty: (THEME_COLOR_U_HIDDEN)
                        border_color_disabled: (THEME_COLOR_U_HIDDEN)
                        border_color_focus: (THEME_COLOR_U_HIDDEN)

                        border_color_2: (THEME_COLOR_U_HIDDEN)
                        border_color_2_hover: (THEME_COLOR_U_HIDDEN)
                        border_color_2_empty: (THEME_COLOR_U_HIDDEN)
                        border_color_2_disabled: (THEME_COLOR_U_HIDDEN)
                        border_color_2_focus: (THEME_COLOR_U_HIDDEN)
                    }

                    draw_cursor: { color: (THEME_COLOR_TEXT_CURSOR) }

                    draw_selection: {
                        border_radius: (THEME_TEXTSELECTION_CORNER_RADIUS)

                        color: (THEME_COLOR_D_HIDDEN)
                        color_hover: (THEME_COLOR_D_HIDDEN)
                        color_focus: (THEME_COLOR_D_HIDDEN)
                        color_empty: (THEME_COLOR_U_HIDDEN)
                        color_disabled: (THEME_COLOR_U_HIDDEN)
                    }
                }

                draw_bg: {
                    disabled: 0.0,

                    border_size: (THEME_BEVELING)
                    border_radius: (THEME_CORNER_RADIUS)

                    color_dither: 1.0

                    color: (THEME_COLOR_INSET)
                    color_hover: (THEME_COLOR_INSET_HOVER)
                    color_focus: (THEME_COLOR_INSET_FOCUS)
                    color_disabled: (THEME_COLOR_INSET_DISABLED)
                    color_drag: (THEME_COLOR_INSET_DRAG)

                    handle_color: (THEME_COLOR_HANDLE_1)
                    handle_color_hover: (THEME_COLOR_HANDLE_1_HOVER)
                    handle_color_focus: (THEME_COLOR_HANDLE_1_FOCUS)
                    handle_color_disabled: (THEME_COLOR_HANDLE_1_DISABLED)
                    handle_color_drag: (THEME_COLOR_HANDLE_1_DRAG)

                    handle_color_2: (THEME_COLOR_HANDLE_2)
                    handle_color_2_hover: (THEME_COLOR_HANDLE_2_HOVER)
                    handle_color_2_focus: (THEME_COLOR_HANDLE_2_FOCUS)
                    handle_color_2_disabled: (THEME_COLOR_HANDLE_2_DISABLED)
                    handle_color_2_drag: (THEME_COLOR_HANDLE_2_DRAG)

                    border_color: (THEME_COLOR_BEVEL_INSET_2)
                    border_color_hover: (THEME_COLOR_BEVEL_INSET_2_HOVER)
                    border_color_focus: (THEME_COLOR_BEVEL_INSET_2_FOCUS)
                    border_color_disabled: (THEME_COLOR_BEVEL_INSET_2_DISABLED)
                    border_color_drag: (THEME_COLOR_BEVEL_INSET_2_DRAG)

                    border_color_2: (THEME_COLOR_BEVEL_INSET_1)
                    border_color_2_hover: (THEME_COLOR_BEVEL_INSET_1_HOVER)
                    border_color_2_focus: (THEME_COLOR_BEVEL_INSET_1_FOCUS)
                    border_color_2_disabled: (THEME_COLOR_BEVEL_INSET_1_DISABLED)
                    border_color_2_drag: (THEME_COLOR_BEVEL_INSET_1_DRAG)

                    val_padding: 3.

                    val_color: (THEME_COLOR_VAL)
                    val_color_hover: (THEME_COLOR_VAL_HOVER)
                    val_color_focus: (THEME_COLOR_VAL_FOCUS)
                    val_color_disabled: (THEME_COLOR_VAL_DISABLED)
                    val_color_drag: (THEME_COLOR_VAL_DRAG)

                    handle_size: 20.
                    bipolar: 0.0,
                }
            }

        } // -- demos
    }
}

