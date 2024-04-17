use tokio;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

mod tutorial_surface;
mod tutorial_pipeline;
mod tutorial_buffers;
mod tutorial_textures;
mod tutorial_buffers_and_camera;

#[tokio::main]
async fn runner() {
    tutorial_buffers_and_camera::run().await;
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
fn main() {
    cfg_if::cfg_if! {
        if #[cfg(target_arch = "wasm32")] {
            std::panic::set_hook(Box::new(console_error_panic_hook::hook));
            console_log::init_with_level(log::Level::Warn).expect("Couldn't initialize logger");
        } else {
            env_logger::init();
        }
    }

    runner()
}
