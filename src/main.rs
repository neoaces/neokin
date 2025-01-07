use nannou::prelude::*;
pub mod links;
pub mod window;

fn main() {
    nannou::app(window::init)
        .update(window::update)
        .loop_mode(LoopMode::RefreshSync)
        .run();
}
