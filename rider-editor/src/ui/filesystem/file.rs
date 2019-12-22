use crate::app::*;
use crate::renderer::*;
use crate::ui::icon::Icon;
use crate::ui::*;
use sdl2::rect::{Point, Rect};
use std::path;

const ICON_DEST_WIDTH: u32 = 16;
const ICON_DEST_HEIGHT: u32 = 16;
const ICON_SRC_WIDTH: u32 = 64;
const ICON_SRC_HEIGHT: u32 = 64;

pub struct FileEntry {
    path: String,
    inner: WidgetInner,
    icon: Icon,
    label: Label,
}

impl std::ops::Deref for FileEntry {
    type Target = WidgetInner;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl std::ops::DerefMut for FileEntry {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl Widget for FileEntry {
    fn texture_path(&self) -> Option<String> {
        None
    }

    fn dest(&self) -> &Rect {
        &self.inner.dest
    }

    fn set_dest(&mut self, rect: &Rect) {
        self.inner.dest = rect.clone();
    }

    fn source(&self) -> &Rect {
        &self.inner.source
    }

    fn set_source(&mut self, rect: &Rect) {
        self.inner.source = rect.clone();
    }

    fn update(&mut self, _ticks: i32, _context: &UpdateContext) -> UpdateResult {
        if !path::Path::new(&self.path).exists() {
            return UpdateResult::RefreshFsTree;
        }
        UpdateResult::NoOp
    }

    fn render<C, R>(&self, canvas: &mut C, renderer: &mut R, context: &RenderContext)
    where
        C: CanvasAccess,
        R: Renderer + CharacterSizeManager,
    {
        let dest = match context {
            &RenderContext::ParentPosition(p) => move_render_point(p.clone(), self.dest()),
            _ => self.dest.clone(),
        };
        self.icon.render(
            canvas,
            renderer,
            &RenderContext::ParentPosition(Point::new(dest.x(), dest.y())),
        );
        self.label.render(
            canvas,
            renderer,
            &RenderContext::ParentPosition(Point::new(dest.x() + NAME_MARGIN, dest.y())),
        )
    }

    fn prepare_ui<R>(&mut self, renderer: &mut R)
    where
        R: Renderer + CharacterSizeManager,
    {
        let rect = renderer.load_character_size('W');
        self.icon.prepare_ui(renderer);
        self.icon.dest.set_height(rect.height());
        self.icon.dest.set_width(rect.height());
        self.label.prepare_ui(renderer);
        let old = self.inner.dest.clone();
        self.inner.dest = Rect::new(
            old.x(),
            old.y(),
            self.name_width() + self.icon_width(),
            self.height(),
        );
    }
}

impl FileEntry {
    pub fn new(name: String, path: String, config: ConfigAccess) -> Self {
        let texture_path = {
            let c = config.read().unwrap();
            let mut themes_dir = c.directories().themes_dir.clone();
            let path = c.theme().images().file_icon();
            themes_dir.push(path);
            themes_dir.to_str().unwrap().to_owned()
        };
        Self {
            path,
            inner: WidgetInner::new(
                config.clone(),
                Rect::new(0, 0, ICON_SRC_WIDTH, ICON_SRC_HEIGHT),
                Rect::new(0, 0, ICON_DEST_WIDTH, ICON_DEST_HEIGHT),
            ),
            icon: Icon::new(
                config.clone(),
                texture_path,
                Rect::new(0, 0, ICON_SRC_WIDTH, ICON_SRC_HEIGHT),
                Rect::new(0, 0, ICON_DEST_WIDTH, ICON_DEST_HEIGHT),
            ),
            label: Label::new(name.clone(), config),
        }
    }

    #[inline]
    pub fn name_width(&self) -> u32 {
        self.label.name_width()
    }

    #[inline]
    pub fn icon_width(&self) -> u32 {
        self.icon.width()
    }

    #[inline]
    pub fn height(&self) -> u32 {
        self.dest().height()
    }

    #[inline]
    pub fn name(&self) -> String {
        self.label.name()
    }

    pub fn path(&self) -> String {
        self.path.clone()
    }

    pub fn full_dest(&self) -> Rect {
        Rect::new(
            self.dest.x(),
            self.dest.y(),
            self.icon.width() + NAME_MARGIN as u32 + self.label.name_width(),
            self.height(),
        )
    }

    pub fn on_left_click(&mut self) -> UpdateResult {
        UpdateResult::OpenFile(self.path.clone())
    }

    pub fn is_left_click_target(&self, point: &Point, context: &UpdateContext) -> bool {
        let dest = Rect::new(
            self.dest.x(),
            self.dest.y(),
            self.icon_width() + self.name_width() + NAME_MARGIN as u32,
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
    use crate::ui::{UpdateContext, Widget};

    //##########################################################
    // name_width
    //##########################################################

    #[test]
    fn assert_initial_name_width() {
        let config = build_config();
        let widget = FileEntry::new("bar.txt".to_owned(), "/foo".to_owned(), config);
        assert_eq!(widget.name_width(), 16);
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
        assert_eq!(widget.icon_width(), 16);
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
        assert_eq!(widget.height(), 16);
    }

    #[test]
    fn assert_prepared_height() {
        let config = build_config();
        let mut renderer = SimpleRendererMock::new(config.clone());
        let mut widget = FileEntry::new("bar.txt".to_owned(), "/foo".to_owned(), config);
        widget.prepare_ui(&mut renderer);
        assert_eq!(widget.height(), 16);
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
        assert_eq!(widget.dest(), &Rect::new(0, 0, 105, 16));
    }

    //##########################################################
    // full_dest
    //##########################################################

    #[test]
    fn assert_initial_full_dest() {
        let config = build_config();
        let widget = FileEntry::new("bar.txt".to_owned(), "/foo".to_owned(), config);
        assert_eq!(widget.full_dest(), Rect::new(0, 0, 52, 16));
    }

    #[test]
    fn assert_prepared_full_dest() {
        let config = build_config();
        let mut renderer = SimpleRendererMock::new(config.clone());
        let mut widget = FileEntry::new("bar.txt".to_owned(), "/foo".to_owned(), config);
        widget.prepare_ui(&mut renderer);
        assert_eq!(widget.full_dest(), Rect::new(0, 0, 125, 16));
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
        assert_eq!(
            widget.update(0, &UpdateContext::Nothing),
            UpdateResult::RefreshFsTree
        );
    }

    #[test]
    fn assert_update_when_does_exists() {
        let config = build_config();
        let mut renderer = SimpleRendererMock::new(config.clone());
        let mut widget = FileEntry::new("bar.txt".to_owned(), "/tmp".to_owned(), config);
        widget.prepare_ui(&mut renderer);
        assert_eq!(
            widget.update(0, &UpdateContext::Nothing),
            UpdateResult::NoOp
        );
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
        assert_eq!(widget.full_dest(), Rect::new(0, 0, 125, 16));
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
