use crate::app::*;
use crate::renderer::*;
use crate::ui::*;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use std::fs;
use std::path;
use std::sync::Arc;

const CHILD_MARGIN: i32 = 4;
const DEFAULT_ICON_SIZE: u32 = 16;

pub struct DirectoryView {
    opened: bool,
    expanded: bool,
    name_width: u32,
    icon_width: u32,
    icon_height: u32,
    height: u32,
    path: String,
    files: Vec<FileEntry>,
    directories: Vec<DirectoryView>,
    pos: Point,
    source: Rect,
    config: ConfigAccess,
}

impl DirectoryView {
    pub fn new(path: String, config: ConfigAccess) -> Self {
        Self {
            opened: false,
            expanded: false,
            name_width: 0,
            icon_width: DEFAULT_ICON_SIZE,
            icon_height: DEFAULT_ICON_SIZE,
            height: 0,
            path,
            files: vec![],
            directories: vec![],
            pos: Point::new(0, 0),
            source: Rect::new(0, 0, 64, 64),
            config,
        }
    }

    pub fn path(&self) -> String {
        self.path.clone()
    }

    pub fn dest(&self) -> Rect {
        match self.expanded {
            true => Rect::new(
                self.pos.x(),
                self.pos.y(),
                self.icon_width + self.name_width + NAME_MARGIN as u32,
                self.height,
            ),
            false => Rect::new(
                self.pos.x(),
                self.pos.y(),
                self.icon_width + self.name_width + NAME_MARGIN as u32,
                self.icon_height,
            ),
        }
    }

    pub fn source(&self) -> &Rect {
        &self.source
    }

    pub fn open_directory(&mut self, dir_path: String, renderer: &mut CanvasRenderer) -> bool {
        match dir_path {
            _ if dir_path == self.path => {
                if !self.opened {
                    self.opened = true;
                    self.expanded = true;
                    self.read_directory(renderer);
                } else {
                    self.expanded = !self.expanded;
                }
                self.calculate_size(renderer);
                true
            }
            _ if dir_path.contains((self.path.clone() + "/").as_str()) => {
                if !self.opened {
                    self.opened = true;
                    self.expanded = true;
                    self.read_directory(renderer);
                }
                for dir in self.directories.iter_mut() {
                    if dir.open_directory(dir_path.clone(), renderer) {
                        break;
                    }
                }
                self.calculate_size(renderer);
                true
            }
            _ => false,
        }
    }

    pub fn refresh(&mut self) {
        unimplemented!()
    }

    pub fn name(&self) -> String {
        path::Path::new(&self.path)
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .to_owned()
    }

    pub fn name_width(&self) -> u32 {
        self.name_width
    }

    pub fn icon_width(&self) -> u32 {
        self.icon_width
    }

    pub fn height(&self) -> u32 {
        match self.expanded {
            true => self.height,
            false => self.icon_height,
        }
    }

    fn read_directory(&mut self, renderer: &mut CanvasRenderer) {
        let entries: fs::ReadDir = match fs::read_dir(self.path.clone()) {
            Ok(d) => d,
            _ => return,
        };
        for e in entries {
            let entry = match e {
                Ok(entry) => entry,
                _ => continue,
            };
            let meta = match entry.metadata() {
                Ok(meta) => meta,
                _ => continue,
            };
            if meta.is_dir() {
                let path = match entry.path().to_str() {
                    Some(p) => p.to_string(),
                    _ => continue,
                };
                let mut directory_view = DirectoryView::new(path, Arc::clone(&self.config));
                directory_view.prepare_ui(renderer);
                self.directories.push(directory_view);
            } else if meta.is_file() {
                let file_name = match entry.file_name().to_str() {
                    Some(p) => p.to_string(),
                    _ => continue,
                };
                let path = match entry.path().to_str() {
                    Some(p) => p.to_string(),
                    _ => continue,
                };
                let mut file_entry = FileEntry::new(file_name, path, Arc::clone(&self.config));
                file_entry.prepare_ui(renderer);
                self.files.push(file_entry);
            }
        }
        self.files.sort_by(|a, b| a.name().cmp(&b.name()));
        self.directories.sort_by(|a, b| a.name().cmp(&b.name()));
    }

