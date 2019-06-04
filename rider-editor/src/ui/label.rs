use crate::app::*;
use crate::renderer::*;
use crate::ui::*;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use std::collections::HashMap;

const DEST_WIDTH: u32 = 16;
const DEST_HEIGHT: u32 = 16;
const SRC_WIDTH: u32 = 64;
const SRC_HEIGHT: u32 = 64;

pub struct Label {
    name_width: u32,
    height: u32,
    name: String,
    source: Rect,
    dest: Rect,
    char_sizes: HashMap<char, Rect>,
    config: ConfigAccess,
}

impl Label {
    pub fn new(name: String, config: ConfigAccess) -> Self {
        Self {
            name,
            name_width: 0,
            height: 0,
            dest: Rect::new(0, 0, DEST_WIDTH, DEST_HEIGHT),
            source: Rect::new(0, 0, SRC_WIDTH, SRC_HEIGHT),
            config,
            char_sizes: HashMap::new(),
        }
    }

    pub fn name_width(&self) -> u32 {
        self.name_width
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn render<C, R>(&self, canvas: &mut C, renderer: &mut R, context: &RenderContext)
    where
        C: CanvasAccess,
        R: Renderer,
    {
        let dest = match context {
            &RenderContext::ParentPosition(p) => move_render_point(p.clone(), &self.dest),
            _ => self.dest.clone(),
        };
        let mut d = dest.clone();
        d.set_x(dest.x() + NAME_MARGIN);

        let font_details = build_font_details(self);
        for c in self.name.chars() {
            let size = self
                .char_sizes
                .get(&c)
                .unwrap_or(&Rect::new(0, 0, 0, 0))
                .clone();
            let mut text_details = TextDetails {
                color: Color::RGBA(255, 255, 255, 0),
                text: c.to_string(),
                font: font_details.clone(),
            };
            let maybe_texture = renderer.load_text_tex(&mut text_details, font_details.clone());

            if let Ok(texture) = maybe_texture {
                d.set_width(size.width());
                d.set_height(size.height());

                canvas
                    .render_image(texture, self.source.clone(), d.clone())
                    .unwrap_or_else(|_| panic!("Failed to draw directory entry texture"));
                d.set_x(d.x() + size.width() as i32);
            }
        }
    }

    pub fn prepare_ui<R>(&mut self, renderer: &mut R)
    where
        R: Renderer + CharacterSizeManager,
    {
        let w_rect = renderer.load_character_size('W');
        self.char_sizes.insert('W', w_rect.clone());
        self.height = w_rect.height();
        self.name_width = 0;

        for c in self.name().chars() {
            let size = { renderer.load_character_size(c.clone()) };
            self.char_sizes.insert(c, size);
            self.name_width += size.width();
        }
        self.dest.set_width(w_rect.height());
        self.dest.set_height(w_rect.height());
    }
}

impl RenderBox for Label {
    fn render_start_point(&self) -> Point {
        self.dest.top_left()
    }

    fn dest(&self) -> Rect {
        self.dest.clone()
    }
}

impl Update for Label {
    fn update(&mut self, _ticks: i32, _context: &UpdateContext) -> UpdateResult {
        UpdateResult::NoOp
    }
}

impl ConfigHolder for Label {
    fn config(&self) -> &ConfigAccess {
        &self.config
    }
}
