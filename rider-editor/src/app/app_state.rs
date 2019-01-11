use crate::app::{UpdateResult, WindowCanvas as WC};
use crate::renderer::Renderer;
use crate::ui::*;
use rider_config::*;
use sdl2::rect::Point;
use sdl2::VideoSubsystem as VS;
use std::fs::read_to_string;
use std::sync::*;

pub struct AppState {
    menu_bar: MenuBar,
    files: Vec<EditorFile>,
    config: Arc<RwLock<Config>>,
    file_editor: FileEditor,
    open_file_modal: Option<OpenFile>,
}

impl AppState {
    pub fn new(config: Arc<RwLock<Config>>) -> Self {
        Self {
            menu_bar: MenuBar::new(Arc::clone(&config)),
            files: vec![],
            file_editor: FileEditor::new(Arc::clone(&config)),
            open_file_modal: None,
            config,
        }
    }

    #[cfg_attr(tarpaulin, skip)]
    pub fn open_file(&mut self, file_path: String, renderer: &mut Renderer) {
        if let Ok(buffer) = read_to_string(&file_path) {
            let mut file = EditorFile::new(file_path.clone(), buffer, self.config.clone());
            file.prepare_ui(renderer);
            match self.file_editor.open_file(file) {
                Some(old) => self.files.push(old),
                _ => (),
            }
        } else {
            eprintln!("Failed to open file: {}", file_path);
        };
    }

    #[cfg_attr(tarpaulin, skip)]
    pub fn open_directory(&mut self, dir_path: String, renderer: &mut Renderer) {
        match self.open_file_modal.as_mut() {
            Some(modal) => modal.open_directory(dir_path, renderer),
            _ => (),
        };
    }

    pub fn file_editor(&self) -> &FileEditor {
        &self.file_editor
    }

    pub fn file_editor_mut(&mut self) -> &mut FileEditor {
        &mut self.file_editor
    }

    pub fn set_open_file_modal(&mut self, modal: Option<OpenFile>) {
        self.open_file_modal = modal;
    }

    pub fn scroll_by(&mut self, x: i32, y: i32) {
        if let Some(modal) = self.open_file_modal.as_mut() {
            modal.scroll_by(x, y);
        } else {
            self.file_editor_mut().scroll_by(x, y);
        }
    }

    pub fn open_file_modal(&self) -> Option<&OpenFile> {
        self.open_file_modal.as_ref()
    }
}

#[cfg_attr(tarpaulin, skip)]
impl Render for AppState {
    fn render(&self, canvas: &mut WC, renderer: &mut Renderer, _context: &RenderContext) {
        self.file_editor
            .render(canvas, renderer, &RenderContext::Nothing);
        self.menu_bar
            .render(canvas, renderer, &RenderContext::Nothing);
        match self.open_file_modal.as_ref() {
            Some(modal) => modal.render(canvas, renderer, &RenderContext::Nothing),
            _ => (),
        };
    }

    fn prepare_ui(&mut self, renderer: &mut Renderer) {
        self.menu_bar.prepare_ui(renderer);
        self.file_editor.prepare_ui(renderer);
    }
}

impl Update for AppState {
    fn update(&mut self, ticks: i32, context: &UpdateContext) -> UpdateResult {
        let res = match self.open_file_modal.as_mut() {
            Some(modal) => modal.update(ticks, &UpdateContext::Nothing),
            _ => UpdateResult::NoOp,
        };
        if res != UpdateResult::NoOp {
            return res;
        }

        self.menu_bar.update(ticks, context);
        self.file_editor.update(ticks, context);
        UpdateResult::NoOp
    }
}

impl AppState {
    pub fn on_left_click(&mut self, point: &Point, video_subsystem: &mut VS) -> UpdateResult {
        match self.open_file_modal.as_mut() {
            Some(modal) => return modal.on_left_click(point, &UpdateContext::Nothing),
            _ => (),
        };
        if self
            .menu_bar
            .is_left_click_target(point, &UpdateContext::Nothing)
        {
            video_subsystem.text_input().stop();
            return self.menu_bar.on_left_click(point, &UpdateContext::Nothing);
        } else {
            if !self
                .file_editor
                .is_left_click_target(point, &UpdateContext::Nothing)
            {
                return UpdateResult::NoOp;
            } else {
                video_subsystem.text_input().start();
                self.file_editor
                    .on_left_click(point, &UpdateContext::Nothing);
            }
        }
        UpdateResult::NoOp
    }

    pub fn is_left_click_target(&self, _point: &Point) -> bool {
        true
    }
}

impl ConfigHolder for AppState {
    fn config(&self) -> &ConfigAccess {
        &self.config
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::support;
//    use crate::ui::modal::open_file;
    use std::sync::Arc;

    #[test]
    fn must_return_none_for_default_file() {
        let config = support::build_config();
        let state = AppState::new(Arc::clone(&config));
        let file = state.file_editor().file();
        assert_eq!(file.is_none(), true);
    }

    #[test]
    fn must_scroll_file_when_no_modal() {
        let config = support::build_config();
        let mut state = AppState::new(Arc::clone(&config));
        let old_scroll = state.file_editor().scroll();
        state.set_open_file_modal(None);
        state.scroll_by(10, 10);
        assert_ne!(state.file_editor().scroll(), old_scroll);
    }

    #[test]
    fn must_scroll_modal_when_modal_was_set() {
        let config = support::build_config();
        let mut state = AppState::new(Arc::clone(&config));
        let modal = OpenFile::new("/".to_owned(), 100, 100, Arc::clone(&config));
        let file_scroll = state.file_editor().scroll();
        let old_scroll = state.file_editor().scroll();
        state.set_open_file_modal(Some(modal));
        state.scroll_by(10, 10);
        assert_eq!(state.file_editor().scroll(), file_scroll);
        assert_ne!(state.open_file_modal().unwrap().scroll(), old_scroll);
    }
}
