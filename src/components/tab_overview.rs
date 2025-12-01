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
            width: 560, height: Fill,
            align: {x: 0, y: 0.4}
            spacing: (THEME_SPACE_2),

            <Image> {
                margin: {bottom: 10},
                width: 250, height: 36.5,
                source: dep("crate://self/resources/logo_makepad.png"),
                fit: Biggest,
            }

            <H4> { text: "Makepad 是一个开源的、跨平台的 UI 框架，由 Rust 编写并为 Rust 服务。它可以在本地和 Web 上运行，支持所有主流平台：Windows、Linux、macOS、iOS 和 Android" }
            <P> {
                text: "基于着色器架构构建的 Makepad 具有高性能，非常适合构建复杂应用，例如 Photoshop，甚至 3D/VR/AR 体验"
            }
            <P> {
                text: "Makepad 的一大亮点是实时样式系统——无需重新编译或重启，就能立即反映 UI 代码的更改。这种紧密的反馈循环消除了开发者与设计师之间的隔阂，大幅提升协作效率和生产力"
            }
            <P> {
                text: "此示例应用程序展示了当前已支持的组件概览"
            }

            <TextBox> { height: Fit, text: "UI Zoo 包含大量组件和变体，因此其加载时间并不能代表典型的 Makepad 应用程序" }

        }
    }
}
