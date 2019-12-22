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
        self.inner.dest.height()
    }

    #[inline]
    pub fn width(&self) -> u32 {
        self.inner.dest.width()
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
}
