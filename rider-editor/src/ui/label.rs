use crate::app::*;
use crate::renderer::*;
use crate::ui::*;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use std::collections::HashMap;

const DEST_WIDTH: u32 = 16;
const DEST_HEIGHT: u32 = 16;
const SRC_WIDTH: u32 = 64;
const SRC_HEIGHT: u32 = 64;

pub struct Label {
    name: String,
    char_sizes: HashMap<char, Rect>,
    inner: WidgetInner,
}

impl std::ops::Deref for Label {
    type Target = WidgetInner;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl std::ops::DerefMut for Label {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl Widget for Label {
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

    fn render<C, R>(&self, canvas: &mut C, renderer: &mut R, context: &RenderContext)
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
        canvas.set_clipping(d.clone());

        let font_details = build_font_details(self);
        for c in self.name.chars() {
            let size = self
                .char_sizes
                .get(&c)
                .cloned()
                .unwrap_or_else(|| Rect::new(0, 0, 0, 0));
            renderer
                .load_text_tex(
                    &mut TextDetails {
                        color: Color::RGBA(255, 255, 255, 0),
                        text: c.to_string(),
                        font: font_details.clone(),
                    },
                    font_details.clone(),
                )
                .and_then(|texture| {
                    d.set_width(size.width());
                    d.set_height(size.height());

                    canvas
                        .render_image(texture, self.source.clone(), d.clone())
                        .unwrap_or_else(|_| panic!("Failed to draw directory entry texture"));
                    d.set_x(d.x() + size.width() as i32);
                    Ok(())
                })
                .unwrap_or_else(|e| {
                    eprintln!("Failed to render label \"{:?}\": {:?}", self.name(), e)
                })
        }
    }

    fn prepare_ui<R>(&mut self, renderer: &mut R)
    where
        R: Renderer + CharacterSizeManager,
    {
        let w_rect = renderer.load_character_size('W');
        self.char_sizes.insert('W', w_rect.clone());
        let mut name_width = 0;

        for c in self.name().chars() {
            let size = { renderer.load_character_size(c.clone()) };
            self.char_sizes.insert(c, size);
            name_width += size.width();
        }
        self.dest.set_width(name_width);
        self.dest.set_height(w_rect.height());
    }
}

impl Label {
    pub fn new(name: String, config: ConfigAccess) -> Self {
        Self {
            name,
            char_sizes: HashMap::new(),
            inner: WidgetInner::new(
                config,
                Rect::new(0, 0, SRC_WIDTH, SRC_HEIGHT),
                Rect::new(0, 0, DEST_WIDTH, DEST_HEIGHT),
            ),
        }
    }

    #[inline]
    pub fn name_width(&self) -> u32 {
        self.dest.width()
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }
}

impl ConfigHolder for Label {
    fn config(&self) -> &ConfigAccess {
        &self.config
    }
}
