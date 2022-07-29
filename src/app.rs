use crate::components::{
    layout::{Inner_frame, Outer_frame},
    texts::Title,
};
use dioxus::prelude::*;

pub fn app(cx: Scope) -> Element {
    cx.render(rsx!(Outer_frame {
        body: cx.render(rsx!(
            Title(),
            Inner_frame {
                body: cx.render(rsx!(

                    h1 {"heheheheheh"}


                ))
            }
        ))
    }))
}
