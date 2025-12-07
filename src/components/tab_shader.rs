use makepad_widgets::*;
use crate::app::App;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    use crate::components::layout_templates::*;

    pub DemoShader = <UIZooTabLayout_B> {
        desc = {
            //<Markdown> { body: dep("crate://self/resources/icon.md") }
            <H2> { text: "Shader desc" }
        } // -- desc
        demos = {
            <H4> { text: "Shader" }
            show_bg: true,
            padding: 3.,
            flow: Overlay,
            shader_btn = <ButtonFlat> {
                margin: {left: 20, top: 100}
                icon_walk: { width: 20. }
                draw_icon: {
                    color: #f00,
                    svg_file: dep("crate://self/resources/Icon_Favorite.svg"),
                }
                draw_bg: {
                    color: #1088,
                }
                text: "Test button",
            }

            draw_bg: {
                fn random2(p: vec2) -> vec2 {
                    return fract(sin(vec2(dot(p, vec2(127.1, 311.7)), dot(p, vec2(269.5, 183.3)))) * 43758.5453)
                }
                fn pixel(self) -> vec4 {
                    let st = self.pos;
                    st.x *= self.rect_size.x / self.rect_size.y;
                    let color = vec3(0.);
                    st *= 3.;
                    let i_st = floor(st);
                    let f_st = fract(st);

                    let m_dist = 1.;
                    for y in 0..2 {
                        for x in 0..2 {
                            let neighbor = vec2(float(x) - 1.0, float(y) - 1.0);
                            let point = random2(i_st + neighbor);
                            point = 0.5 + 0.5 * sin(self.time + 6.2831 * point);
                            let diff = neighbor + point - f_st;
                            let dist = length(diff);
                            m_dist = min(m_dist, dist);
                        }
                    }
                    color += m_dist;
                    color += 1.- step(0.02, m_dist);
                    color.r += step(0.98, f_st.x) + step(0.98, f_st.y);
                    if sin(self.time) > 0. {
                        color -= step(0.7, abs(sin(27.0 * m_dist))) * 0.5;
                    }

                    let sdf = Sdf2d::viewport(self.pos);
                    sdf.circle(0.5,0.5,0.5);
                    sdf.fill(vec4(color, 1.0));
                    return sdf.result;
                }
            }
        } // -- demos
    }
}

pub fn handle_actions(app: &mut App, cx: &mut Cx, actions:&Actions) {
    if app.ui.button(ids!(shader_btn)).clicked(&actions) {
        log!("Shader Button Clicked {}", app.counter);
        app.counter += 1;
        app.ui.button(ids!(shader_btn))
            .set_text(cx, &format!("Clicky clicky! {}", app.counter));
    }
}
