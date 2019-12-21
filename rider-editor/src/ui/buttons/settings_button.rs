use crate::app::UpdateResult as UR;
use crate::ui::{UpdateContext, Widget, WidgetInner};
use rider_config::ConfigAccess;
use sdl2::rect::{Point, Rect};

const ICON_DEST_WIDTH: u32 = 16;
const ICON_DEST_HEIGHT: u32 = 16;
const ICON_SRC_WIDTH: u32 = 16;
const ICON_SRC_HEIGHT: u32 = 16;

pub struct SettingsButton {
    inner: WidgetInner,
}

impl std::ops::Deref for SettingsButton {
    type Target = WidgetInner;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl std::ops::DerefMut for SettingsButton {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl Widget for SettingsButton {
    fn texture_path(&self) -> Option<String> {
        let c = self.config.read().unwrap();
        let mut themes_dir = c.directories().themes_dir.clone();
        let path = c.theme().images().settings_icon();
        themes_dir.push(path);
        Some(themes_dir.to_str().unwrap().to_owned())
    }

    fn dest(&self) -> &Rect {
        &self.dest
    }

    fn set_dest(&mut self, rect: &Rect) {
        self.inner.dest = rect.clone();
    }

    fn source(&self) -> &Rect {
        &self.source
    }

    fn set_source(&mut self, rect: &Rect) {
        self.inner.source = rect.clone();
    }

    fn on_left_click(&mut self, _point: &Point, _context: &UpdateContext) -> UR {
        UR::OpenSettings
    }

    fn padding_width(&self) -> u32 {
        ICON_DEST_WIDTH
    }

    fn padding_height(&self) -> u32 {
        ICON_DEST_HEIGHT
    }
}

impl SettingsButton {
    pub fn new(config: ConfigAccess) -> Self {
        Self {
            inner: WidgetInner::new(
                config,
                Rect::new(0, 0, ICON_SRC_WIDTH, ICON_SRC_HEIGHT),
                Rect::new(0, 0, ICON_DEST_WIDTH, ICON_DEST_HEIGHT),
            ),
        }
    }
}
