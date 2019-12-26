use sdl2::rect::{Point, Rect};
use std::sync::*;

use crate::app::UpdateResult as UR;
use crate::renderer::renderer::Renderer;
use crate::ui::file::editor_file_section::EditorFileSection;
use crate::ui::text_character::TextCharacter;
use crate::ui::*;
use rider_config::Config;
use rider_config::ConfigHolder;

#[derive(Clone, Debug)]
pub struct EditorFile {
    path: String,
    sections: Vec<EditorFileSection>,
    dest: Rect,
    buffer: String,
    config: Arc<RwLock<Config>>,
    line_height: u32,
}

impl EditorFile {
    pub fn new(path: String, buffer: String, config: Arc<RwLock<Config>>) -> Self {
        use std::path::Path;
        let ext = Path::new(&path)
            .extension()
            .and_then(|p| p.to_str())
            .map_or("txt", |s| s)
            .to_string();
        let sections = vec![EditorFileSection::new(
            buffer.clone(),
            ext,
            Arc::clone(&config),
        )];

        Self {
            path,
            sections,
            dest: Rect::new(0, 0, 0, 0),
            buffer,
            config,
            line_height: 0,
        }
    }

    pub fn buffer(&self) -> String {
        self.buffer.clone()
    }

    pub fn buffer_ref(&self) -> &String {
        &self.buffer
    }

    pub fn path(&self) -> String {
        self.path.clone()
    }

    pub fn sections(&self) -> &Vec<EditorFileSection> {
        &self.sections
    }

    pub fn line_height(&self) -> u32 {
        self.line_height
    }

    pub fn render_position(&self) -> &Rect {
        &self.dest
    }

    pub fn get_section_at_mut(&mut self, index: usize) -> Option<&mut EditorFileSection> {
        self.sections.get_mut(index)
    }

    fn refresh_characters_position(&mut self) {
        let mut current: Rect = Rect::new(0, 0, 0, 0);
        for section in self.sections.iter_mut() {
            section.update_positions(&mut current);
        }
    }

    pub fn iter_char(&self) -> EditorFileIterator {
        EditorFileIterator::new(self)
    }
}

impl TextCollection for EditorFile {
    fn get_character_at(&self, index: usize) -> Option<TextCharacter> {
        for section in self.sections.iter() {
            let character = section.get_character_at(index);
            if character.is_some() {
                return character;
            }
        }
        None
    }

    fn get_line(&self, line: &usize) -> Option<Vec<&TextCharacter>> {
        let mut vec: Vec<&TextCharacter> = vec![];
        for section in self.sections.iter() {
            match section.get_line(line) {
                Some(v) => vec.append(&mut v.clone()),
                _ => (),
            }
        }

        if vec.is_empty() {
            None
        } else {
            Some(vec)
        }
    }

    fn get_last_at_line(&self, line: usize) -> Option<TextCharacter> {
        let mut current: Option<TextCharacter> = None;
        'scanning: for section in self.sections.iter().rev() {
            for token in section.tokens().iter().rev() {
                for text_character in token.characters().iter().rev() {
                    match text_character.line() {
                        l if l > line => continue,
                        l if l < line => break 'scanning,
                        _ => (),
                    };
                    match (
                        current.clone().map(|ref c| c.is_new_line()),
                        text_character.is_new_line(),
                    ) {
                        (None, true) => current = Some(text_character.clone()),
                        (None, false) => current = Some(text_character.clone()),
                        (Some(true), false) => current = Some(text_character.clone()),
                        (Some(false), false) => break,
                        _ => continue,
                    };
                }
            }
        }
        match current {
            Some(ref tc) => {
                // Click on empty new line
                if tc.is_new_line() && self.get_character_at(tc.position() - 1).is_some() {
                    return self.get_character_at(tc.position() - 1);
                }
                return current;
            }
            None => None,
        }
    }
}

impl EditorFile {
    pub fn render<R, C>(&self, canvas: &mut C, renderer: &mut R, context: &RenderContext)
    where
        C: CanvasAccess,
        R: Renderer + CharacterSizeManager + ConfigHolder,
    {
        for section in self.sections.iter() {
            section.render(canvas, renderer, context);
        }
    }

    pub fn prepare_ui<R>(&mut self, renderer: &mut R)
    where
        R: ConfigHolder + CharacterSizeManager + Renderer,
    {
        for section in self.sections.iter_mut() {
            section.prepare_ui(renderer);
        }

        let r = renderer.load_character_size('W');
        self.line_height = r.height();
        self.refresh_characters_position();
    }
}

impl Update for EditorFile {
    fn update(&mut self, ticks: i32, context: &UpdateContext) -> UR {
        let mut result = UR::NoOp;
        for section in self.sections.iter_mut() {
            result = section.update(ticks, context);
        }
        result
    }
}

impl ClickHandler for EditorFile {
    fn on_left_click(&mut self, point: &Point, context: &UpdateContext) -> UR {
        let mut index = -1;
        for (i, section) in self.sections.iter().enumerate() {
            if section.is_left_click_target(point, context) {
                index = i as i32;
                break;
            }
        }
        if index >= 0 {
            return self
                .get_section_at_mut(index as usize)
                .unwrap()
                .on_left_click(point, &context);
        }
        UR::NoOp
    }

    fn is_left_click_target(&self, point: &Point, context: &UpdateContext) -> bool {
        for section in self.sections.iter() {
            if section.is_left_click_target(point, context) {
                return true;
            }
        }
        false
    }
}

