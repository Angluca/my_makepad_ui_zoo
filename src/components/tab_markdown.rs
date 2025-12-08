use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    use crate::components::layout_templates::*;

    pub DemoMarkdown = <UIZooTabLayout_B> {
        desc = {
            <Markdown> { body: dep("crate://self/resources/markdown.md") }
        } // -- desc
        demos = {
            <Markdown> {
                width: Fill, height: Fit
                body:"
                # Headline 1
                ## Headline 2
                ### Headline 3
                #### Headline 4
                This is standard text with a \n
                line break a short ~~strike through~~ demo.\n
                *Italic text* \n\n **Bold text** \n
                - Bullet\n - Another bullet
                - Third bullet
                1. Numbered list Bullet
                2. Another list entry
                3. Third list entry \n
                `Monospaced text`
                > This is a quote.\nThis is `inline code`. \n
                ```code block```
                "
            }
        } // -- demos
    }
}

