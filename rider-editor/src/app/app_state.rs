use crate::app::application::Application;
use crate::app::UpdateResult;
use crate::renderer::renderer::Renderer;
use crate::ui::*;
use rider_config::*;
use sdl2::rect::Point;
use sdl2::VideoSubsystem as VS;
use std::fs::{read_to_string, File};
use std::io::Write;
use std::sync::*;

pub struct AppState {
    menu_bar: MenuBar,
    project_tree: ProjectTreeSidebar,
    files: Vec<EditorFile>,
    config: Arc<RwLock<Config>>,
    file_editor: FileEditor,
    modal: Option<ModalType>,
}

impl AppState {
    pub fn new(config: Arc<RwLock<Config>>) -> Self {
        Self {
            menu_bar: MenuBar::new(Arc::clone(&config)),
            project_tree: ProjectTreeSidebar::new(
                Application::current_working_directory(),
                config.clone(),
            ),
            files: vec![],
            file_editor: FileEditor::new(Arc::clone(&config)),
            modal: None,
            config,
        }
    }

    #[cfg_attr(tarpaulin, skip)]
    pub fn open_file<R>(&mut self, file_path: String, renderer: &mut R) -> Result<(), String>
    where
        R: Renderer + CharacterSizeManager + ConfigHolder,
    {
        let buffer = read_to_string(&file_path)
            .map_err(|file_path| format!("Failed to open file: {}", file_path))?;
        let mut file = EditorFile::new(file_path.clone(), buffer, self.config.clone());
        file.prepare_ui(renderer);
        match self.file_editor.open_file(file) {
            Some(old) => self.files.push(old),
            _ => (),
        }
        Ok(())
    }

    pub fn save_file(&self) -> Result<(), String> {
        let editor_file = match self.file_editor.file() {
            Some(f) => f,
            _ => Err("No buffer found".to_string())?,
        };
        let mut f = File::create(editor_file.path())
            .or_else(|_| Err("File can't be opened".to_string()))?;

        f.write_all(editor_file.buffer().as_bytes())
            .or_else(|_| Err("Failed to write to file".to_string()))?;

        f.flush()
            .or_else(|_| Err("Failed to write to file".to_string()))?;
        Ok(())
    }

    pub fn open_settings<R>(&mut self, renderer: &mut R) -> Result<(), String>
    where
        R: Renderer + CharacterSizeManager,
    {
        match self.modal {
            None => {
                let mut settings = Settings::new(self.config.clone());
                settings.prepare_ui(renderer);
                self.modal = Some(ModalType::Settings(settings));
            }
            _ => return Ok(()),
        }
        Ok(())
    }

    pub fn close_modal(&mut self) -> Result<(), String> {
        self.modal = None;
        Ok(())
    }

    pub fn open_directory<R>(&mut self, dir_path: String, renderer: &mut R)
    where
        R: Renderer + CharacterSizeManager,
    {
        match self.modal.as_mut() {
            Some(ModalType::OpenFile(modal)) => modal.open_directory(dir_path, renderer),
            None => self.project_tree.open_directory(dir_path, renderer),
            _ => (),
        };
    }

    #[cfg_attr(tarpaulin, skip)]
    pub fn file_editor(&self) -> &FileEditor {
        &self.file_editor
    }

    #[cfg_attr(tarpaulin, skip)]
    pub fn file_editor_mut(&mut self) -> &mut FileEditor {
        &mut self.file_editor
    }

    pub fn set_open_file_modal(&mut self, modal: Option<OpenFile>) {
        self.modal = if let Some(modal) = modal {
            Some(ModalType::OpenFile(modal))
        } else {
            None
        };
    }

    pub fn scroll_by(&mut self, x: i32, y: i32) {
        match self.modal.as_mut() {
            Some(ModalType::OpenFile(modal)) => modal.scroll_by(x, y),
            Some(ModalType::Settings(modal)) => modal.scroll_by(x, y),
            _ => self.file_editor_mut().scroll_by(x, y),
        };
    }

    pub fn open_file_modal(&self) -> Option<&OpenFile> {
        match self.modal {
            Some(ModalType::OpenFile(ref m)) => Some(m),
            _ => None,
        }
    }

    pub fn settings_modal(&self) -> Option<&Settings> {
        match self.modal {
            Some(ModalType::Settings(ref m)) => Some(m),
            _ => None,
        }
    }
}

#[cfg_attr(tarpaulin, skip)]
impl AppState {
    pub fn render<C, R>(&self, canvas: &mut C, renderer: &mut R, _context: &RenderContext)
    where
        C: CanvasAccess,
        R: Renderer + ConfigHolder + CharacterSizeManager,
    {
        // file editor
        self.file_editor
            .render(canvas, renderer, &RenderContext::Nothing);

        // menu bar
        self.menu_bar
            .render(canvas, renderer, &RenderContext::Nothing);

        // project tree
        self.project_tree
            .render(canvas, renderer, &RenderContext::Nothing);

        // settings modal
        match self.modal.as_ref() {
            Some(ModalType::OpenFile(modal)) => {
                return modal.render(canvas, renderer, &RenderContext::Nothing)
            }
            Some(ModalType::Settings(modal)) => {
                return modal.render(canvas, renderer, &RenderContext::Nothing)
            }
            _ => (),
        };
    }

    pub fn prepare_ui<R>(&mut self, renderer: &mut R)
    where
        R: Renderer + CharacterSizeManager,
    {
        self.menu_bar.prepare_ui();
        self.project_tree.prepare_ui(renderer);
        self.file_editor.prepare_ui(renderer);
    }

