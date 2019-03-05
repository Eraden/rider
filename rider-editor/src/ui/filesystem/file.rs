use crate::app::*;
use crate::renderer::*;
use crate::ui::*;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use std::collections::HashMap;
use std::path;

const ICON_DEST_WIDTH: u32 = 16;
const ICON_DEST_HEIGHT: u32 = 16;
const ICON_SRC_WIDTH: u32 = 64;
const ICON_SRC_HEIGHT: u32 = 64;

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
            dest: Rect::new(0, 0, ICON_DEST_WIDTH, ICON_DEST_HEIGHT),
            source: Rect::new(0, 0, ICON_SRC_WIDTH, ICON_SRC_HEIGHT),
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

    fn render_icon<C, R>(&self, canvas: &mut C, renderer: &mut R, dest: &mut Rect)
    where
        C: CanvasAccess,
        R: Renderer,
    {
        let dir_texture_path = {
            let c = self.config.read().unwrap();
            let mut themes_dir = c.directories().themes_dir.clone();
            let path = c.theme().images().file_icon();
            themes_dir.push(path);
            themes_dir.to_str().unwrap().to_owned()
        };
        let maybe_tex = renderer.load_image(dir_texture_path.clone());
        if let Ok(texture) = maybe_tex {
            dest.set_width(ICON_DEST_WIDTH);
            dest.set_height(ICON_DEST_HEIGHT);
            canvas
                .render_image(texture, self.source.clone(), dest.clone())
                .unwrap_or_else(|_| panic!("Failed to draw directory entry texture"));
        }
    }

    fn render_name<C, R>(&self, canvas: &mut C, renderer: &mut R, dest: &mut Rect)
    where
        C: CanvasAccess,
        R: Renderer,
    {
        let mut d = dest.clone();
        d.set_x(dest.x() + NAME_MARGIN);

        let font_details = build_font_details(self);
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
            let maybe_texture = renderer.load_text_tex(&mut text_details, font_details.clone());

            if let Ok(texture) = maybe_texture {
                d.set_width(size.width());
                d.set_height(size.height());

                canvas
                    .render_image(texture, self.source.clone(), d.clone())
                    .unwrap_or_else(|_| panic!("Failed to draw directory entry texture"));
                d.set_x(d.x() + size.width() as i32)
            }
        }
    }

    pub fn render<C, R>(&self, canvas: &mut C, renderer: &mut R, context: &RenderContext)
    where
        C: CanvasAccess,
        R: Renderer,
    {
        let mut dest = match context {
            &RenderContext::RelativePosition(p) => move_render_point(p.clone(), &self.dest),
            _ => self.dest.clone(),
        };
        self.render_icon(canvas, renderer, &mut dest);
        self.render_name(canvas, renderer, &mut dest.clone());
    }

    pub fn prepare_ui<R>(&mut self, renderer: &mut R)
    where
        R: Renderer + CharacterSizeManager,
    {
        let w_rect = renderer.load_character_size('W');
        self.char_sizes.insert('W', w_rect.clone());
        self.height = w_rect.height();
        self.icon_width = w_rect.height();
        self.name_width = 0;

        for c in self.name().chars() {
            let size = { renderer.load_character_size(c.clone()) };
            self.char_sizes.insert(c, size);
            self.name_width += size.width();
        }
        self.dest.set_width(w_rect.height());
        self.dest.set_height(w_rect.height());
    }

    pub fn update(&mut self) -> UpdateResult {
        if !path::Path::new(&self.path).exists() {
            return UpdateResult::RefreshFsTree;
        }
        UpdateResult::NoOp
    }

    pub fn render_start_point(&self) -> Point {
        self.dest.top_left()
    }

    pub fn on_left_click(&mut self) -> UpdateResult {
        UpdateResult::OpenFile(self.path.clone())
    }

    pub fn is_left_click_target(&self, point: &Point, context: &UpdateContext) -> bool {
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

impl ConfigHolder for FileEntry {
    fn config(&self) -> &ConfigAccess {
        &self.config
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::support::build_config;
    use crate::tests::support::CanvasMock;
    use crate::tests::support::SimpleRendererMock;

    //##########################################################
    // name_width
    //##########################################################

    #[test]
    fn assert_initial_name_width() {
        let config = build_config();
        let widget = FileEntry::new("bar.txt".to_owned(), "/foo".to_owned(), config);
        assert_eq!(widget.name_width(), 0);
    }

    #[test]
    fn assert_prepared_name_width() {
        let config = build_config();
        let mut renderer = SimpleRendererMock::new(config.clone());
        let mut widget = FileEntry::new("bar.txt".to_owned(), "/foo".to_owned(), config);
        widget.prepare_ui(&mut renderer);
        assert_eq!(widget.name_width(), 91);
    }

    //##########################################################
    // icon_width
    //##########################################################

    #[test]
    fn assert_initial_icon_width() {
        let config = build_config();
        let widget = FileEntry::new("bar.txt".to_owned(), "/foo".to_owned(), config);
        assert_eq!(widget.icon_width(), 0);
    }

    #[test]
    fn assert_prepared_icon_width() {
        let config = build_config();
        let mut renderer = SimpleRendererMock::new(config.clone());
        let mut widget = FileEntry::new("bar.txt".to_owned(), "/foo".to_owned(), config);
        widget.prepare_ui(&mut renderer);
        assert_eq!(widget.icon_width(), 14);
    }

    //##########################################################
    // height
    //##########################################################

    #[test]
    fn assert_initial_height() {
        let config = build_config();
        let widget = FileEntry::new("bar.txt".to_owned(), "/foo".to_owned(), config);
        assert_eq!(widget.height(), 0);
    }

    #[test]
    fn assert_prepared_height() {
        let config = build_config();
        let mut renderer = SimpleRendererMock::new(config.clone());
        let mut widget = FileEntry::new("bar.txt".to_owned(), "/foo".to_owned(), config);
        widget.prepare_ui(&mut renderer);
        assert_eq!(widget.height(), 14);
    }

    //##########################################################
    // name
    //##########################################################

    #[test]
    fn assert_initial_name() {
        let config = build_config();
        let widget = FileEntry::new("bar.txt".to_owned(), "/foo".to_owned(), config);
        assert_eq!(widget.name(), "bar.txt".to_owned());
    }

    #[test]
    fn assert_prepared_name() {
        let config = build_config();
        let mut renderer = SimpleRendererMock::new(config.clone());
        let mut widget = FileEntry::new("bar.txt".to_owned(), "/foo".to_owned(), config);
        widget.prepare_ui(&mut renderer);
        assert_eq!(widget.name(), "bar.txt".to_owned());
    }

    //##########################################################
    // path
    //##########################################################

    #[test]
    fn assert_initial_path() {
        let config = build_config();
        let widget = FileEntry::new("bar.txt".to_owned(), "/foo".to_owned(), config);
        assert_eq!(widget.path(), "/foo".to_owned());
    }

    #[test]
    fn assert_prepared_path() {
        let config = build_config();
        let mut renderer = SimpleRendererMock::new(config.clone());
        let mut widget = FileEntry::new("bar.txt".to_owned(), "/foo".to_owned(), config);
        widget.prepare_ui(&mut renderer);
        assert_eq!(widget.path(), "/foo".to_owned());
    }

    //##########################################################
    // source
    //##########################################################

    #[test]
    fn assert_initial_source() {
        let config = build_config();
        let widget = FileEntry::new("bar.txt".to_owned(), "/foo".to_owned(), config);
        assert_eq!(widget.source(), &Rect::new(0, 0, 64, 64));
    }

    #[test]
    fn assert_prepared_source() {
        let config = build_config();
        let mut renderer = SimpleRendererMock::new(config.clone());
        let mut widget = FileEntry::new("bar.txt".to_owned(), "/foo".to_owned(), config);
        widget.prepare_ui(&mut renderer);
        assert_eq!(widget.source(), &Rect::new(0, 0, 64, 64));
    }

    //##########################################################
    // dest
    //##########################################################

    #[test]
    fn assert_initial_dest() {
        let config = build_config();
        let widget = FileEntry::new("bar.txt".to_owned(), "/foo".to_owned(), config);
        assert_eq!(widget.dest(), &Rect::new(0, 0, 16, 16));
    }

    #[test]
    fn assert_prepared_dest() {
        let config = build_config();
        let mut renderer = SimpleRendererMock::new(config.clone());
        let mut widget = FileEntry::new("bar.txt".to_owned(), "/foo".to_owned(), config);
        widget.prepare_ui(&mut renderer);
        assert_eq!(widget.dest(), &Rect::new(0, 0, 14, 14));
    }

    //##########################################################
    // full_dest
    //##########################################################

    #[test]
    fn assert_initial_full_dest() {
        let config = build_config();
        let widget = FileEntry::new("bar.txt".to_owned(), "/foo".to_owned(), config);
        assert_eq!(widget.full_dest(), Rect::new(0, 0, 20, 1));
    }

    #[test]
    fn assert_prepared_full_dest() {
        let config = build_config();
        let mut renderer = SimpleRendererMock::new(config.clone());
        let mut widget = FileEntry::new("bar.txt".to_owned(), "/foo".to_owned(), config);
        widget.prepare_ui(&mut renderer);
        assert_eq!(widget.full_dest(), Rect::new(0, 0, 125, 14));
    }

    //##########################################################
    // update
    //##########################################################

    #[test]
    fn assert_update_when_doesnt_exists() {
        let config = build_config();
        let mut renderer = SimpleRendererMock::new(config.clone());
        let mut widget = FileEntry::new("bar.txt".to_owned(), "/foo".to_owned(), config);
        widget.prepare_ui(&mut renderer);
        assert_eq!(widget.update(), UpdateResult::RefreshFsTree);
    }

    #[test]
    fn assert_update_when_does_exists() {
        let config = build_config();
        let mut renderer = SimpleRendererMock::new(config.clone());
        let mut widget = FileEntry::new("bar.txt".to_owned(), "/tmp".to_owned(), config);
        widget.prepare_ui(&mut renderer);
        assert_eq!(widget.update(), UpdateResult::NoOp);
    }

    //##########################################################
    // render
    //##########################################################

    #[test]
    fn assert_render() {
        let config = build_config();
        let mut renderer = SimpleRendererMock::new(config.clone());
        let mut canvas = CanvasMock::new();
        let mut widget = FileEntry::new("bar.txt".to_owned(), "/foo".to_owned(), config);
        widget.prepare_ui(&mut renderer);
        widget.render(&mut canvas, &mut renderer, &RenderContext::Nothing);
        assert_eq!(widget.full_dest(), Rect::new(0, 0, 125, 14));
    }

    //##########################################################
    // is_left_click_target
    //##########################################################

    #[test]
    fn assert_is_left_click_target_when_target() {
        let config = build_config();
        let mut renderer = SimpleRendererMock::new(config.clone());
        let mut canvas = CanvasMock::new();
        let mut widget = FileEntry::new("bar.txt".to_owned(), "/foo".to_owned(), config);
        widget.prepare_ui(&mut renderer);
        widget.render(&mut canvas, &mut renderer, &RenderContext::Nothing);
        let p = Point::new(0, 0);
        let context = UpdateContext::Nothing;
        assert_eq!(widget.is_left_click_target(&p, &context), true);
    }

    #[test]
    fn assert_is_left_click_target_when_target_with_parent() {
        let config = build_config();
        let mut renderer = SimpleRendererMock::new(config.clone());
        let mut canvas = CanvasMock::new();
        let mut widget = FileEntry::new("bar.txt".to_owned(), "/foo".to_owned(), config);
        widget.prepare_ui(&mut renderer);
        widget.render(&mut canvas, &mut renderer, &RenderContext::Nothing);
        let p = Point::new(0, 0);
        let context = UpdateContext::ParentPosition(Point::new(0, 0));
        assert_eq!(widget.is_left_click_target(&p, &context), true);
    }

    #[test]
    fn refute_is_left_click_target_when_target() {
        let config = build_config();
        let mut renderer = SimpleRendererMock::new(config.clone());
        let mut canvas = CanvasMock::new();
        let mut widget = FileEntry::new("bar.txt".to_owned(), "/foo".to_owned(), config);
        widget.prepare_ui(&mut renderer);
        widget.render(&mut canvas, &mut renderer, &RenderContext::Nothing);
        let p = Point::new(9000, 0);
        let context = UpdateContext::Nothing;
        assert_eq!(widget.is_left_click_target(&p, &context), false);
    }

    #[test]
    fn refute_is_left_click_target_when_target_with_parent() {
        let config = build_config();
        let mut renderer = SimpleRendererMock::new(config.clone());
        let mut canvas = CanvasMock::new();
        let mut widget = FileEntry::new("bar.txt".to_owned(), "/foo".to_owned(), config);
        widget.prepare_ui(&mut renderer);
        widget.render(&mut canvas, &mut renderer, &RenderContext::Nothing);
        let p = Point::new(0, 9000);
        let context = UpdateContext::ParentPosition(Point::new(0, 0));
        assert_eq!(widget.is_left_click_target(&p, &context), false);
    }
}