impl TextWidget for EditorFile {
    fn full_rect(&self) -> Rect {
        let mut max_line_width = 0;
        let mut height = 0;
        for (index, section) in self.sections.iter().enumerate() {
            let r = section.full_rect();

            if index == 0 {
                height = r.height();
                max_line_width = r.width();
            } else {
                height += r.height();
                if max_line_width < r.width() {
                    max_line_width = r.width();
                }
            }
        }
        Rect::new(0, 0, max_line_width, height)
    }
}

impl RenderBox for EditorFile {
    fn render_start_point(&self) -> Point {
        self.dest.top_left()
    }

    fn dest(&self) -> Rect {
        self.dest.clone()
    }
}

pub struct EditorFileIterator<'a> {
    current_section: usize,
    current_token: usize,
    current_character: usize,
    file: &'a EditorFile,
}

impl<'a> EditorFileIterator<'a> {
    pub fn new(file: &'a EditorFile) -> Self {
        Self {
            file,
            current_section: 0,
            current_token: 0,
            current_character: 0,
        }
    }

    fn get_section(&self) -> Option<&'a EditorFileSection> {
        self.file.sections().get(self.current_section)
    }

    fn get_token(&mut self, section: &'a EditorFileSection) -> Option<&'a EditorFileToken> {
        section.tokens().get(self.current_token).or_else(|| {
            self.current_section += 1;
            self.current_token = 0;
            self.current_character = 0;
            self.get_token(self.get_section()?)
        })
    }

    fn get_character(&mut self, token: &'a EditorFileToken) -> Option<&'a TextCharacter> {
        token
            .characters()
            .get(self.current_character)
            .or_else(|| {
                self.current_character = 0;
                self.current_token += 1;
                let token = self.get_token(self.get_section()?)?;
                self.get_character(token)
            })
            .and_then(|c| {
                self.current_character += 1;
                Some(c)
            })
    }
}

impl<'a> Iterator for EditorFileIterator<'a> {
    type Item = &'a TextCharacter;

    fn next(&mut self) -> Option<Self::Item> {
        let token = self.get_token(self.get_section()?)?;
        self.get_character(token)
    }
}

#[cfg(test)]
mod tests {
    use crate::tests::support;
    use crate::tests::support::SimpleRendererMock;
    use crate::ui::*;
    use sdl2::rect::{Point, Rect};

    //##################################################
    // iterator
    //##################################################

    #[test]
    fn assert_simple_iterations() {
        let config = support::build_config();
        let mut renderer = SimpleRendererMock::new(config.clone());
        let mut file =
            EditorFile::new("./foo.txt".to_owned(), "a b c d".to_owned(), config.clone());
        file.prepare_ui(&mut renderer);
        for (index, c) in file.iter_char().enumerate() {
            match index {
                0 => assert_eq!(c.text_character(), 'a'),
                1 => assert_eq!(c.text_character(), ' '),
                2 => assert_eq!(c.text_character(), 'b'),
                3 => assert_eq!(c.text_character(), ' '),
                4 => assert_eq!(c.text_character(), 'c'),
                5 => assert_eq!(c.text_character(), ' '),
                6 => assert_eq!(c.text_character(), 'd'),
                _ => assert_eq!("must have 7 entries", "have more than 7 entries"),
            }
        }
    }

    //##################################################
    // path
    //##################################################

    #[test]
    fn assert_path_txt() {
        let config = support::build_config();
        let buffer = "".to_owned();
        let path = "/example.txt".to_owned();
        let widget = EditorFile::new(path, buffer, config);
        assert_eq!(widget.path(), "/example.txt".to_owned());
    }

    #[test]
    fn assert_path_rs() {
        let config = support::build_config();
        let buffer = "".to_owned();
        let path = "/example.rs".to_owned();
        let widget = EditorFile::new(path, buffer, config);
        assert_eq!(widget.path(), "/example.rs".to_owned());
    }

    //##################################################
    // buffer
    //##################################################

    #[test]
    fn assert_empty_buffer() {
        let config = support::build_config();
        let buffer = "".to_owned();
        let path = "/example.txt".to_owned();
        let widget = EditorFile::new(path, buffer, config);
        assert_eq!(widget.buffer(), "".to_owned());
    }

    #[test]
    fn assert_some_buffer() {
        let config = support::build_config();
        let buffer = "fn main(){}".to_owned();
        let path = "some.rs".to_owned();
        let widget = EditorFile::new(path, buffer, config);
        assert_eq!(widget.buffer(), "fn main(){}".to_owned());
    }

    //##################################################
    // line height
    //##################################################

    #[test]
    fn assert_initial_line_height() {
        let config = support::build_config();
        let buffer = "".to_owned();
        let path = "/example.txt".to_owned();
        let widget = EditorFile::new(path, buffer, config);
        assert_eq!(widget.line_height(), 0);
    }

    //##################################################
    // render box
    //##################################################

    #[test]
    fn assert_dest() {
        let config = support::build_config();
        let buffer = "".to_owned();
        let path = "/example.txt".to_owned();
        let widget = EditorFile::new(path, buffer, config);
        let result = widget.dest().clone();
        let expected = Rect::new(0, 0, 1, 1);
        assert_eq!(result, expected);
    }

    #[test]
    fn assert_render_start_point() {
        let config = support::build_config();
        let buffer = "".to_owned();
        let path = "/example.txt".to_owned();
        let widget = EditorFile::new(path, buffer, config);
        let result = widget.render_start_point().clone();
        let expected = Point::new(0, 0);
        assert_eq!(result, expected);
    }
}