    pub fn update(&mut self, ticks: i32, context: &UpdateContext) -> UpdateResult {
        let res = match self.modal.as_mut() {
            Some(ModalType::OpenFile(modal)) => modal.update(ticks, context.clone()),
            Some(ModalType::Settings(modal)) => modal.update(ticks, context.clone()),
            None => UpdateResult::NoOp,
        };
        if res != UpdateResult::NoOp {
            return res;
        }

        // menu bar
        self.menu_bar.update(ticks, context);

        // sidebar
        self.project_tree.update(ticks, context);

        // file editor
        let context = UpdateContext::ParentPosition(
            self.project_tree.full_rect().top_right() + Point::new(10, 0),
        );
        self.file_editor.update(ticks, &context);
        UpdateResult::NoOp
    }
}

impl AppState {
    #[cfg_attr(tarpaulin, skip)]
    pub fn on_left_click(&mut self, point: &Point, video_subsystem: &mut VS) -> UpdateResult {
        if self
            .project_tree
            .is_left_click_target(point, &UpdateContext::Nothing)
        {
            return self
                .project_tree
                .on_left_click(point, &UpdateContext::Nothing);
        }
        match self.modal.as_mut() {
            Some(ModalType::OpenFile(modal)) => {
                return modal.on_left_click(point, &UpdateContext::Nothing)
            }
            Some(ModalType::Settings(modal)) => {
                return modal.on_left_click(point, &UpdateContext::Nothing)
            }
            _ => (),
        };
        if self
            .menu_bar
            .is_left_click_target(point, &UpdateContext::Nothing)
        {
            video_subsystem.text_input().stop();
            return self.menu_bar.on_left_click(point, &UpdateContext::Nothing);
        } else if !self
            .file_editor
            .is_left_click_target(point, &UpdateContext::Nothing)
        {
            return UpdateResult::NoOp;
        } else {
            video_subsystem.text_input().start();
            return self
                .file_editor
                .on_left_click(point, &UpdateContext::Nothing);
        }
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
        assert_ne!(
            state
                .open_file_modal()
                .unwrap_or_else(|| panic!("Failed to open file modal"))
                .scroll(),
            old_scroll
        );
    }

    #[test]
    fn must_fail_save_file_when_none_is_open() {
        let config = support::build_config();
        let state = AppState::new(Arc::clone(&config));
        let result = state.save_file();
        assert_eq!(result, Err(format!("No buffer found")));
    }

    #[test]
    fn must_succeed_save_file_when_file_is_open() {
        assert_eq!(std::fs::create_dir_all("/tmp").is_ok(), true);
        assert_eq!(
            std::fs::write(
                "/tmp/must_succeed_save_file_when_file_is_open.md",
                "Foo bar"
            )
            .is_ok(),
            true
        );

        let config = support::build_config();
        let mut renderer = support::SimpleRendererMock::new(config.clone());
        let mut state = AppState::new(Arc::clone(&config));
        let result = state.open_file(
            format!("/tmp/must_succeed_save_file_when_file_is_open.md"),
            &mut renderer,
        );
        assert_eq!(result, Ok(()));
        let result = state.save_file();
        assert_eq!(result, Ok(()));
    }

    #[test]
    fn must_succeed_save_file_when_file_does_not_exists() {
        assert_eq!(std::fs::create_dir_all("/tmp").is_ok(), true);
        assert_eq!(
            std::fs::write(
                "/tmp/must_succeed_save_file_when_file_does_not_exists.md",
                "Foo bar"
            )
            .is_ok(),
            true
        );

        let config = support::build_config();
        let mut renderer = support::SimpleRendererMock::new(config.clone());
        let mut state = AppState::new(Arc::clone(&config));
        let result = state.open_file(
            format!("/tmp/must_succeed_save_file_when_file_does_not_exists.md"),
            &mut renderer,
        );
        assert_eq!(result, Ok(()));
        let result = state.save_file();
        assert_eq!(result, Ok(()));
    }

    #[test]
    fn must_close_modal_when_no_modal_is_open() {
        let config = support::build_config();
        let mut state = AppState::new(Arc::clone(&config));
        assert_eq!(state.close_modal(), Ok(()));
    }

    #[test]
    fn must_close_modal_when_some_modal_is_open() {
        let config = support::build_config();
        let mut state = AppState::new(Arc::clone(&config));
        let modal = OpenFile::new("/".to_owned(), 100, 100, Arc::clone(&config));
        state.set_open_file_modal(Some(modal));
        assert_eq!(state.close_modal(), Ok(()));
    }

    #[test]
    fn open_settings_when_there_is_no_other_modal() {
        let config = support::build_config();
        let mut renderer = support::SimpleRendererMock::new(config.clone());
        let mut state = AppState::new(Arc::clone(&config));
        assert_eq!(state.open_settings(&mut renderer), Ok(()));
    }

    #[test]
    fn open_settings_when_other_modal_is_open() {
        let config = support::build_config();
        let mut renderer = support::SimpleRendererMock::new(config.clone());
        let mut state = AppState::new(Arc::clone(&config));
        let modal = OpenFile::new("/".to_owned(), 100, 100, Arc::clone(&config));
        state.set_open_file_modal(Some(modal));
        assert_eq!(state.open_settings(&mut renderer), Ok(()));
    }

    #[test]
    fn must_open_directory() {
        assert_eq!(
            std::fs::create_dir_all("/tmp/must_open_directory").is_ok(),
            true
        );

        let config = support::build_config();
        let mut renderer = support::SimpleRendererMock::new(config.clone());
        let mut state = AppState::new(Arc::clone(&config));
        state.open_directory("/must_open_directory".to_owned(), &mut renderer);
    }
}
