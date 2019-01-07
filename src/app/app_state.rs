use crate::app::caret_manager;
use crate::app::file_content_manager;
use crate::app::{UpdateResult, WindowCanvas as WC};
use crate::config::*;
use crate::renderer::Renderer;
use crate::ui::caret::*;
use crate::ui::file::editor_file::EditorFile;
use crate::ui::file::*;
use crate::ui::menu_bar::MenuBar;
use crate::ui::text_character::TextCharacter;
use crate::ui::*;
use sdl2::rect::{Point, Rect};
use sdl2::VideoSubsystem as VS;
use std::boxed::Box;
use std::rc::Rc;
use std::sync::*;

pub struct AppState {
    menu_bar: MenuBar,
    files: Vec<EditorFile>,
    config: Arc<RwLock<Config>>,
    file_editor: FileEditor,
}

impl AppState {
    pub fn new(config: Arc<RwLock<Config>>) -> Self {
        Self {
            menu_bar: MenuBar::new(Arc::clone(&config)),
            files: vec![],
            file_editor: FileEditor::new(Arc::clone(&config)),
            config,
        }
    }

    pub fn open_file(&mut self, file_path: String, renderer: &mut Renderer) {
        use std::fs::read_to_string;

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

    pub fn file_editor(&self) -> &FileEditor {
        &self.file_editor
    }

    pub fn file_editor_mut(&mut self) -> &mut FileEditor {
        &mut self.file_editor
    }
}

impl Render for AppState {
    fn render(&self, canvas: &mut WC, renderer: &mut Renderer, _parent: Parent) {
        self.file_editor.render(canvas, renderer, None);
        self.menu_bar.render(canvas, renderer, None);
    }

    fn prepare_ui(&mut self, renderer: &mut Renderer) {
        self.menu_bar.prepare_ui(renderer);
        self.file_editor.prepare_ui(renderer);
    }
}

impl Update for AppState {
    fn update(&mut self, ticks: i32, context: &UpdateContext) -> UpdateResult {
        self.menu_bar.update(ticks, context);
        self.file_editor.update(ticks, context);
        UpdateResult::NoOp
    }
}

impl AppState {
    pub fn on_left_click(&mut self, point: &Point, video_subsystem: &mut VS) -> UpdateResult {
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
