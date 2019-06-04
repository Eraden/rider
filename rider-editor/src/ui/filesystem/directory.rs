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

    //    pub fn dest(&self) -> Rect {
    //        Rect::new(
    //            self.pos.x(),
    //            self.pos.y(),
    //            self.icon_width,
    //            self.icon_height,
    //        )
    //    }

    pub fn source(&self) -> &Rect {
        &self.source
    }

    pub fn open_directory<R>(&mut self, dir_path: String, renderer: &mut R) -> bool
    where
        R: Renderer + CharacterSizeManager,
    {
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

    fn read_directory<R>(&mut self, renderer: &mut R)
    where
        R: Renderer + CharacterSizeManager,
    {
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

    fn render_icon<C, R>(&self, canvas: &mut C, renderer: &mut R, dest: &mut Rect)
    where
        C: CanvasAccess,
        R: Renderer,
    {
        let dir_texture_path = {
            let c = self.config.read().unwrap();
            let mut themes_dir = c.directories().themes_dir.clone();
            let path = c.theme().images().directory_icon();
            themes_dir.push(path);
            themes_dir.to_str().unwrap().to_owned()
        };
        if let Ok(texture) = renderer.load_image(dir_texture_path.clone()) {
            canvas
                .render_image(
                    texture,
                    self.source.clone(),
                    Rect::new(dest.x(), dest.y(), self.icon_width, self.icon_height),
                )
                .unwrap_or_else(|_| panic!("Failed to draw directory entry texture"));
        }
    }

    fn render_name<C, R>(&self, canvas: &mut C, renderer: &mut R, dest: &mut Rect)
    where
        C: CanvasAccess,
        R: Renderer + CharacterSizeManager,
    {
        let mut d = dest.clone();
        d.set_x(dest.x() + NAME_MARGIN);
        let font_details = build_font_details(self);
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

    fn render_children<C, R>(&self, canvas: &mut C, renderer: &mut R, dest: &mut Rect)
    where
        C: CanvasAccess,
        R: Renderer + CharacterSizeManager,
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
            let context = RenderContext::ParentPosition(point.clone());
            dir.render(canvas, renderer, &context);
            point = point + Point::new(0, dir.height() as i32 + CHILD_MARGIN as i32);
        }
        for file in self.files.iter() {
            let context = RenderContext::ParentPosition(point.clone());
            file.render(canvas, renderer, &context);
            point = point + Point::new(0, file.height() as i32 + CHILD_MARGIN as i32);
        }
    }

    fn calculate_size<R>(&mut self, renderer: &mut R)
    where
        R: CharacterSizeManager,
    {
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

    pub fn render<R, C>(&self, canvas: &mut C, renderer: &mut R, context: &RenderContext)
    where
        R: Renderer + CharacterSizeManager,
        C: CanvasAccess,
    {
        let dest = self.dest();
        let move_point = match context {
            &RenderContext::ParentPosition(p) => p.clone(),
            _ => Point::new(0, 0),
        };
        let mut dest = move_render_point(move_point, &dest);
        self.render_icon::<C, R>(canvas, renderer, &mut dest);
        self.render_name::<C, R>(canvas, renderer, &mut dest.clone());
        self.render_children::<C, R>(canvas, renderer, &mut dest);
    }

    pub fn prepare_ui<R>(&mut self, renderer: &mut R)
    where
        R: Renderer + CharacterSizeManager,
    {
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

    pub fn update(&mut self, ticks: i32, context: &UpdateContext) -> UpdateResult {
        if !path::Path::new(&self.path).exists() {
            return UpdateResult::RefreshFsTree;
        }
        if self.opened {
            for dir in self.directories.iter_mut() {
                dir.update(ticks, context);
            }
            for file in self.files.iter_mut() {
                file.update();
            }
        }
        UpdateResult::NoOp
    }

    pub fn render_start_point(&self) -> Point {
        self.pos.clone()
    }

    pub fn on_left_click(&mut self, point: &Point, context: &UpdateContext) -> UpdateResult {
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
                return file.on_left_click();
            }
            p = p + Point::new(0, file.height() as i32 + CHILD_MARGIN);
        }

        UpdateResult::NoOp
    }

    pub fn is_left_click_target(&self, point: &Point, context: &UpdateContext) -> bool {
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

impl ConfigHolder for DirectoryView {
    fn config(&self) -> &ConfigAccess {
        &self.config
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::support::CanvasMock;
    use crate::tests::support::SimpleRendererMock;
    use crate::tests::support::{build_config, build_path};

    //##########################################################
    // name_width
    //##########################################################

    #[test]
    fn assert_initial_name_width() {
        let config = build_config();
        let widget = DirectoryView::new("/foo".to_owned(), config);
        assert_eq!(widget.name_width(), 0);
    }

    #[test]
    fn assert_prepared_name_width() {
        let config = build_config();
        let mut renderer = SimpleRendererMock::new(config.clone());
        let mut widget = DirectoryView::new("/foo".to_owned(), config);
        widget.prepare_ui(&mut renderer);
        assert_eq!(widget.name_width(), 39);
    }

    //##########################################################
    // icon_width
    //##########################################################

    #[test]
    fn assert_initial_icon_width() {
        let config = build_config();
        let widget = DirectoryView::new("/foo".to_owned(), config);
        assert_eq!(widget.icon_width(), 16);
    }

    #[test]
    fn assert_prepared_icon_width() {
        let config = build_config();
        let mut renderer = SimpleRendererMock::new(config.clone());
        let mut widget = DirectoryView::new("/foo".to_owned(), config);
        widget.prepare_ui(&mut renderer);
        assert_eq!(widget.icon_width(), 14);
    }

    //##########################################################
    // height
    //##########################################################

    #[test]
    fn assert_initial_height() {
        let config = build_config();
        let widget = DirectoryView::new("/foo".to_owned(), config);
        assert_eq!(widget.height(), 16);
    }

    #[test]
    fn assert_prepared_height() {
        let config = build_config();
        let mut renderer = SimpleRendererMock::new(config.clone());
        let mut widget = DirectoryView::new("/foo".to_owned(), config);
        widget.prepare_ui(&mut renderer);
        assert_eq!(widget.height(), 14);
    }

    //##########################################################
    // name
    //##########################################################

    #[test]
    fn assert_initial_name() {
        let config = build_config();
        let widget = DirectoryView::new("/foo".to_owned(), config);
        assert_eq!(widget.name(), "foo".to_owned());
    }

    #[test]
    fn assert_prepared_name() {
        let config = build_config();
        let mut renderer = SimpleRendererMock::new(config.clone());
        let mut widget = DirectoryView::new("/foo".to_owned(), config);
        widget.prepare_ui(&mut renderer);
        assert_eq!(widget.name(), "foo".to_owned());
    }

    //##########################################################
    // path
    //##########################################################

    #[test]
    fn assert_initial_path() {
        let config = build_config();
        let widget = DirectoryView::new("/foo".to_owned(), config);
        assert_eq!(widget.path(), "/foo".to_owned());
    }

    #[test]
    fn assert_prepared_path() {
        let config = build_config();
        let mut renderer = SimpleRendererMock::new(config.clone());
        let mut widget = DirectoryView::new("/foo".to_owned(), config);
        widget.prepare_ui(&mut renderer);
        assert_eq!(widget.path(), "/foo".to_owned());
    }

    //##########################################################
    // source
    //##########################################################

    #[test]
    fn assert_initial_source() {
        let config = build_config();
        let widget = DirectoryView::new("/foo".to_owned(), config);
        assert_eq!(widget.source(), &Rect::new(0, 0, 64, 64));
    }

    #[test]
    fn assert_prepared_source() {
        let config = build_config();
        let mut renderer = SimpleRendererMock::new(config.clone());
        let mut widget = DirectoryView::new("/foo".to_owned(), config);
        widget.prepare_ui(&mut renderer);
        assert_eq!(widget.source(), &Rect::new(0, 0, 64, 64));
    }

    //##########################################################
    // dest
    //##########################################################

    #[test]
    fn assert_initial_dest() {
        let config = build_config();
        let widget = DirectoryView::new("/foo".to_owned(), config);
        assert_eq!(widget.dest(), Rect::new(0, 0, 36, 16));
    }

    #[test]
    fn assert_prepared_dest() {
        let config = build_config();
        let mut renderer = SimpleRendererMock::new(config.clone());
        let mut widget = DirectoryView::new("/foo".to_owned(), config);
        widget.prepare_ui(&mut renderer);
        assert_eq!(widget.dest(), Rect::new(0, 0, 73, 14));
    }

    //##########################################################
    // update
    //##########################################################

    #[test]
    fn assert_update_when_doesnt_exists() {
        let config = build_config();
        let mut renderer = SimpleRendererMock::new(config.clone());
        let mut widget = DirectoryView::new("/foo".to_owned(), config);
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
        let mut widget = DirectoryView::new("/tmp".to_owned(), config);
        widget.prepare_ui(&mut renderer);
        assert_eq!(
            widget.update(0, &UpdateContext::Nothing),
            UpdateResult::NoOp
        );
    }

    #[test]
    fn assert_update_expanded() {
        build_path("/tmp/rider-editor/directory-view-test".to_owned());

        let config = build_config();
        let mut renderer = SimpleRendererMock::new(config.clone());
        let mut widget =
            DirectoryView::new("/tmp/rider-editor/directory-view-test".to_owned(), config);
        widget.prepare_ui(&mut renderer);
        widget.open_directory(
            "/tmp/rider-editor/directory-view-test".to_owned(),
            &mut renderer,
        );
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
    fn assert_render_no_expanded() {
        build_path("/tmp/rider-editor/directory-view-test".to_owned());

        let config = build_config();
        let mut renderer = SimpleRendererMock::new(config.clone());
        let mut canvas = CanvasMock::new();
        let mut widget =
            DirectoryView::new("/tmp/rider-editor/directory-view-test".to_owned(), config);
        widget.prepare_ui(&mut renderer);
        widget.render(&mut canvas, &mut renderer, &RenderContext::Nothing);
    }

    #[test]
    fn assert_render_expanded() {
        build_path("/tmp/rider-editor/directory-view-test".to_owned());

        let config = build_config();
        let mut renderer = SimpleRendererMock::new(config.clone());
        let mut canvas = CanvasMock::new();
        let mut widget =
            DirectoryView::new("/tmp/rider-editor/directory-view-test".to_owned(), config);
        widget.prepare_ui(&mut renderer);
        widget.open_directory(
            "/tmp/rider-editor/directory-view-test".to_owned(),
            &mut renderer,
        );
        widget.prepare_ui(&mut renderer);
        widget.render(&mut canvas, &mut renderer, &RenderContext::Nothing);
    }

    //##########################################################
    // is_left_click_target
    //##########################################################

    #[test]
    fn assert_is_left_click_target_when_target() {
        build_path("/tmp/rider-editor/directory-view-test".to_owned());

        let config = build_config();
        let mut renderer = SimpleRendererMock::new(config.clone());
        let mut canvas = CanvasMock::new();
        let mut widget = DirectoryView::new("/foo".to_owned(), config);
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
        let mut widget = DirectoryView::new("/foo".to_owned(), config);
        widget.prepare_ui(&mut renderer);
        widget.render(&mut canvas, &mut renderer, &RenderContext::Nothing);
        let p = Point::new(0, 0);
        let context = UpdateContext::ParentPosition(Point::new(0, 0));
        assert_eq!(widget.is_left_click_target(&p, &context), true);
    }

    #[test]
    fn assert_is_left_click_target_expanded() {
        build_path("/tmp/rider-editor/directory-view-test".to_owned());

        let config = build_config();
        let mut renderer = SimpleRendererMock::new(config.clone());
        let mut canvas = CanvasMock::new();
        let mut widget =
            DirectoryView::new("/tmp/rider-editor/directory-view-test".to_owned(), config);
        widget.prepare_ui(&mut renderer);
        widget.open_directory(
            "/tmp/rider-editor/directory-view-test".to_owned(),
            &mut renderer,
        );
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
        let mut widget = DirectoryView::new("/foo".to_owned(), config);
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
        let mut widget = DirectoryView::new("/foo".to_owned(), config);
        widget.prepare_ui(&mut renderer);
        widget.render(&mut canvas, &mut renderer, &RenderContext::Nothing);
        let p = Point::new(0, 9000);
        let context = UpdateContext::ParentPosition(Point::new(0, 0));
        assert_eq!(widget.is_left_click_target(&p, &context), false);
    }

    //##########################################################
    // on_left_click
    //##########################################################

    #[test]
    fn assert_on_left_click_when_target() {
        build_path("/tmp/rider-editor/directory-view-test".to_owned());

        let config = build_config();
        let mut renderer = SimpleRendererMock::new(config.clone());
        let mut canvas = CanvasMock::new();
        let mut widget =
            DirectoryView::new("/tmp/rider-editor/directory-view-test".to_owned(), config);
        widget.prepare_ui(&mut renderer);
        widget.render(&mut canvas, &mut renderer, &RenderContext::Nothing);
        let p = Point::new(0, 0);
        let context = UpdateContext::Nothing;
        assert_eq!(
            widget.on_left_click(&p, &context),
            UpdateResult::OpenDirectory("/tmp/rider-editor/directory-view-test".to_owned())
        );
    }

    #[test]
    fn assert_on_left_click_when_target_with_parent() {
        build_path("/tmp/rider-editor/directory-view-test".to_owned());

        let config = build_config();
        let mut renderer = SimpleRendererMock::new(config.clone());
        let mut canvas = CanvasMock::new();
        let mut widget =
            DirectoryView::new("/tmp/rider-editor/directory-view-test".to_owned(), config);
        widget.prepare_ui(&mut renderer);
        widget.render(&mut canvas, &mut renderer, &RenderContext::Nothing);
        let p = Point::new(0, 0);
        let context = UpdateContext::ParentPosition(Point::new(0, 0));
        assert_eq!(
            widget.on_left_click(&p, &context),
            UpdateResult::OpenDirectory("/tmp/rider-editor/directory-view-test".to_owned())
        );
    }

    #[test]
    fn assert_on_left_click_expanded() {
        build_path("/tmp/rider-editor/directory-view-test".to_owned());

        let config = build_config();
        let mut renderer = SimpleRendererMock::new(config.clone());
        let mut canvas = CanvasMock::new();
        let mut widget =
            DirectoryView::new("/tmp/rider-editor/directory-view-test".to_owned(), config);
        widget.prepare_ui(&mut renderer);
        widget.open_directory(
            "/tmp/rider-editor/directory-view-test".to_owned(),
            &mut renderer,
        );
        widget.prepare_ui(&mut renderer);
        widget.render(&mut canvas, &mut renderer, &RenderContext::Nothing);
        let p = Point::new(0, 0);
        let context = UpdateContext::ParentPosition(Point::new(0, 0));
        assert_eq!(
            widget.on_left_click(&p, &context),
            UpdateResult::OpenDirectory("/tmp/rider-editor/directory-view-test".to_owned())
        );
    }
}
