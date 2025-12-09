use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    use crate::components::layout_templates::*;

    IMG_A = dep("crate://self/resources/ducky.png")
    IMG_PROFILE_A = (IMG_A)
    COLOR_TEXT = #x8
    COLOR_TEXT_LIGHT = #0xCCC
    COLOR_USER = #x444

    Post = <View> {
        width: Fill, height: Fit
        padding: { top: 10., bottom: 10. }
        body = <RoundedView> {
            width: Fill, height: Fit
            spacing: 10.
            <Image> {
                width: 20, height: 20
                source: (IMG_PROFILE_A)
            }
            content = <View> {
                width: Fill, height: Fit
                text = <P> { text: "en?" }
            }
        }
    }

    NewsFeed = {{NewsFeed}} {
        list = <PortalList> {
            scroll_bar: <ScrollBar> {}
            TopSpace = <View> { height: 0. }
            BottomSpace = <View> { height: 10. }
            Post = <CachedView> {
                flow: Down
                <Post> {}
                <View> {
                    width: Fill, height: Fit
                    align: { x: 1. }
                    spacing: 6
                    likes = <Button> {}
                    comments = <Button> {}
                }
                <Hr> {}
            }
        }
    }

    pub DemoPortalList = <UIZooTabLayout_B> {
        desc = {
            <Markdown> { body: dep("crate://self/resources/portallist.md") }
        } // -- desc
        demos = {
            news_feed = <NewsFeed> {}
        } // -- demos
    }
}

#[derive(Live, LiveHook, Widget)]
struct NewsFeed {
    #[deref] view: View
}

impl Widget for NewsFeed {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        while let Some(item) = self.view.draw_walk(cx, scope, walk).step() {
            if let Some(mut list) = item.as_portal_list().borrow_mut() {
                list.set_item_range(cx, 0, 200);
                while let Some(item_id) = list.next_visible_item(cx) {
                    let template = match item_id {
                        0 => live_id!(TopSpace),
                        _ => live_id!(Post)
                    };
                    let item = list.item(cx, item_id, template);
                    let text = match item_id % 4 {
                        1 => format!("At vero eos et accusam et justo duo dolores et ea rebum. Stet clita kasd gubergren, no sea takimata sanctus est Lorem ipsum dolor sit amet. Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua. At vero eos et accusam et justo duo dolores et ea rebum."),
                        2 => format!("How are you? 你还好么?"),
                        3 => format!("Stet clita kasd gubergren, no sea takimata sanctus est Lorem ipsum dolor sit amet."),
                        _ => format!("Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua.")
                    };
                    item.label(ids!(content.text)).set_text(cx, &text);
                    item.button(ids!(likes)).set_text(cx, &format!("likes: {}",item_id % 23));
                    item.button(ids!(comments)).set_text(cx, &format!("comments: {}",item_id % 6));
                    item.draw_all(cx, &mut Scope::empty());
                }
            }
        }
        DrawStep::done()
    }

    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
    }
}

