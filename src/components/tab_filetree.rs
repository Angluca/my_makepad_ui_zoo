use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    use crate::components::layout_templates::*;
    use crate::components::demofiletree::*;

    pub DemoFT = <UIZooTabLayout_B> {
        desc = {
            <Markdown> { body: dep("crate://self/resources/filetree.md") }
        } // -- desc
        demos = {
            <DemoFileTree> { file_tree:{ width: Fill, height: Fill } }
        } // -- demos
    }
}

