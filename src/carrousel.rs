use makepad_widgets::*;

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;

    Carrousel = <View> {
        width: 200,
        height: 100,
        draw_bg: {color: #550},
        show_bg: true,

        align: {x: 0.5, y: 0.5},

        <Label> {
            text: "Hello World"
        }
    }
}