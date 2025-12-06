use makepad_widgets::*;
use crate::app::App;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    use crate::components::layout_templates::*;

    pub DemoDropDown = <UIZooTabLayout_B> {
        desc = {
            <Markdown> { body: dep("crate://self/resources/dropdown.md") }
        } // -- desc
        demos = {
            <H4> { text: "Standard" }
            dropdown = <DropDown> {
                labels: ["Value One", "Value Two", "Third", "Fourth Value", "Option E", "Hexagons"],
                values: [ValueOne, ValueTwo, Third, FourthValue, OptionE, Hexagons]
            }

            <H4> { text: "Standard, Position: BelowInput" }
            dropdown_below = <DropDown> {
                popup_menu_position:BelowInput,
                labels: ["Value One", "Value Two", "Third", "Fourth Value", "Option E", "Hexagons"],
                values: [ValueOne, ValueTwo, Third, FourthValue, OptionE, Hexagons]
            }

            <H4> { text: "Standard, disabled" }
            dropdown_disabled = <DropDown> {
                labels: ["Value One", "Value Two", "Third", "Fourth Value", "Option E", "Hexagons"],
                values: [ValueOne, ValueTwo, Third, FourthValue, OptionE, Hexagons]
                animator: {
                    disabled = { default: on }
                }
            }

            <H4> { text: "DropDownFlat" }
            dropdown_flat = <DropDownFlat> {
                labels: ["Value One", "Value Two", "Third", "Fourth Value", "Option E", "Hexagons"],
                values: [ValueOne, ValueTwo, Third, FourthValue, OptionE, Hexagons]
            }

            <H4> { text: "DropDownFlat, Position: BelowInput" }
            dropdown_flat_below = <DropDownFlat> {
                popup_menu_position:BelowInput,
                labels: ["Value One", "Value Two", "Third", "Fourth Value", "Option E", "Hexagons"],
                values: [ValueOne, ValueTwo, Third, FourthValue, OptionE, Hexagons]
            }

            <Hr> {}
            <H4> { text: "DropDownGradientX" }
            dropdown_gradient_x = <DropDownGradientX> {
                labels: ["Value One", "Value Two", "Third", "Fourth Value", "Option E", "Hexagons"],
                values: [ValueOne, ValueTwo, Third, FourthValue, OptionE, Hexagons]
                draw_bg: {
                    color: #8
                    color_hover: #A
                }
            }

            <H4> { text: "DropDownGradientX, Position: BelowInput" }
            dropdown_gradient_x_below = <DropDownGradientX> {
                popup_menu_position: BelowInput,
                labels: ["Value One", "Value Two", "Third", "Fourth Value", "Option E", "Hexagons"],
                values: [ValueOne, ValueTwo, Third, FourthValue, OptionE, Hexagons]
            }

            <Hr> {}
            <H4> { text: "DropDownGradientY" }
            dropdown_gradient_y = <DropDownGradientY> {
                labels: ["Value One", "Value Two", "Third", "Fourth Value", "Option E", "Hexagons"],
                values: [ValueOne, ValueTwo, Third, FourthValue, OptionE, Hexagons]
                draw_bg: {
                    color: #9
                    color_hover: #A
                }
            }

            <H4> { text: "DropDownGradientY, Position: BelowInput" }
            dropdown_gradient_y_below = <DropDownGradientY> {
                popup_menu_position: BelowInput,
                labels: ["Value One", "Value Two", "Third", "Fourth Value", "Option E", "Hexagons"],
                values: [ValueOne, ValueTwo, Third, FourthValue, OptionE, Hexagons]
            }

            <Hr> {}
            <H4> { text: "Styling Attributes Reference" }
            dropdown_customized = <DropDown> {
                labels: ["Value One", "Value Two", "Third", "Fourth Value", "Option E", "Hexagons"],
                values: [ValueOne, ValueTwo, Third, FourthValue, OptionE, Hexagons]
                popup_menu: <PopupMenu> {}
                popup_menu_position: BelowInput
                selected_item: 2

                width: Fit, height: Fit
                align: { x: 0, y: 0 }

                padding: <THEME_MSPACE_1> { left: (THEME_SPACE_2), right: 22.5 }
                margin: <THEME_MSPACE_V_1> {}

                draw_text: {
                    color: (THEME_COLOR_LABEL_INNER)
                    color_hover: (THEME_COLOR_LABEL_INNER_HOVER)
                    color_focus: (THEME_COLOR_LABEL_INNER_FOCUS)
                    color_down: (THEME_COLOR_LABEL_INNER_DOWN)
                    color_disabled: (THEME_COLOR_LABEL_INNER_DISABLED)

                    text_style: <THEME_FONT_BOLD> {
                        font_size: (THEME_FONT_SIZE_4)
                        line_spacing: 1.2
                    }
                }

                draw_bg: {
                    border_size: (THEME_BEVELING)
                    border_radius: (THEME_CORNER_RADIUS)

                    color_dither: 1.0

                    color: (THEME_COLOR_OUTSET)
                    color_hover: (THEME_COLOR_OUTSET_HOVER)
                    color_down: (THEME_COLOR_OUTSET_DOWN)
                    color_focus: (THEME_COLOR_OUTSET_FOCUS)
                    color_disabled: (THEME_COLOR_OUTSET_DISABLED)

                    border_color: (THEME_COLOR_BEVEL_OUTSET_1)
                    border_color_hover: (THEME_COLOR_BEVEL_OUTSET_1_HOVER)
                    border_color_focus: (THEME_COLOR_BEVEL_OUTSET_1_FOCUS)
                    border_color_down: (THEME_COLOR_BEVEL_OUTSET_1_DOWN)
                    border_color_disabled: (THEME_COLOR_BEVEL_OUTSET_1_DISABLED)

                    border_color_2: (THEME_COLOR_BEVEL_OUTSET_2)
                    border_color_2_hover: (THEME_COLOR_BEVEL_OUTSET_2_HOVER)
                    border_color_2_focus: (THEME_COLOR_BEVEL_OUTSET_2_FOCUS)
                    border_color_2_down: (THEME_COLOR_BEVEL_OUTSET_2_DOWN)
                    border_color_2_disabled: (THEME_COLOR_BEVEL_OUTSET_2_DISABLED)

                    arrow_color: (THEME_COLOR_LABEL_INNER)
                    arrow_color_hover: (THEME_COLOR_LABEL_INNER_HOVER)
                    arrow_color_focus: (THEME_COLOR_LABEL_INNER_FOCUS)
                    arrow_color_down: (THEME_COLOR_LABEL_INNER_DOWN)
                    arrow_color_disabled: (THEME_COLOR_LABEL_INNER_DISABLED)
                }
            }
        } // -- demos
    }
}

