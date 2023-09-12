use makepad_widgets::*;

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;

    Carrousel = <View> {
        flow: Down
        <Image> {
            width: 400
            height: 266
            source: dep("crate://self/resources/image1.png")
        }
        <Image> {
            width: 400
            height: 266
            source: dep("crate://self/resources/image2.png")
        }
        <Image> {
            width: 400
            height: 266
            source: dep("crate://self/resources/image3.png")
        }
    }
}