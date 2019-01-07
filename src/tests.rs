#[cfg(test)]
pub mod support {
    use crate::config::*;
    use crate::renderer::*;
    use sdl2::render::{Canvas, WindowCanvas};
    use sdl2::*;
    use sdl2::{Sdl, TimerSubsystem, VideoSubsystem};
    use std::borrow::*;
    use std::sync::*;

    pub fn build_config() -> Arc<RwLock<Config>> {
        Arc::new(RwLock::new(Config::new()))
    }

    pub fn build_canvas() -> WindowCanvas {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem
            .window("Test", 1, 1)
            .borderless()
            .opengl()
            .build()
            .unwrap();

        window.into_canvas().accelerated().build().unwrap()
    }
}
