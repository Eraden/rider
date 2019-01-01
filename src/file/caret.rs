pub struct Caret<'a> {
    character: char,
    source: Rect,
    dest: Rect,
    visible: bool,
    texture: Option<Rc<Texture<'a>>>,
}

impl<'a> Caret<'a> {
    pub fn new() -> Self {
        Self {
            character: '│',
            source: Rect::new(0, 0, 0, 0),
            dest: Rect::new(0, 0, 0, 0),
            visible: true,
            texture: None,
        }
    }
}
