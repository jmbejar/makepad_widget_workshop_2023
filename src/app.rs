use makepad_widgets::*;
use crate::carrousel::CarrouselAction;

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;

    import crate::carrousel::Carrousel;

    App = {{App}} {
        ui: <Window> {
            window: {position: vec2(0, 0), inner_size: vec2(400, 800)},

            body = {
                flow: Down
                spacing: 10
                padding: {top: 100}

                page_label = <Label> {text: "Hello World"}
                <Carrousel> {}
            }
        }
    }
}

app_main!(App);

#[derive(Live)]
pub struct App {
    #[live]
    ui: WidgetRef,
}

impl LiveHook for App {
    fn before_live_design(cx: &mut Cx) {
        makepad_widgets::live_design(cx);
        crate::carrousel::live_design(cx);
    }
}

impl AppMain for App {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        if let Event::Draw(event) = event {
            return self.ui.draw_widget_all(&mut Cx2d::new(cx, event));
        }

        let actions = self.ui.handle_widget_event(cx, event);

        for action in actions {
            match action.action() {
                CarrouselAction::PageChanged(id) => {
                    println!("Page changed to {}", id);
                    self.ui.label(id!(page_label)).set_text_and_redraw(cx, &format!("Page {}", id));
                },
                _ => (),
            }
        }
    }
}
