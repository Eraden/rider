pub use crate::app::app_state::AppState;
pub use crate::renderer::CanvasRenderer;
use crate::ui::caret::{CaretPosition, MoveDirection};
use crate::ui::*;
pub use rider_config::{Config, ConfigAccess, ConfigHolder};
use sdl2::event::*;
use sdl2::hint;
use sdl2::keyboard::Keycode;
use sdl2::keyboard::Scancode;
use sdl2::mouse::*;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::Canvas;
use sdl2::rwops::RWops;
use sdl2::surface::Surface;
use sdl2::video::Window;
use sdl2::EventPump;
use sdl2::{Sdl, TimerSubsystem, VideoSubsystem};
use std::env;
use std::process::Command;
use std::sync::{Arc, RwLock};
use std::thread::sleep;
use std::time::Duration;

pub type WindowCanvas = Canvas<Window>;

#[derive(PartialEq, Clone, Debug)]
pub enum UpdateResult {
    NoOp,
    Stop,
    RefreshPositions,
    MouseLeftClicked(Point),
    MouseDragStart(Point),
    MouseDragStop(Point),
    MoveCaret(Rect, CaretPosition),
    DeleteFront,
    DeleteBack,
    Input(String),
    InsertNewLine,
    MoveCaretLeft,
    MoveCaretRight,
    MoveCaretUp,
    MoveCaretDown,
    Scroll { x: i32, y: i32 },
    WindowResize { width: i32, height: i32 },
    RefreshFsTree,
    OpenFile(String),
    OpenDirectory(String),
    OpenFileModal,
    FileDropped(String),
    SaveCurrentFile,
}

#[cfg_attr(tarpaulin, skip)]
pub struct Application {
    config: Arc<RwLock<Config>>,
    clear_color: Color,
    sdl_context: Sdl,
    canvas: WindowCanvas,
    video_subsystem: VideoSubsystem,
    tasks: Vec<UpdateResult>,
}

#[cfg_attr(tarpaulin, skip)]
impl Application {
    pub fn new() -> Self {
        let generator_path = rider_config::directories::get_binary_path("rider-generator")
            .unwrap_or_else(|e| panic!(e));
        Command::new(generator_path).status().unwrap();

        let mut config = Config::new();
        config.set_theme(config.editor_config().current_theme().clone());
        let config = Arc::new(RwLock::new(config));
        let sdl_context = sdl2::init().unwrap();

        hint::set("SDL_GL_MULTISAMPLEBUFFERS", "1");
        hint::set("SDL_GL_MULTISAMPLESAMPLES", "8");
        hint::set("SDL_GL_ACCELERATED_VISUAL", "1");
        hint::set("SDL_HINT_RENDER_SCALE_QUALITY", "2");
        hint::set("SDL_HINT_VIDEO_ALLOW_SCREENSAVER", "1");

        let video_subsystem = sdl_context.video().unwrap();

        let mut window: Window = {
            let c = config.read().unwrap();
            video_subsystem
                .window("Rider", c.width(), c.height())
                .position_centered()
                .resizable()
                .opengl()
                .build()
                .unwrap()
        };
        let icon_bytes = include_bytes!("../../assets/images/gear-64x64.bmp").clone();
        let mut rw = RWops::from_bytes(&icon_bytes).unwrap();
        let mut icon = Surface::load_bmp_rw(&mut rw).unwrap();
        window.set_icon(&mut icon);

        let canvas = window.into_canvas().accelerated().build().unwrap();
        let clear_color: Color = { config.read().unwrap().theme().background().into() };

        Self {
            sdl_context,
            video_subsystem,
            canvas,
            tasks: vec![],
            clear_color,
            config,
        }
    }

    pub fn init(&mut self) {
        self.clear();
    }

