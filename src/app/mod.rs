pub mod app_state;
pub mod config;

use sdl2::event::Event;
use sdl2::hint;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::EventPump;
use sdl2::{Sdl, TimerSubsystem};

use std::thread::sleep;
use std::time::Duration;

pub type WindowCanvas = Canvas<Window>;

use crate::app::app_state::AppState;
use crate::app::config::Config;
use crate::renderer::Renderer;

#[derive(PartialEq, Clone, Debug)]
pub enum UpdateResult {
    NoOp,
    Stop,
    RefreshPositions,
}

pub enum Task {
    OpenFile { file_path: String },
}

pub struct Application {
    config: Config,
    sdl_context: Sdl,
    canvas: WindowCanvas,
    tasks: Vec<Task>,
}

impl Application {
    pub fn new() -> Self {
        let config = Config::new();
        let sdl_context = sdl2::init().unwrap();
        hint::set("SDL_GL_MULTISAMPLEBUFFERS", "1");
        hint::set("SDL_GL_MULTISAMPLESAMPLES", "8");
        hint::set("SDL_GL_ACCELERATED_VISUAL", "1");
        hint::set("SDL_HINT_RENDER_SCALE_QUALITY", "2");
        hint::set("SDL_HINT_VIDEO_ALLOW_SCREENSAVER", "1");
        let video_subsystem = sdl_context.video().unwrap();
        let window = video_subsystem
            .window("Editor", config.width, config.height)
            .position_centered()
            .opengl()
            .build()
            .unwrap();

        let canvas = window.into_canvas().accelerated().build().unwrap();

        Self {
            config,
            sdl_context,
            canvas,
            tasks: vec![],
        }
    }

    pub fn init(&mut self) {
        self.clear();
    }

    pub fn run(&mut self) {
        let mut timer: TimerSubsystem = self.sdl_context.timer().unwrap();
        let mut event_pump = self.sdl_context.event_pump().unwrap();
        let font_context = sdl2::ttf::init().unwrap();
        let sleep_time = Duration::new(0, 1_000_000_000u32 / 60);
        let mut app_state = AppState::new();
        let mut renderer = Renderer::new(
            self.config.clone(),
            &font_context,
            self.canvas.texture_creator()
        );

        'running: loop {
            match self.handle_events(&mut event_pump) {
                UpdateResult::Stop => break 'running,
                UpdateResult::RefreshPositions => (),
                UpdateResult::NoOp => (),
            }
            for task in self.tasks.iter() {
                match task {
                    Task::OpenFile { file_path } => {
                        use crate::file::editor_file::*;
                        app_state.open_file(file_path.clone(), &mut renderer);
//                        use std::fs::read_to_string;
//                        if let Ok(buffer) = read_to_string(&file_path) {
//                            println!("read: {}\n{}", file_path, buffer);
//                            let file = EditorFile::new(file_path.clone(), buffer, &mut renderer);
//                            app_state.current_file = app_state.files.len() as i16;
//                            app_state.files.push(file);
//                        }
                    },
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
        self.canvas.set_draw_color(Color::RGB(255, 255, 255));
        self.canvas.clear();
    }

    fn handle_events(&mut self, event_pump: &mut EventPump) -> UpdateResult {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => return UpdateResult::Stop,
                _ => (),
            }
        }
        UpdateResult::NoOp
    }
}