    fn render_icon<T>(&self, canvas: &mut T, renderer: &mut CanvasRenderer, dest: &mut Rect)
    where
        T: CanvasAccess,
    {
        let dir_texture_path = {
            let c = self.config.read().unwrap();
            let mut themes_dir = c.directories().themes_dir.clone();
            let path = c.theme().images().directory_icon();
            themes_dir.push(path);
            themes_dir.to_str().unwrap().to_owned()
        };
        let texture = renderer
            .texture_manager()
            .load(dir_texture_path.as_str())
            .unwrap_or_else(|_| panic!("Failed to load directory entry texture"));

        canvas
            .render_image(
                texture,
                self.source.clone(),
                Rect::new(dest.x(), dest.y(), self.icon_width, self.icon_height),
            )
            .unwrap_or_else(|_| panic!("Failed to draw directory entry texture"));
    }

    fn render_name<T>(&self, canvas: &mut T, renderer: &mut CanvasRenderer, dest: &mut Rect)
    where
        T: CanvasAccess,
    {
        let mut d = dest.clone();
        d.set_x(dest.x() + NAME_MARGIN);
        let font_details = build_font_details(self);
        let font = renderer.font_manager().load(&font_details).unwrap();
        let name = self.name();
        let config = self.config.read().unwrap();
        let text_color = config.theme().code_highlighting().title.color();

        for c in name.chars() {
            let size = renderer.load_character_size(c.clone());
            let mut text_details = TextDetails {
                color: Color::RGBA(text_color.r, text_color.g, text_color.b, text_color.a),
                text: c.to_string(),
                font: font_details.clone(),
            };
            let text_texture = renderer
                .texture_manager()
                .load_text(&mut text_details, font.clone())
                .unwrap();
            d.set_width(size.width());
            d.set_height(size.height());

            canvas
                .render_image(text_texture, self.source.clone(), d.clone())
                .unwrap_or_else(|_| panic!("Failed to draw directory entry texture"));
            d.set_x(d.x() + size.width() as i32);
        }
    }

    fn render_children<T>(&self, canvas: &mut T, renderer: &mut CanvasRenderer, dest: &mut Rect)
    where
        T: CanvasAccess,
    {
        if !self.expanded {
            return;
        }
        let mut point = dest.top_left()
            + Point::new(
                self.icon_width as i32 + CHILD_MARGIN,
                self.icon_height as i32 + CHILD_MARGIN,
            );
        for dir in self.directories.iter() {
            let context = RenderContext::RelativePosition(point.clone());
            dir.render(canvas, renderer, &context);
            point = point + Point::new(0, dir.height() as i32 + CHILD_MARGIN as i32);
        }
        for file in self.files.iter() {
            let context = RenderContext::RelativePosition(point.clone());
            file.render(canvas, renderer, &context);
            point = point + Point::new(0, file.height() as i32 + CHILD_MARGIN as i32);
        }
    }

    fn calculate_size(&mut self, renderer: &mut CanvasRenderer) {
        let size = renderer.load_character_size('W');
        self.height = size.height();
        self.icon_height = size.height();
        self.icon_width = size.height();
        self.name_width = 0;

        for c in self.name().chars() {
            let size = renderer.load_character_size(c.clone());
            self.name_width += size.width();
        }

        for dir in self.directories.iter_mut() {
            self.height = self.height + dir.height() + CHILD_MARGIN as u32;
        }
        for file in self.files.iter_mut() {
            self.height = self.height + file.height() + CHILD_MARGIN as u32;
        }
    }

    fn name_and_icon_rect(&self) -> Rect {
        Rect::new(
            self.pos.x(),
            self.pos.y(),
            self.icon_width + self.name_width + NAME_MARGIN as u32,
            self.icon_height,
        )
    }
}

impl ConfigHolder for DirectoryView {
    fn config(&self) -> &ConfigAccess {
        &self.config
    }
}

#[cfg_attr(tarpaulin, skip)]
impl DirectoryView {
    pub fn render<T>(&self, canvas: &mut T, renderer: &mut CanvasRenderer, context: &RenderContext)
    where
        T: CanvasAccess,
    {
        let dest = self.dest();
        let move_point = match context {
            &RenderContext::RelativePosition(p) => p.clone(),
            _ => Point::new(0, 0),
        };
        let mut dest = move_render_point(move_point, &dest);
        self.render_icon::<T>(canvas, renderer, &mut dest);
        self.render_name::<T>(canvas, renderer, &mut dest.clone());
        self.render_children::<T>(canvas, renderer, &mut dest);
    }

