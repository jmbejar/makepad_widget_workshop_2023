use makepad_widgets::*;

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;

    App = {{App}} {
        ui: <Window> {
            window: {position: vec2(0, 0), inner_size: vec2(400, 800)},

            body = {
                align: {x: 0.5, y: 0.5},

                <RoundedView> {
                    width: 200,
                    height: 100,
                    draw_bg: {color: #550}

                    align: {x: 0.5, y: 0.5},

                    <Label> {
                        text: "Hello World"
                    }
                }
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
    }
}

impl AppMain for App {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        if let Event::Draw(event) = event {
            return self.ui.draw_widget_all(&mut Cx2d::new(cx, event));
        }
    }
}
