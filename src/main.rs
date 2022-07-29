pub mod components;
pub mod app;


fn main() {
    dioxus::web::launch(app::app)
}
