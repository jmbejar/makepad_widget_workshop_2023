use makepad_widgets::*;

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;

    ImageContainer = <View> {
        width: Fit,
        height: Fit,
        visible: false,
        image = <Image> {
            width: 400
            height: 266
        }
    }

    Carrousel = {{Carrousel}} {
        flow: Down,
        page1 = <ImageContainer> {
            image = {
                source: dep("crate://self/resources/image1.png")
            }
        }
        page2 = <ImageContainer> {
            image = {
                source: dep("crate://self/resources/image2.png")
            }
        }
        page3 = <ImageContainer> {
            image = {
                source: dep("crate://self/resources/image3.png")
            }
        }
    }
}

#[derive(Live)]
pub struct Carrousel {
    #[deref]
    view: View,

    #[rust(0)]
    current_page: u8,

    #[rust]
    pages: Vec<ViewRef>,
}

impl LiveHook for Carrousel {
    fn before_live_design(cx: &mut Cx) {
        register_widget!(cx, Carrousel);
    }

    fn after_new_from_doc(&mut self, _cx: &mut Cx) {
        self.pages = vec![
            self.view.view(id!(page1)),
            self.view.view(id!(page2)),
            self.view.view(id!(page3))
        ];

        self.pages[self.current_page as usize].set_visible(true);
    }
}

impl Widget for Carrousel {
    fn redraw(&mut self, cx: &mut Cx) {
        self.view.redraw(cx);
    }

    fn draw_walk_widget(&mut self, cx: &mut Cx2d, walk: Walk) -> WidgetDraw {
        self.view.draw_walk_widget(cx, walk)
    }
}
