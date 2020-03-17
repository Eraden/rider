use crate::ui::{Widget, WidgetInner};
use rider_config::ConfigAccess;
use sdl2::rect::Rect;

pub struct Icon {
    path: String,
    inner: WidgetInner,
}

impl std::ops::Deref for Icon {
    type Target = WidgetInner;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl std::ops::DerefMut for Icon {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl Icon {
    pub fn new(config: ConfigAccess, path: String, source: Rect, dest: Rect) -> Self {
        Self {
            path,
            inner: WidgetInner::new(config, source, dest),
        }
    }

    #[inline]
    pub fn height(&self) -> u32 {
        self.dest.height()
    }

    #[inline]
    pub fn width(&self) -> u32 {
        self.dest.width()
    }

    pub fn set_texture_path(&mut self, path: String) {
        self.path = path;
    }
}

impl Widget for Icon {
    fn texture_path(&self) -> Option<String> {
        Some(self.path.clone())
    }

    fn dest(&self) -> &Rect {
        &self.dest
    }

    fn set_dest(&mut self, rect: &Rect) {
        self.dest = rect.clone();
    }

    fn source(&self) -> &Rect {
        &self.source
    }

    fn set_source(&mut self, rect: &Rect) {
        self.source = rect.clone();
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::app::UpdateResult;
    use crate::tests::*;
    use crate::ui::UpdateContext;
    use sdl2::rect::Point;

    #[test]
    fn must_return_noop_on_left_click() {
        let config = build_config();
        let dest = Rect::new(0, 10, 20, 30);
        let src = Rect::new(40, 50, 60, 70);
        let path = "/foo/bar.png".to_owned();
        let mut widget = Icon::new(config, path, src, dest.clone());
        assert_eq!(
            widget.on_left_click(&Point::new(0, 0), &UpdateContext::Nothing),
            UpdateResult::NoOp
        );
    }

    #[test]
    fn must_use_inner() {
        let config = build_config();
        let dest = Rect::new(0, 10, 20, 30);
        let src = Rect::new(40, 50, 60, 70);
        let path = "/foo/bar.png".to_owned();
        let mut widget = Icon::new(config, path, src, dest.clone());

        assert_eq!(widget.dest(), &dest);
        widget.set_dest(&Rect::new(1, 2, 3, 4));
        assert_eq!(widget.dest(), &Rect::new(1, 2, 3, 4));

        assert_eq!(widget.source(), &src);
        widget.set_source(&Rect::new(5, 6, 7, 8));
        assert_eq!(widget.source(), &Rect::new(5, 6, 7, 8));
    }
}
