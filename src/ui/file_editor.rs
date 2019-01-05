use sdl2::rect::*;

use crate::app::*;
use crate::ui::*;

pub struct FileEditor {
    dest: Rect,
}

impl FileEditor {
    pub fn new(dest: Rect) -> Self {
        Self { dest }
    }
}

impl Render for FileEditor {
    fn render(
        &self,
        _canvas: &mut WindowCanvas,
        _renderer: &mut Renderer,
        _parent: Option<&RenderBox>,
    ) -> UpdateResult {
        unimplemented!()
    }

    fn prepare_ui(&mut self, _renderer: &mut Renderer) {
        unimplemented!()
    }
}

impl Update for FileEditor {
    fn update(&mut self, _ticks: i32) -> UpdateResult {
        unimplemented!()
    }
}

impl RenderBox for FileEditor {
    fn render_start_point(&self) -> Point {
        unimplemented!()
    }
}
