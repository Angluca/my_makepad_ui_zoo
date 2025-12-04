use makepad_widgets::*;
use crate::app::App;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    use crate::components::layout_templates::*;

    pub MyUI = <UIZooTabLayout_B> {
        desc = {
            //<Markdown> { body: dep("crate://self/resources/icon.md") }
            <H1> { text: "Test desc"}
        } // -- desc
        demos = {
            <H4> { text: "Test demo"}
            my_ui_btn = <Button> { text: "Click me Click me !" }
            <RoundedView> {
                show_bg: true,
                padding: 3.,
                flow: Overlay,
                <ButtonFlat> {
                    margin: {left: 0, top: 30}
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
                    color: #f08,
                    fn pixel(self) -> vec4 {
                        let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                        sdf.circle(100, 120, 100.5);
                        sdf.fill_keep(#808);
                        sdf.hexagon(100, 200, 60);
                        sdf.glow_keep(#220, 4);
                        sdf.hexagon(130, 200, 80);
                        //sdf.union();
                        sdf.intersect();
                        //sdf.subtract();
                        sdf.fill_premul(#00f);
                        //sdf.stroke(#008, 4);
                        sdf.rotate(0.3, 200, 400)
                        sdf.translate(-20, 50);
                        sdf.scale(1.6, -50, 50);

                        sdf.move_to(030, 020);
                        sdf.hline(150, 6);
                        sdf.line_to(600, 000);
                        sdf.line_to(300, 200);
                        sdf.close_path();
                        sdf.fill_keep(mix(mix(#f00, #0f0, self.pos.x), mix(#2f2, #00f, self.pos.x), self.pos.x));
                        sdf.rect(400,0, 200, 300);
                        sdf.stroke_keep(#288, 2);
                        sdf.rect(120,100, 500, 100);
                        sdf.gloop(20);
                        sdf.glow(#228, 4);
                        return sdf.result;
                    }
                }
            }

        } // -- demos
    }
}

pub fn handle_actions(app: &mut App, cx: &mut Cx, actions:&Actions) {
    if app.ui.button(ids!(my_ui_btn)).clicked(&actions) {
        log!("MyUI Button Clicked {}", app.counter);
        app.counter += 1;
        app.ui.button(ids!(my_ui_btn))
            .set_text(cx, &format!("Clicky clicky! {}", app.counter));
    }
}