#[derive(Live, LiveHook, LiveRead, PartialEq, Debug)]
pub enum DropDownEnum {
    #[pick]
    ValueOne, ValueTwo,
    Third, FourthValue,
    OptionE, Hexagons
}

#[derive(Live, LiveHook, LiveRead, LiveRegister)]
pub struct DataBindingsForApp {
    #[live] fnumber: f32,
    #[live] inumber: i32,
    #[live] dropdown: DropDownEnum,
    #[live] dropdown_below: DropDownEnum,
    #[live] dropdown_disabled: DropDownEnum,
    #[live] dropdown_customized: DropDownEnum,
    #[live] dropdown_flat: DropDownEnum,
    #[live] dropdown_flat_below: DropDownEnum,
    #[live] dropdown_gradient_x: DropDownEnum,
    #[live] dropdown_gradient_x_below: DropDownEnum,
    #[live] dropdown_gradient_y: DropDownEnum,
    #[live] dropdown_gradient_y_below: DropDownEnum,
}

pub fn data_bind(mut db: DataBindingMap) {
    db.bind(ids!(dropdown), ids_array!(dropdown));
    db.bind(ids!(dropdown_below), ids_array!(dropdown_below));
    db.bind(ids!(dropdown_disabled), ids_array!(dropdown_disabled));
    db.bind(ids!(dropdown_customized), ids_array!(dropdown_customized));
    db.bind(ids!(dropdown_flat), ids_array!(dropdown_flat));
    db.bind(ids!(dropdown_flat_below), ids_array!(dropdown_flat_below));
    db.bind(ids!(dropdown_gradient_x), ids_array!(dropdown_gradient_x));
    db.bind(ids!(dropdown_gradient_x_below), ids_array!(dropdown_gradient_x_below));
    db.bind(ids!(dropdown_gradient_y), ids_array!(dropdown_gradient_y));
    db.bind(ids!(dropdown_gradient_y_below), ids_array!(dropdown_gradient_y_below));
}

pub fn handle_actions(app: &mut App, cx: &mut Cx, actions:&Actions) {
    if app.ui.button(ids!(my_ui_btn)).clicked(&actions) {
        log!("MyUI Button Clicked {}", app.counter);
        app.counter += 1;
        app.ui.button(ids!(my_ui_btn))
            .set_text(cx, &format!("Clicky clicky! {}", app.counter));
    }

    let mut db = DataBindingStore::new();
    db.data_bind(cx, actions, &app.ui, data_bind);
    app.bindings.apply_over(cx, &db.nodes);
}

//pub fn handle_startup(app: &mut App, cx: &mut Cx) {
    ////let ui = app.ui.clone();
    //let db = DataBindingStore::from_nodes(app.bindings.live_read());
    //data_bind(db.data_to_widgets(cx, &app.ui));
//}

