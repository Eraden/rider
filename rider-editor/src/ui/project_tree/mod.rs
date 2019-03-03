use crate::renderer::renderer::Renderer;
use crate::ui::filesystem::directory::DirectoryView;
use crate::ui::text_character::CharacterSizeManager;
use crate::ui::CanvasAccess;
use crate::ui::RenderContext;
use rider_config::config::Config;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use std::sync::Arc;
use std::sync::RwLock;

pub struct ProjectTreeSidebar {
    dest: Rect,
    config: Arc<RwLock<Config>>,
    root: String,
    border_color: Color,
    background_color: Color,
    dir_view: DirectoryView,
}

impl ProjectTreeSidebar {
    pub fn new(root: String, config: Arc<RwLock<Config>>) -> Self {
        let (background_color, border_color, h): (Color, Color, u32) = {
            let c = config.read().unwrap();
            (
                c.theme().background().into(),
                c.theme().border_color().into(),
                c.height(),
            )
        };

        Self {
            dest: Rect::new(0, 0, 100, h),
            dir_view: DirectoryView::new(root.clone(), config.clone()),
            config,
            root,
            background_color,
            border_color,
        }
    }

    pub fn update(&mut self, _ticks: i32) {
        let config = self.config.read().unwrap();
        let height = config.height();
        //        let left_margin = config.editor_left_margin();
        let top_margin = config.menu_height() as i32;
        //        self.dest.set_x(left_margin);
        self.dest.set_y(top_margin);
        self.dest.set_height(height - top_margin as u32);
    }

    pub fn prepare_ui<R>(&mut self, renderer: &mut R)
    where
        R: Renderer + CharacterSizeManager,
    {
        let config = self.config.read().unwrap();
        let height = config.height();
        let left_margin = config.editor_left_margin();
        let top_margin = config.menu_height() as i32;
        self.dest.set_x(left_margin);
        self.dest.set_y(top_margin);
        self.dest.set_height(height);
        self.dir_view.prepare_ui(renderer);
    }

    pub fn render<C, R>(&self, canvas: &mut C, renderer: &mut R)
    where
        R: Renderer + CharacterSizeManager,
        C: CanvasAccess,
    {
        canvas.set_clipping(self.dest.clone());
        canvas
            .render_rect(self.dest.clone(), self.background_color.clone())
            .unwrap();
        canvas
            .render_border(self.dest.clone(), self.border_color.clone())
            .unwrap();

        // dir view
        let context = RenderContext::RelativePosition(self.dest.top_left());
        self.dir_view.render(canvas, renderer, &context);
    }

    pub fn full_rect(&self) -> Rect {
        self.dest.clone()
    }

    pub fn root(&self) -> String {
        self.root.clone()
    }
}

#[cfg(test)]
mod tests {
    /*let pwd = env::current_dir().unwrap().to_str().unwrap().to_string();*/
}