    pub fn prepare_ui(&mut self, renderer: &mut CanvasRenderer) {
        if self.opened {
            for dir in self.directories.iter_mut() {
                dir.prepare_ui(renderer);
            }
            for file in self.files.iter_mut() {
                file.prepare_ui(renderer);
            }
        }
        self.calculate_size(renderer);
    }
}

impl Update for DirectoryView {
    fn update(&mut self, ticks: i32, context: &UpdateContext) -> UpdateResult {
        if !path::Path::new(&self.path).exists() {
            return UpdateResult::RefreshFsTree;
        }
        if self.opened {
            for dir in self.directories.iter_mut() {
                dir.update(ticks, context);
            }
            for file in self.files.iter_mut() {
                file.update(ticks, context);
            }
        }
        UpdateResult::NoOp
    }
}

impl RenderBox for DirectoryView {
    fn render_start_point(&self) -> Point {
        self.pos.clone()
    }

    fn dest(&self) -> Rect {
        Rect::new(
            self.pos.x(),
            self.pos.y(),
            self.icon_width,
            self.icon_height,
        )
    }
}

impl ClickHandler for DirectoryView {
    fn on_left_click(&mut self, point: &Point, context: &UpdateContext) -> UpdateResult {
        let dest = self.dest();
        let move_point = match context {
            &UpdateContext::ParentPosition(p) => p.clone(),
            _ => Point::new(0, 0),
        };
        let dest = move_render_point(move_point.clone(), &dest);

        // icon or name is target of click
        let icon_or_name = self.name_and_icon_rect();
        if move_render_point(move_point, &icon_or_name).contains_point(point.clone()) {
            return UpdateResult::OpenDirectory(self.path.clone());
        }

        if !self.expanded {
            return UpdateResult::NoOp;
        }

        let mut p = dest.top_left()
            + Point::new(
                self.icon_width as i32 + CHILD_MARGIN,
                self.icon_height as i32 + CHILD_MARGIN,
            );
        for dir in self.directories.iter_mut() {
            let context = UpdateContext::ParentPosition(p.clone());
            if dir.is_left_click_target(&point, &context) {
                return dir.on_left_click(&point, &context);
            }
            p = p + Point::new(0, dir.height() as i32 + CHILD_MARGIN);
        }
        for file in self.files.iter_mut() {
            let context = UpdateContext::ParentPosition(p.clone());
            if file.is_left_click_target(&point, &context) {
                return file.on_left_click(&point, &context);
            }
            p = p + Point::new(0, file.height() as i32 + CHILD_MARGIN);
        }

        UpdateResult::NoOp
    }

    fn is_left_click_target(&self, point: &Point, context: &UpdateContext) -> bool {
        let dest = self.dest();
        let move_point = match context {
            UpdateContext::ParentPosition(p) => p.clone(),
            _ => Point::new(0, 0),
        };
        let dest = move_render_point(move_point.clone(), &dest);

        // icon or name is target of click
        let name_and_icon_rect = self.name_and_icon_rect();
        if move_render_point(move_point.clone(), &name_and_icon_rect).contains_point(point.clone())
        {
            return true;
        }
        if !self.expanded {
            return false;
        }
        let mut p = dest.top_left()
            + Point::new(
                self.icon_width as i32 + CHILD_MARGIN,
                self.icon_height as i32 + CHILD_MARGIN,
            );
        // subdirectory is target of click
        for dir in self.directories.iter() {
            let context = UpdateContext::ParentPosition(p.clone());
            if dir.is_left_click_target(&point, &context) {
                return true;
            }
            p = p + Point::new(0, dir.height() as i32 + CHILD_MARGIN);
        }
        // file inside directory is target of click
        for file in self.files.iter() {
            let context = UpdateContext::ParentPosition(p.clone());
            if file.is_left_click_target(&point, &context) {
                return true;
            }
            p = p + Point::new(0, file.height() as i32 + CHILD_MARGIN);
        }
        false
    }
}
