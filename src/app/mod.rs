use crate::app::app_state::AppState;
use crate::config::Config;
use crate::renderer::Renderer;
use crate::themes::*;
use crate::ui::*;
use crate::ui::caret::{CaretPosition,MoveDirection};

use std::rc::Rc;
use std::thread::sleep;
use std::time::Duration;

use sdl2::event::Event;
use sdl2::hint;
use sdl2::keyboard::{Keycode, Mod};
use sdl2::mouse::MouseButton;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::Canvas;
use sdl2::ttf::Sdl2TtfContext;
use sdl2::video::Window;
use sdl2::EventPump;
use sdl2::{Sdl, TimerSubsystem, VideoSubsystem};

pub mod app_state;

pub type WindowCanvas = Canvas<Window>;

#[derive(PartialEq, Clone, Debug)]
pub enum UpdateResult {
    NoOp,
    Stop,
    RefreshPositions,
    MouseLeftClicked(Point),
    MoveCaret(Rect, CaretPosition),
    DeleteFront,
    DeleteBack,
    Input(String),
    InsertNewLine,
    MoveCaretLeft,
    MoveCaretRight,
    MoveCaretUp,
    MoveCaretDown,
}

pub enum Task {
    OpenFile { file_path: String },
}

pub struct Application {
    config: Rc<Config>,
    clear_color: Color,
    sdl_context: Sdl,
    canvas: WindowCanvas,
    video_subsystem: VideoSubsystem,
    tasks: Vec<Task>,
}

impl Application {
    pub fn new() -> Self {
        let config = Rc::new(Config::new());
        let sdl_context = sdl2::init().unwrap();
        hint::set("SDL_GL_MULTISAMPLEBUFFERS", "1");
        hint::set("SDL_GL_MULTISAMPLESAMPLES", "8");
        hint::set("SDL_GL_ACCELERATED_VISUAL", "1");
        hint::set("SDL_HINT_RENDER_SCALE_QUALITY", "2");
        hint::set("SDL_HINT_VIDEO_ALLOW_SCREENSAVER", "1");

        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem
            .window("Rider", config.width(), config.height())
            .position_centered()
            .opengl()
            .build()
            .unwrap();

        let canvas = window.into_canvas().accelerated().build().unwrap();

        Self {
            sdl_context,
            video_subsystem,
            canvas,
            tasks: vec![],
            clear_color: config.theme().background().into(),
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
        let mut app_state = AppState::new(self.config.clone());
        let mut renderer = Renderer::new(self.config.clone(), &font_context, &texture_creator);

        'running: loop {
            match self.handle_events(&mut event_pump) {
                UpdateResult::Stop => break 'running,
                UpdateResult::RefreshPositions => (),
                UpdateResult::NoOp => (),
                UpdateResult::MoveCaret(_, _pos) => (),
                UpdateResult::MouseLeftClicked(point) => {
                    app_state.on_left_click(&point, &mut self.video_subsystem);
                }
                UpdateResult::DeleteFront => {
                    app_state.delete_front();
                }
                UpdateResult::DeleteBack => {
                    app_state.delete_back();
                }
                UpdateResult::Input(text) => {
                    app_state.insert_text(text, &mut renderer);
                }
                UpdateResult::InsertNewLine => {
                    app_state.insert_new_line(&mut renderer);
                }
                UpdateResult::MoveCaretLeft => {
                    app_state.move_caret(MoveDirection::Left);
                }
                UpdateResult::MoveCaretRight => {
                    app_state.move_caret(MoveDirection::Right);
                }
                UpdateResult::MoveCaretUp => {
                    app_state.move_caret(MoveDirection::Up);
                }
                UpdateResult::MoveCaretDown => {
                    app_state.move_caret(MoveDirection::Down);
                }
            }
            for task in self.tasks.iter() {
                match task {
                    Task::OpenFile { file_path } => {
                        use crate::ui::file::editor_file::*;
                        app_state.open_file(file_path.clone());
                    }
                }
            }
            self.tasks.clear();

            self.clear();

            app_state.update(timer.ticks() as i32);
            app_state.render(&mut self.canvas, &mut renderer);

            self.present();
            sleep(sleep_time);
        }
    }

    pub fn open_file(&mut self, file_path: String) {
        self.tasks.push(Task::OpenFile { file_path });
    }

    fn present(&mut self) {
        self.canvas.present();
    }

    fn clear(&mut self) {
        self.canvas.set_draw_color(self.clear_color.clone());
        self.canvas.clear();
    }

    fn handle_events(&mut self, event_pump: &mut EventPump) -> UpdateResult {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => return UpdateResult::Stop,
                Event::MouseButtonUp {
                    mouse_btn, x, y, ..
                } => match mouse_btn {
                    MouseButton::Left => return UpdateResult::MouseLeftClicked(Point::new(x, y)),
                    _ => (),
                },
                Event::KeyDown { keycode, .. } => {
                    let keycode = if keycode.is_some() {
                        keycode.unwrap()
                    } else {
                        return UpdateResult::NoOp;
                    };
                    match keycode {
                        Keycode::Backspace => return UpdateResult::DeleteFront,
                        Keycode::Delete => return UpdateResult::DeleteBack,
                        Keycode::KpEnter | Keycode::Return => return UpdateResult::InsertNewLine,
                        Keycode::Left => return UpdateResult::MoveCaretLeft,
                        Keycode::Right => return UpdateResult::MoveCaretRight,
                        Keycode::Up => return UpdateResult::MoveCaretUp,
                        Keycode::Down => return UpdateResult::MoveCaretDown,
                        _ => UpdateResult::NoOp,
                    };
                }
                Event::TextInput { text, .. } => {
                    println!("text input: {}", text);
                    return UpdateResult::Input(text);
                }
                _ => (),
            }
        }
        UpdateResult::NoOp
    }
}
