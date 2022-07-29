use std::fmt::Arguments;

use dioxus::prelude::*;

#[derive(Props)]
pub struct FrameProps<'a> {
    body: Element<'a>
}

pub fn Outer_frame<'a>(cx: Scope<'a, FrameProps<'a>>) -> Element {
    let out_fr_styles: Arguments = format_args!("width: 100vw; height: 100vh; display: flex; justify-content: center; align-items: center; flex-direction: column;");
    cx.render(rsx!(
        div {
            style: out_fr_styles,
            &cx.props.body
    }
))
}


pub fn Inner_frame<'a>(cx: Scope<'a, FrameProps<'a>>) -> Element {
    let in_fr_styles = format_args!("width: 35vw; height: 80vh; border-radius: 2px; box-shadow: 1px 1px 1000px rgb(235, 235, 224); ");

    cx.render(rsx!(
        div {
            style: in_fr_styles,
            &cx.props.body
    }
))

}




