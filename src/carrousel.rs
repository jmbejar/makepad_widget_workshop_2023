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
        // flow: Down,
        flow: Overlay,

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

        page_animation_offset: 400.0

        animator: {
            page = {
                default: show,
                restart = {
                    from: {all: Snap}
                    apply: {page_animation_offset: 400.0}
                }
                show = {
                    redraw: true,
                    from: {all: Forward {duration: 0.5}}
                    apply: {page_animation_offset: 0.0}
                }
                cancel = {
                    from: {all: Snap}
                    apply: {page_animation_offset: 0.0}
                }
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

    #[live]
    page_animation_offset: f64,

    #[animator]
    animator: Animator,

    #[rust]
    next_frame: NextFrame,

    #[rust(None)]
    waiting_since: Option<f64>,
}

impl LiveHook for Carrousel {
    fn before_live_design(cx: &mut Cx) {
        register_widget!(cx, Carrousel);
    }

    fn after_apply(&mut self, cx: &mut Cx, from: ApplyFrom, _index: usize, _nodes: &[LiveNode]) {
        if from.is_from_doc()  {
            self.pages = vec![
                self.view.view(id!(page1)),
                self.view.view(id!(page2)),
                self.view.view(id!(page3))
            ];

            self.pages[self.current_page as usize].set_visible(true);

            self.next_frame = cx.new_next_frame();
        }
    }
}

#[derive(Clone, WidgetAction)]
pub enum CarrouselAction {
    None,
    PageChanged(u8),
}

impl Widget for Carrousel {
    fn handle_widget_event_with(
        &mut self,
        cx: &mut Cx,
        event: &Event,
        dispatch_action: &mut dyn FnMut(&mut Cx, WidgetActionItem),
    ) {
        let uid = self.widget_uid();
        self.handle_event_with(cx, event, &mut |cx, action| {
            dispatch_action(cx, WidgetActionItem::new(action.into(), uid));
        });
    }

    fn redraw(&mut self, cx: &mut Cx) {
        self.view.redraw(cx);
    }

    fn draw_walk_widget(&mut self, cx: &mut Cx2d, walk: Walk) -> WidgetDraw {
        self.view.draw_walk_widget(cx, walk)
    }
}

impl Carrousel {
    fn handle_event_with(
        &mut self,
        cx: &mut Cx,
        event: &Event,
        dispatch_action: &mut dyn FnMut(&mut Cx, CarrouselAction),
    ) {
        // Make sure we redraw when the animation is happening
        if self.animator_handle_event(cx, event).must_redraw() {
            self.update_image_positions(cx);
            self.redraw(cx);
        }

        self.orchestrate_animations(cx, event, dispatch_action);

        match event.hits(cx, self.view.area()) {
            Hit::FingerUp(fe) => if fe.is_over {
                // Do not fire a new animation if the carrousel is already animating
                if self.can_animate(cx) {
                    self.waiting_since = None;
                    self.play_animation(cx);

                    dispatch_action(cx, CarrouselAction::PageChanged(self.current_page))
                }
            }
            _ => ()
        };
    }

    fn reset_frames_visibility(&mut self) {
        for page in &mut self.pages {
            page.set_visible(false);
        }
    }

    fn update_current_page(&mut self) {
        self.current_page = (self.current_page + 1) % self.pages.len() as u8;
    }

    fn can_animate(&self, cx: &mut Cx) -> bool {
        !self.animator.is_track_animating(cx, id!(page))
    }

    fn play_animation(&mut self, cx: &mut Cx) {
        self.update_current_page();
        self.reset_frames_visibility();

        self.animator_play(cx, id!(page.restart));

        let next_page = &self.pages[self.current_page as usize];
        next_page.apply_over(cx, live!{ image = { margin: {left: (-400.0)} } });
        next_page.set_visible(true);

        let prev_page = (self.current_page + self.pages.len() as u8 - 1) % self.pages.len() as u8;
        self.pages[prev_page as usize].set_visible(true);
    }

    fn set_current_page(&mut self, cx: &mut Cx, page: u8) {
        self.current_page = page;
        self.reset_frames_visibility();

        self.animator_play(cx, id!(page.cancel));

        let next_page = &self.pages[self.current_page as usize];
        next_page.apply_over(cx, live!{ image = { margin: {left: (0.0)} } });
        next_page.set_visible(true);

        self.waiting_since = None;
    }

    fn update_image_positions(&mut self, cx: &mut Cx) {
        self.pages[self.current_page as usize]
            .apply_over(cx, live!{ image = { margin: {left: (self.page_animation_offset)} } });

        let prev_page = (self.current_page + self.pages.len() as u8 - 1) % self.pages.len() as u8;
        self.pages[prev_page as usize]
            .apply_over(cx, live!{ image = { margin: {left: (self.page_animation_offset - 400.0)} } });
    }

    fn orchestrate_animations(
        &mut self,
        cx: &mut Cx,
        event: &Event,
        dispatch_action: &mut dyn FnMut(&mut Cx, CarrouselAction)
    ){
        // Decide what to do when no animation is happening
        if self.can_animate(cx) {

            // Fire the "show" animation when the "restart" animation is done
            if self.animator.animator_in_state(cx, id!(page.restart)) {
                self.animator_play(cx, id!(page.show));
            }

            // Fire the "restart" animation automatically after some seconds idle
            if let Some(ne) = self.next_frame.is_event(event) {
                match self.waiting_since {
                    None => {
                        self.waiting_since = Some(ne.time);
                    }
                    Some(time) => {
                        if ne.time - time > 3.0 {
                            self.waiting_since = None;
                            self.play_animation(cx);

                            dispatch_action(cx, CarrouselAction::PageChanged(self.current_page))
                        }
                    }
                }
            }
        }

        self.next_frame = cx.new_next_frame();
    }
}

#[derive(Clone, WidgetRef)]
pub struct CarrouselRef(WidgetRef);

impl CarrouselRef {
    pub fn page_changed(&self, actions: &Vec<WidgetActionItem>) -> Option<u8> {
        if let Some(item) = actions.find_single_action(self.widget_uid()) {
            if let CarrouselAction::PageChanged(id) = item.action() {
                return Some(id);
            }
        }

        None
    }

    pub fn set_current_page(&self, cx: &mut Cx, page: u8) {
        if let Some(inner) = self.0.borrow_mut::<Carrousel>().as_mut() {
            inner.set_current_page(cx, page);
            inner.redraw(cx);
        }
    }
}
