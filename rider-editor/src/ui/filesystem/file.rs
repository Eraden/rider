use crate::app::*;
use crate::renderer::*;
use crate::ui::*;
use rider_config::directories;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use std::collections::HashMap;
use std::path;

pub struct FileEntry {
    name_width: u32,
    icon_width: u32,
    height: u32,
    name: String,
    path: String,
    dest: Rect,
    source: Rect,
    config: ConfigAccess,
    char_sizes: HashMap<char, Rect>,
}

impl FileEntry {
    pub fn new(name: String, path: String, config: ConfigAccess) -> Self {
        Self {
            name,
            path,
            name_width: 0,
            icon_width: 0,
            height: 0,
            dest: Rect::new(0, 0, 16, 16),
            source: Rect::new(0, 0, 64, 64),
            config,
            char_sizes: HashMap::new(),
        }
    }

    pub fn name_width(&self) -> u32 {
        self.name_width
    }

    pub fn icon_width(&self) -> u32 {
        self.icon_width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn path(&self) -> String {
        self.path.clone()
    }

    pub fn dest(&self) -> &Rect {
        &self.dest
    }

    pub fn source(&self) -> &Rect {
        &self.source
    }

    pub fn full_dest(&self) -> Rect {
        Rect::new(
            self.dest.x(),
            self.dest.y(),
            self.icon_width + NAME_MARGIN as u32 + self.name_width,
            self.height,
        )
    }

    fn render_icon<T>(&self, canvas: &mut T, renderer: &mut Renderer, dest: &mut Rect)
    where
        T: RenderImage,
    {
        let dir_texture_path = {
            let c = self.config.read().unwrap();
            let mut themes_dir = directories::themes_dir();
            let path = c.theme().images().file_icon();
            themes_dir.push(path);
            themes_dir.to_str().unwrap().to_owned()
        };
        let texture = renderer
            .texture_manager()
            .load(dir_texture_path.as_str())
            .unwrap_or_else(|_| panic!("Failed to load directory entry texture"));
        dest.set_width(16);
        dest.set_height(16);
        canvas
            .render_image(texture, self.source.clone(), dest.clone())
            .unwrap_or_else(|_| panic!("Failed to draw directory entry texture"));
    }

    fn render_name<T>(&self, canvas: &mut T, renderer: &mut Renderer, dest: &mut Rect)
    where
        T: RenderImage,
    {
        let mut d = dest.clone();
        d.set_x(dest.x() + NAME_MARGIN);

        let font_details = build_font_details(self);
        let font = renderer.font_manager().load(&font_details).unwrap();
        let texture_manager = renderer.texture_manager();
        let name = self.name();

        for c in name.chars() {
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
            let text_texture = texture_manager.load_text(&mut text_details, &font).unwrap();
            d.set_width(size.width());
            d.set_height(size.height());

            canvas
                .render_image(text_texture, self.source.clone(), d.clone())
                .unwrap_or_else(|_| panic!("Failed to draw directory entry texture"));
            d.set_x(d.x() + size.width() as i32)
        }
    }
}

impl ConfigHolder for FileEntry {
    fn config(&self) -> &ConfigAccess {
        &self.config
    }
}

#[cfg_attr(tarpaulin, skip)]
impl FileEntry {
    pub fn render<T>(&self, canvas: &mut T, renderer: &mut Renderer, context: &RenderContext)
    where
        T: RenderImage,
    {
        let mut dest = match context {
            &RenderContext::RelativePosition(p) => move_render_point(p.clone(), &self.dest),
            _ => self.dest.clone(),
        };
        self.render_icon(canvas, renderer, &mut dest);
        self.render_name(canvas, renderer, &mut dest.clone());
    }

    pub fn prepare_ui(&mut self, renderer: &mut Renderer) {
        let w_rect = get_text_character_rect('W', renderer).unwrap();
        self.char_sizes.insert('W', w_rect.clone());
        self.height = w_rect.height();
        self.icon_width = w_rect.height();
        self.name_width = 0;

        for c in self.name().chars() {
            let size = { get_text_character_rect(c.clone(), renderer).unwrap() };
            self.char_sizes.insert(c, size);
            self.name_width += size.width();
        }
        self.dest.set_width(w_rect.height());
        self.dest.set_height(w_rect.height());
    }
}

impl Update for FileEntry {
    fn update(&mut self, _ticks: i32, _context: &UpdateContext) -> UpdateResult {
        if !path::Path::new(&self.path).exists() {
            return UpdateResult::RefreshFsTree;
        }
        UpdateResult::NoOp
    }
}

impl RenderBox for FileEntry {
    fn render_start_point(&self) -> Point {
        self.dest.top_left()
    }

    fn dest(&self) -> Rect {
        self.dest.clone()
    }
}

impl ClickHandler for FileEntry {
    fn on_left_click(&mut self, _point: &Point, _context: &UpdateContext) -> UpdateResult {
        UpdateResult::OpenFile(self.path.clone())
    }

    fn is_left_click_target(&self, point: &Point, context: &UpdateContext) -> bool {
        let dest = Rect::new(
            self.dest.x(),
            self.dest.y(),
            self.icon_width + self.name_width + NAME_MARGIN as u32,
            self.dest.height(),
        );
        let rect = match context {
            UpdateContext::ParentPosition(p) => move_render_point(p.clone(), &dest),
            _ => dest,
        };
        rect.contains_point(point.clone())
    }
}
