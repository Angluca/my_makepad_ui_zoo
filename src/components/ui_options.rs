use makepad_widgets::*;
use crate::app::App;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    pub UIOptions = <View> {
        width: Fill, height: Fit,
        flow: Right,
        spacing: (THEME_SPACE_2),
        align: {x: 0, y: 0}
        padding: <THEME_MSPACE_2> {},

        show_bg: true,
        draw_bg: {
            uniform color_dither: 1.0
            uniform border_radius: 0.
            uniform border_size: (THEME_BEVELING)
            uniform color_1: (THEME_COLOR_BG_APP * 0.9);
            uniform color_2: #282828;

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size)
                let dither = Math::random_2d(self.pos.xy) * 0.04 * self.color_dither;
                sdf.rect(-2., 1., self.rect_size.x + 2., self.rect_size.y + 30.);
                sdf.fill_keep( (THEME_COLOR_BG_HIGHLIGHT) );
                sdf.stroke( mix((THEME_COLOR_BEVEL_OUTSET_1), #fff0,
                        pow(self.pos.y, 0.1)), self.border_size );
                return sdf.result
            }
        }

        theme_contrast = <Slider> { text: "Contrast", default: 1.0, min: 0.5 max: 2.0 }
        <View> {
            flow: Down
            spacing: 0.
            <Label> { margin: {top: (THEME_SPACE_1)}, padding: 0., width: Fit, text: "Tint Color"}
            theme_tint_color = <TextInput> { empty_text: "Hex color", text: "00f" }
        }
        theme_tint_amount = <Slider> { text: "Tint Amount", default: 0.0, min:0.0 max: 1.0  }
        <Vr> {}
        <Pbold> {
            width: Fit,
            text: "Font"
        }
        theme_font_size = <Slider> { text: "Size", default: 10.0, min: 3.0 max: 12.0  }
        theme_font_size_contrast = <Slider> { text: "Size Contrast", default: 2.5, min:1.0 max: 5.0  }
        <Vr> {}
        theme_bevel = <Slider> { text: "Bevel", default: 0.75, min:0.0 max: 1.5   }
        theme_rounding = <Slider> { text: "Rounding", default: 2.5, min: 1.0 max: 2.5   }
        theme_space = <Slider> { text: "Space", default: 6.0, min: 1.0 max: 10.0   }

    }
}

pub fn handle_actions(app: &mut App, cx: &mut Cx, actions:&Actions) {
    for slider in [
        (ids!(theme_contrast), live_id!(THEME_COLOR_CONTRAST)),
        (ids!(theme_tint_amount), live_id!(THEME_COLOR_TINT_AMOUNT)),
        (ids!(theme_font_size), live_id!(THEME_FONT_SIZE_BASE)),
        (ids!(theme_font_size_contrast), live_id!(THEME_FONT_SIZE_CONTRAST)),
        (ids!(theme_bevel), live_id!(THEME_BEVELING)),
        (ids!(theme_rounding), live_id!(THEME_CORNER_RADIUS)),
        (ids!(theme_space), live_id!(THEME_SPACE_FACTOR)),
    ] {
        if let Some(value) = app.ui.slider(slider.0).end_slide(&actions) {
            cx.set_dsl_value(
                live_id!(makepad_widgets),
                live_id!(theme_desktop_dark),
                slider.1,
                LiveValue::Float64(value)
            );
            cx.reload_ui_dsl();
        }
    }
}

