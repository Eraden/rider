use crate::app::{ConfigAccess, UpdateResult as UR};
use crate::ui::{UpdateContext, Widget, WidgetInner};
use sdl2::rect::{Point, Rect};

const ICON_DEST_WIDTH: u32 = 16;
const ICON_DEST_HEIGHT: u32 = 16;
const ICON_SRC_WIDTH: u32 = 32;
const ICON_SRC_HEIGHT: u32 = 32;

pub struct SaveButton {
    inner: WidgetInner,
}

impl std::ops::Deref for SaveButton {
    type Target = WidgetInner;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl std::ops::DerefMut for SaveButton {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl Widget for SaveButton {
    fn texture_path(&self) -> Option<String> {
        let c = self.config.read().unwrap();
        let mut themes_dir = c.directories().themes_dir.clone();
        let path = c.theme().images().save_icon();
        themes_dir.push(path);
        Some(themes_dir.to_str().unwrap().to_owned())
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

    fn on_left_click(&mut self, _point: &Point, _context: &UpdateContext) -> UR {
        UR::SaveCurrentFile
    }

    fn padding_width(&self) -> u32 {
        ICON_DEST_WIDTH
    }

    fn padding_height(&self) -> u32 {
        ICON_DEST_HEIGHT
    }
}

impl SaveButton {
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

#[cfg(test)]
mod test {
    use super::*;
    use crate::app::UpdateResult;
    use crate::tests::*;

    #[test]
    fn must_return_save_on_left_click() {
        let config = build_config();
        let mut widget = SaveButton::new(config);
        assert_eq!(
            widget.on_left_click(&Point::new(0, 0), &UpdateContext::Nothing),
            UpdateResult::SaveCurrentFile
        );
    }

    #[test]
    fn must_use_inner() {
        let config = build_config();
        let mut widget = SaveButton::new(config);

        assert_eq!(
            widget.dest(),
            &Rect::new(0, 0, ICON_DEST_WIDTH, ICON_DEST_HEIGHT)
        );
        widget.set_dest(&Rect::new(1, 2, 3, 4));
        assert_eq!(widget.dest(), &Rect::new(1, 2, 3, 4));

        assert_eq!(
            widget.source(),
            &Rect::new(0, 0, ICON_SRC_WIDTH, ICON_SRC_HEIGHT)
        );
        widget.set_source(&Rect::new(5, 6, 7, 8));
        assert_eq!(widget.source(), &Rect::new(5, 6, 7, 8));
    }

    #[test]
    fn must_have_padding() {
        let config = build_config();
        let widget = SaveButton::new(config);
        assert_eq!(widget.padding_width(), ICON_DEST_WIDTH);
        assert_eq!(widget.padding_height(), ICON_DEST_HEIGHT);
    }
}
