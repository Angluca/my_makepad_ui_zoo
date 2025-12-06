use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    pub OverView = <View> {
        spacing: (THEME_SPACE_2),
        padding: <THEME_MSPACE_2> {},
        flow: Down,
        align: {x: 0.5, y: 0.5}
        width: Fill, height: Fill,

        <ScrollYView> {
            flow: Down,
            width: 560., height: Fill,
            align: {x: 0, y: 0.4}
            spacing: (THEME_SPACE_2),

            <Image> {
                margin: {bottom: 10.},
                width: 250., height: 36.5,
                source: dep("crate://self/resources/logo_makepad.png"),
                fit: Biggest,
            }

            <H4> { text: "Makepad is an open-source, cross-platform UI framework written in and for Rust. It runs natively and on the web, supporting all major platforms: Windows, Linux, macOS, iOS, and Android." }
            <P> {
                text: "Built on a shader-based architecture, Makepad delivers high performance, making it suitable for complex applications like Photoshop or even 3D/VR/AR experiences."
            }
            <P> {
                text: "One of Makepad’s standout features is live styling — a powerful system that reflects UI code changes instantly without recompilation or restarts. This tight feedback loop bridges the gap between developers and designers, streamlining collaboration and maximizing productivity."
            }
            <P> {
                text: "This example application provides an overview of the currently supported widgets."
            }

            <TextBox> { height: Fit, text: "UI Zoo hosts a high number of widgets and variants, resulting in loading times not representative of typical Makepad applications." }

        }
    }
}
