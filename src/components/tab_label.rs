use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    use crate::components::layout_templates::*;

    pub DemoLabel = <UIZooTabLayout_B> {
        desc = {
            <Markdown> { body: dep("crate://self/resources/label.md") }
        } // -- desc
        demos = {
            <H4> { text: "Standard" }
            <Label> { text:"Default single line text" }

            <Hr> {}
            <H4> { text: "LabelGradientX" }
            <LabelGradientX> { text: "<LabelGradientX>" }
            <LabelGradientX> {
                text: "<LabelGradientX>"
                draw_text: {
                    color: #0ff
                    color: #088
                    text_style: { font_size: 20. }
                }
            }

            <Hr> {}
            <H4> { text: "LabelGradientY" }
            <LabelGradientY> { text: "<LabelGradientY>" }
            <LabelGradientY> {
                text: "<LabelGradientY>"
                draw_text: {
                    color: #0ff
                    color: #088
                    text_style: { font_size: 20. }
                }
            }

            <Hr> {}
            <H4> { text: "TextBox" }
            <TextBox> {
                text: "Sed ut perspiciatis unde omnis iste natus error sit voluptatem accusantium doloremque laudantium, totam rem aperiam, eaque ipsa quae ab illo inventore veritatis et quasi architecto beatae vitae dicta sunt explicabo. Nemo enim ipsam voluptatem quia voluptas sit aspernatur aut odit aut fugit, sed quia consequuntur magni dolores eos qui ratione voluptatem sequi nesciunt. Neque porro quisquam est, qui dolorem ipsum quia dolor sit amet, consectetur, adipisci velit, sed quia non numquam eius modi tempora incidunt ut labore et dolore magnam aliquam quaerat voluptatem. Ut enim ad minima veniam, quis nostrum exercitationem ullam corporis suscipit laboriosam, nisi ut aliquid ex ea commodi consequatur? Quis autem vel eum iure reprehenderit qui in ea voluptate velit esse quam nihil molestiae consequatur, vel illum qui dolorem eum fugiat quo voluptas nulla pariatur?"
            }

            <Hr> {}
            <H4> { text: "Typographic System" }
            <H1> { text: "H1 headline" }
            <H1italic> { text: "H1_italic headline" }
            <H2> { text: "H2 headline" }
            <H2italic> { text: "H2_italic headline" }
            <H3> { text: "H3 headline" }
            <H3italic> { text: "H3_italic headline" }
            <H4> { text: "H4 headline" }
            <H4italic> { text: "H4_italic headline" }
            <P> { text: "P copy text" }
            <Pitalic> { text: "P_italic copy text" }
            <Pbold> { text: "P_bold copy text" }
            <Pbolditalic> { text: "P_bold_italic copy text" }

            <Hr> {}
            <H4> { text: "Styling Attributes Reference" }
            <Label> {
                text: "You can style text using colors and fonts"
                draw_text: {
                    color: #0ff
                    wrap: Word
                    text_style: {
                        font_size: 20.,
                        line_spacing: 1.4,
                    }
                }
            }

            <Hr> {}
            <H4> { text: "Custom Shader" }
            <Label> {
                text: "OR EVEN SOME PIXELSHADERS"
                draw_text: {
                    color: (THEME_COLOR_MAKEPAD)
                    fn get_color(self) ->vec4{
                        return mix((THEME_COLOR_MAKEPAD), (THEME_COLOR_U_HIDDEN), self.pos.x)
                    }
                    text_style: { font_size: 40. }
                },
            }
        } // -- demos
    }
}

