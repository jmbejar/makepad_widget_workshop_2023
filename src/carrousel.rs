use makepad_widgets::*;

const CARROUSEL_MAX_OFFSET: f64 = 400.0;

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

        carrousel_page_offset: 0.0

        animator: {
            carrousel = {
                default: restart,
                restart = {
                    from: {all: Snap}
                    apply: {carrousel_page_offset: 400.0}
                }
                show = {
                    redraw: true,
                    from: {all: Forward {duration: 0.5}}
                    apply: {carrousel_page_offset: 0.0}
                }
            }
        }
    }
}

#[derive(Live)]
pub struct Carrousel {
    #[rust] area: Area,

    #[live]
    images: Vec<LiveDependency>,

    #[live]
    page_template: Option<LivePtr>,

    #[rust(0)]
    current_page: u8,

    #[rust]
    pages: ComponentMap<LiveId,WidgetRef>,

    #[live]
    carrousel_page_offset: f64,

    #[animator]
    animator: Animator,
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
        // Make sure we redraw when the animation is happening
        if self.animator_handle_event(cx, event).must_redraw() {
            self.redraw(cx);
        }

        // Fire the "show" animation when the "restart" animation is done
        if self.animator.animator_in_state(cx, id!(carrousel.restart)) {
            self.animator_play(cx, id!(carrousel.show));
        }

        match event.hits(cx, self.area) {
            Hit::FingerUp(fe) => if fe.is_over {
                // Do not fire a new animation if the carrousel is already animating
                if !self.animator.is_track_animating(cx, id!(carrousel)) {
                    self.update_current_page();
                    self.animator_play(cx, id!(carrousel.restart));
                    //self.redraw(cx);
                }
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

        self.draw_page_with_offset(cx, self.current_page, self.carrousel_page_offset, walk);

        let prev_page_idx = (self.current_page + self.images.len() as u8 - 1) % self.images.len() as u8;
        self.draw_page_with_offset(cx, prev_page_idx, self.carrousel_page_offset - CARROUSEL_MAX_OFFSET, walk);

        cx.end_turtle_with_area(&mut self.area);
    }

    fn draw_page_with_offset(&mut self, cx: &mut Cx2d, index: u8, offset: f64, walk: Walk) {
        let widget_id = LiveId::from_str(&format!("page{}", index));
        let page = self.pages.get_or_insert(cx, widget_id, |cx| {
            WidgetRef::new_from_ptr(cx, self.page_template)
        });

        let _ = page.draw_walk_widget(
            cx,
            walk.with_margin_left(offset)
        );
    }

    fn update_current_page(&mut self) {
        self.current_page = (self.current_page + 1) % self.images.len() as u8;
    }
}