    pub fn run(&mut self) {
        let mut timer: TimerSubsystem = self.sdl_context.timer().unwrap();
        let mut event_pump = self.sdl_context.event_pump().unwrap();
        let font_context = sdl2::ttf::init().unwrap();
        let texture_creator = self.canvas.texture_creator();
        let sleep_time = Duration::new(0, 1_000_000_000u32 / 60);
        let mut app_state = AppState::new(Arc::clone(&self.config));
        let mut renderer =
            CanvasRenderer::new(Arc::clone(&self.config), &font_context, &texture_creator);
        app_state.prepare_ui(&mut renderer);

        'running: loop {
            self.handle_events(&mut event_pump);
            let mut new_tasks: Vec<UpdateResult> = vec![];
            for task in self.tasks.iter() {
                match task {
                    UpdateResult::Stop => break 'running,
                    UpdateResult::RefreshPositions => (),
                    UpdateResult::NoOp => (),
                    UpdateResult::MoveCaret(_, _pos) => (),
                    UpdateResult::MouseLeftClicked(point) => {
                        let res = app_state.on_left_click(&point, &mut self.video_subsystem);
                        match res {
                            UpdateResult::OpenDirectory(_) => new_tasks.push(res),
                            UpdateResult::OpenFile(_) => {
                                new_tasks.push(res);
                                app_state.set_open_file_modal(None);
                            }
                            _ => {}
                        }
                    }
                    UpdateResult::DeleteFront => {
                        app_state.file_editor_mut().delete_front(&mut renderer);
                    }
                    UpdateResult::DeleteBack => {
                        app_state.file_editor_mut().delete_back(&mut renderer);
                    }
                    UpdateResult::Input(text) => {
                        app_state
                            .file_editor_mut()
                            .insert_text(text.clone(), &mut renderer);
                    }
                    UpdateResult::InsertNewLine => {
                        app_state.file_editor_mut().insert_new_line(&mut renderer);
                    }
                    UpdateResult::MoveCaretLeft => {
                        app_state.file_editor_mut().move_caret(MoveDirection::Left);
                    }
                    UpdateResult::MoveCaretRight => {
                        app_state.file_editor_mut().move_caret(MoveDirection::Right);
                    }
                    UpdateResult::MoveCaretUp => {
                        app_state.file_editor_mut().move_caret(MoveDirection::Up);
                    }
                    UpdateResult::MoveCaretDown => {
                        app_state.file_editor_mut().move_caret(MoveDirection::Down);
                    }
                    UpdateResult::Scroll { x, y } => {
                        app_state.scroll_by(-x.clone(), -y.clone());
                    }
                    UpdateResult::WindowResize { width, height } => {
                        let mut c = app_state.config().write().unwrap();
                        let w = width.clone();
                        let h = height.clone();
                        if w > 0 {
                            c.set_width(w as u32);
                        }
                        if h > 0 {
                            c.set_height(h as u32);
                        }
                    }
                    UpdateResult::RefreshFsTree => unimplemented!(),
                    UpdateResult::OpenFile(file_path) => {
                        app_state.open_file(file_path.clone(), &mut renderer);
                    }
                    UpdateResult::OpenDirectory(dir_path) => {
                        app_state.open_directory(dir_path.clone(), &mut renderer);
                    }
                    UpdateResult::OpenFileModal => {
                        let pwd = Self::current_working_directory();
                        let mut modal =
                            OpenFile::new(pwd.clone(), 400, 800, Arc::clone(&self.config));
                        modal.prepare_ui(&mut renderer);
                        modal.open_directory(pwd.clone(), &mut renderer);
                        app_state.set_open_file_modal(Some(modal));
                    }
                    UpdateResult::MouseDragStart(_point) => (),
                    UpdateResult::MouseDragStop(_point) => (),
                    UpdateResult::FileDropped(_path) => (),
                    UpdateResult::SaveCurrentFile => {
                        app_state
                            .save_file()
                            .unwrap_or_else(|e| eprintln!("Failed to save {:?}", e));
                    }
                }
            }
            self.tasks = new_tasks;

            self.clear();

            app_state.update(timer.ticks() as i32, &UpdateContext::Nothing);
            app_state.render(&mut self.canvas, &mut renderer, &RenderContext::Nothing);

            self.present();

            if !cfg!(test) {
                sleep(sleep_time);
            }
        }
    }

    pub fn open_file(&mut self, file_path: String) {
        self.tasks.push(UpdateResult::OpenFile(file_path));
    }

    fn present(&mut self) {
        self.canvas.present();
    }

    fn clear(&mut self) {
        self.canvas.set_draw_color(self.clear_color.clone());
        self.canvas.clear();
    }

    fn handle_events(&mut self, event_pump: &mut EventPump) {
        let left_control_pressed = event_pump
            .keyboard_state()
            .is_scancode_pressed(Scancode::LCtrl);
        let shift_pressed = event_pump
            .keyboard_state()
            .is_scancode_pressed(Scancode::LShift)
            || event_pump
                .keyboard_state()
                .is_scancode_pressed(Scancode::RShift);

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => self.tasks.push(UpdateResult::Stop),
                Event::MouseButtonUp {
                    mouse_btn, x, y, ..
                } if mouse_btn == MouseButton::Left => {
                    self.tasks
                        .push(UpdateResult::MouseDragStart(Point::new(x, y)));
                    self.tasks
                        .push(UpdateResult::MouseLeftClicked(Point::new(x, y)));
                }
                Event::DropFile { filename, .. } => {
                    self.tasks.push(UpdateResult::FileDropped(filename))
                }
                Event::MouseButtonDown {
                    x, y, mouse_btn, ..
                } if mouse_btn == MouseButton::Left => self
                    .tasks
                    .push(UpdateResult::MouseDragStart(Point::new(x, y))),
                Event::KeyDown { keycode, .. } if keycode.is_some() => match keycode.unwrap() {
                    Keycode::Backspace => {
                        self.tasks.push(UpdateResult::DeleteFront);
                    }
                    Keycode::Delete => {
                        self.tasks.push(UpdateResult::DeleteBack);
                    }
                    Keycode::KpEnter | Keycode::Return => {
                        self.tasks.push(UpdateResult::InsertNewLine);
                    }
                    Keycode::Left => {
                        self.tasks.push(UpdateResult::MoveCaretLeft);
                    }
                    Keycode::Right => {
                        self.tasks.push(UpdateResult::MoveCaretRight);
                    }
                    Keycode::Up => {
                        self.tasks.push(UpdateResult::MoveCaretUp);
                    }
                    Keycode::Down => {
                        self.tasks.push(UpdateResult::MoveCaretDown);
                    }
                    Keycode::O if left_control_pressed && !shift_pressed => {
                        self.tasks.push(UpdateResult::OpenFileModal)
                    }
                    Keycode::S if left_control_pressed => {
                        self.tasks.push(UpdateResult::SaveCurrentFile)
                    }
                    _ => {}
                },
                Event::TextInput { text, .. } => {
                    self.tasks.push(UpdateResult::Input(text));
                }
                Event::MouseWheel {
                    direction, x, y, ..
                } => match direction {
                    MouseWheelDirection::Normal => {
                        self.tasks.push(UpdateResult::Scroll { x, y });
                    }
                    MouseWheelDirection::Flipped => {
                        self.tasks.push(UpdateResult::Scroll { x, y: -y });
                    }
                    _ => {
                        // ignore
                    }
                },
                Event::Window {
                    win_event: WindowEvent::Resized(w, h),
                    ..
                } => self.tasks.push(UpdateResult::WindowResize {
                    width: w,
                    height: h,
                }),
                _ => {}
            }
        }
    }

    pub fn current_working_directory() -> String {
        env::current_dir().unwrap().to_str().unwrap().to_string()
    }
}

#[cfg_attr(tarpaulin, skip)]
impl ConfigHolder for Application {
    fn config(&self) -> &ConfigAccess {
        &self.config
    }
}
