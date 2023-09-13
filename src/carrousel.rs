use makepad_widgets::*;

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;

    ImageContainer = <View> {
        width: Fit,
        height: Fit,
        image = <Image> {
            width: 400
            height: 266
        }
    }

    Carrousel = {{Carrousel}} {
        images: [
            dep("crate://self/resources/image1.png"),
            dep("crate://self/resources/image2.png"),
            dep("crate://self/resources/image3.png")
        ]

        page_template: <ImageContainer> {}
    }
}

#[derive(Live, LiveHook)]
#[live_ignore]
pub enum CarrouselPageOrder {
    #[pick] Normal,
    Reverse
}

#[derive(Live)]
pub struct Carrousel {
    #[rust] area: Area,

    #[live]
    page_order: CarrouselPageOrder,

    #[live]
    images: Vec<LiveDependency>,

    #[live]
    page_template: Option<LivePtr>,

    #[rust(0)]
    current_page: u8,

    #[rust]
    pages: ComponentMap<LiveId,WidgetRef>
}

impl LiveHook for Carrousel {
    fn before_live_design(cx: &mut Cx) {
        register_widget!(cx, Carrousel);
    }

    fn after_apply(&mut self, cx: &mut Cx, from: ApplyFrom, _index: usize, _nodes: &[LiveNode]) {
        if from.is_from_doc()  {
            for (idx, image_dep) in self.images.iter().enumerate() {
                let widget_id = LiveId::from_str(&format!("page{}", idx));
                let page = self.pages.get_or_insert(cx, widget_id, |cx| {
                    WidgetRef::new_from_ptr(cx, self.page_template)
                });

                page
                    .image(id!(image))
                    .load_image_dep_by_path(cx, &image_dep.as_str());
            }
        }
    }
}

impl Widget for Carrousel {
    fn handle_widget_event_with(
        &mut self,
        cx: &mut Cx,
        event: &Event,
        _dispatch_action: &mut dyn FnMut(&mut Cx, WidgetActionItem),
    ) {
        match event.hits(cx, self.area) {
            Hit::FingerUp(fe) => if fe.is_over {
                self.update_current_page();
                self.redraw(cx);
            }
            _ => ()
        };
    }

    fn redraw(&mut self, cx: &mut Cx) {
        self.area.redraw(cx);
    }

    fn draw_walk_widget(&mut self, cx: &mut Cx2d, walk: Walk) -> WidgetDraw {
        self.draw_walk(cx, walk);
        WidgetDraw::done()
    }
}

impl Carrousel {
    fn draw_walk(&mut self, cx: &mut Cx2d, walk: Walk) {
        cx.begin_turtle(walk, Layout::default());

        let widget_id = LiveId::from_str(&format!("page{}", self.current_page));
        let page = self.pages.get_or_insert(cx, widget_id, |cx| {
            WidgetRef::new_from_ptr(cx, self.page_template)
        });

        let _ = page.draw_walk_widget(cx, walk);
        cx.end_turtle_with_area(&mut self.area);
    }

    fn update_current_page(&mut self) {
        match self.page_order {
            CarrouselPageOrder::Normal => {
                self.current_page = (self.current_page + 1) % self.images.len() as u8;
            }
            CarrouselPageOrder::Reverse => {
                self.current_page = (self.current_page + self.images.len() as u8 - 1) % self.images.len() as u8;
            }
        }
    }
}
