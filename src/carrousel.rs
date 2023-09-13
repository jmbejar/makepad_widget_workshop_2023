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

        animator: {
            page1 = {
                default: show,
                restart = {
                    from: {all: Snap}
                    apply: {page1 = { image = { margin: {left: 400.0}}}}
                }
                show = {
                    redraw: true,
                    from: {all: Forward {duration: 0.5}}
                    apply: {page1 = { image = { margin: {left: 0.0}}}}
                }
                // Added a new animation to hide the page
                hide = {
                    redraw: true,
                    from: {all: Forward {duration: 0.5}}
                    apply: {page1 = { image = { margin: {left: -400.0}}}}
                }
            }

            page2 = {
                default: hide,
                restart = {
                    from: {all: Snap}
                    apply: {page2 = { image = { margin: {left: 400.0}}}}
                }
                show = {
                    redraw: true,
                    from: {all: Forward {duration: 0.5}}
                    apply: {page2 = { image = { margin: {left: 0.0}}}}
                }
                hide = {
                    redraw: true,
                    from: {all: Forward {duration: 0.5}}
                    apply: {page2 = { image = { margin: {left: -400.0}}}}
                }
            }

            page3 = {
                default: hide,
                restart = {
                    from: {all: Snap}
                    apply: {page3 = { image = { margin: {left: 400.0}}}}
                }
                show = {
                    redraw: true,
                    from: {all: Forward {duration: 0.5}}
                    apply: {page3 = { image = { margin: {left: 0.0}}}}
                }
                hide = {
                    redraw: true,
                    from: {all: Forward {duration: 0.5}}
                    apply: {page3 = { image = { margin: {left: -400.0}}}}
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

    #[animator]
    animator: Animator,
}

impl LiveHook for Carrousel {
    fn before_live_design(cx: &mut Cx) {
        register_widget!(cx, Carrousel);
    }

    fn after_apply(&mut self, _cx: &mut Cx, from: ApplyFrom, _index: usize, _nodes: &[LiveNode]) {
        if from.is_from_doc()  {
            self.pages = vec![
                self.view.view(id!(page1)),
                self.view.view(id!(page2)),
                self.view.view(id!(page3))
            ];

            self.pages[self.current_page as usize].set_visible(true);
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
        if self.animator.animator_in_state(cx, id!(page1.restart)) {
            self.animator_play(cx, id!(page1.show));
        }
        if self.animator.animator_in_state(cx, id!(page2.restart)) {
            self.animator_play(cx, id!(page2.show));
        }
        if self.animator.animator_in_state(cx, id!(page3.restart)) {
            self.animator_play(cx, id!(page3.show));
        }

        match event.hits(cx, self.view.area()) {
            Hit::FingerUp(fe) => if fe.is_over {
                // Do not fire a new animation if the carrousel is already animating
                if self.can_animate(cx) {
                    self.play_animation(cx);
                }
            }
            _ => ()
        };
    }

    fn redraw(&mut self, cx: &mut Cx) {
        self.view.redraw(cx);
    }

    fn draw_walk_widget(&mut self, cx: &mut Cx2d, walk: Walk) -> WidgetDraw {
        self.view.draw_walk_widget(cx, walk)
    }
}

impl Carrousel {
    fn reset_frames_visibility(&mut self) {
        for page in &mut self.pages {
            page.set_visible(false);
        }
    }

    fn update_current_page(&mut self) {
        self.current_page = (self.current_page + 1) % self.pages.len() as u8;
    }

    fn can_animate(&self, cx: &mut Cx) -> bool {
        !self.animator.is_track_animating(cx, id!(page1)) &&
            !self.animator.is_track_animating(cx, id!(page2)) &&
            !self.animator.is_track_animating(cx, id!(page3))
    }

    fn play_animation(&mut self, cx: &mut Cx) {
        self.update_current_page();
        self.reset_frames_visibility();

        match self.current_page {
            0 => self.animator_play(cx, id!(page1.restart)),
            1 => self.animator_play(cx, id!(page2.restart)),
            2 => self.animator_play(cx, id!(page3.restart)),
            _ => ()
        }
        self.pages[self.current_page as usize].set_visible(true);

        let prev_page = (self.current_page + self.pages.len() as u8 - 1) % self.pages.len() as u8;
        match prev_page {
            0 => self.animator_play(cx, id!(page1.hide)),
            1 => self.animator_play(cx, id!(page2.hide)),
            2 => self.animator_play(cx, id!(page3.hide)),
            _ => ()
        }
        self.pages[prev_page as usize].set_visible(true);
    }
}
