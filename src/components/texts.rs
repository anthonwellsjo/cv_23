use dioxus::prelude::*;


pub fn Title (cx: Scope) -> Element {
    cx.render(rsx!(
        
        h1 {
        style: "font-family: Lato; font-weight: thin; font-size: 4vw; position: absolute; top:0; left: 5vw;",
        "Anthon Wellsjö"
    }
))
}
