use makepad_widgets::*;

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;

    Carrousel = {{Carrousel}} {
        image1: <Image> {
            width: 400
            height: 266
            source: dep("crate://self/resources/image1.png")
        }
        image2: <Image> {
            width: 400
            height: 266
            source: dep("crate://self/resources/image2.png")
        }
        image3: <Image> {
            width: 400
            height: 266
            source: dep("crate://self/resources/image3.png")
        }
    }
}

#[derive(Live)]
pub struct Carrousel {
    #[live]
    image1: Image,

    #[live]
    image2: Image,

    #[live]
    image3: Image,
}

impl LiveHook for Carrousel {
    fn before_live_design(cx: &mut Cx) {
        register_widget!(cx, Carrousel);
    }
}

impl Widget for Carrousel {
    fn redraw(&mut self, _cx: &mut Cx) {
    }

    fn draw_walk_widget(&mut self, cx: &mut Cx2d, walk: Walk) -> WidgetDraw {
        cx.begin_turtle(walk, Layout {flow: Flow::Down, ..Layout::default()});

        let walk1 = self.image1.walk(cx);
        self.image1.draw_walk_widget(cx, walk1) ?;

        let walk2 = self.image2.walk(cx);
        self.image2.draw_walk_widget(cx, walk2) ?;

        let walk3 = self.image3.walk(cx);
        self.image3.draw_walk_widget(cx, walk3) ?;

        cx.end_turtle();

        WidgetDraw::done()
    }
}
