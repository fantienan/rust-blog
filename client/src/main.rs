mod app;
mod components;
mod models;
mod fetch;

fn main() {
    yew::Renderer::<app::App>::new().render();
}
