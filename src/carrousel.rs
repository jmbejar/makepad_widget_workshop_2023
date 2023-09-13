use makepad_widgets::*;

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;

    Carrousel = {{Carrousel}} {
        flow: Down,

        image1 = <Image> {
            width: 400
            height: 266
            source: dep("crate://self/resources/image1.png")
        }
        image2 = <Image> {
            width: 400
            height: 266
            source: dep("crate://self/resources/image2.png")
        }
        image3 = <Image> {
            width: 400
            height: 266
            source: dep("crate://self/resources/image3.png")
        }
    }
}

#[derive(Live)]
pub struct Carrousel {
    #[deref]
    view: View,
}

impl LiveHook for Carrousel {
    fn before_live_design(cx: &mut Cx) {
        register_widget!(cx, Carrousel);
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
