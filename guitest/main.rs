#[cfg(test)]
mod file_editor {
    use sdl2::pixels::*;
    use sdl2::rect::*;
    use std::sync::*;

    #[test]
    fn add_text() {
        use crate::tests::support;
        let config = support::build_config();
        let canvas = support::build_canvas();
        let font_context = sdl2::ttf::init().unwrap();
        let texture_creator = canvas.texture_creator();
        let mut renderer = Renderer::new(Arc::clone(&config), &font_context, &texture_creator);

        let mut editor = FileEditor::new(Arc::clone(&config));
        let mut file = EditorFile::new("./foo.txt".to_string(), "foo".to_string(), config.clone());
        file.prepare_ui(&mut renderer);
        assert_eq!(editor.open_file(file).is_none(), true);
        assert_eq!(editor.caret().position().text_position(), 0);
        assert_eq!(editor.file().is_some(), true);
        assert_eq!(editor.file().unwrap().sections().len(), 1);
        assert_eq!(editor.file().unwrap().get_character_at(0).is_some(), true);

        editor.insert_text("z".to_string(), &mut renderer);
        assert_eq!(editor.caret().position().text_position(), 1);
        assert_eq!(editor.file().is_some(), true);
        assert_eq!(editor.file().unwrap().buffer(), "zfoo".to_string());
    }
}

#[cfg(test)]
mod text_character {
    use sdl2::pixels::*;
    use sdl2::rect::*;
    use std::sync::*;

    #[test]
    fn must_return_valid_source() {
        let config = support::build_config();
        let canvas = support::build_canvas();
        let font_context = sdl2::ttf::init().unwrap();
        let texture_creator = canvas.texture_creator();
        let mut renderer = Renderer::new(Arc::clone(&config), &font_context, &texture_creator);

        let mut widget =
            TextCharacter::new('\n', 0, 0, true, Color::RGB(1, 12, 123), Arc::clone(&config));
        widget.prepare_ui(&mut renderer);
        assert_eq!(widget.source(), &Rect::new(0, 0, 0, 0));
    }

    #[test]
    fn must_return_valid_dest() {
        let config = support::build_config();
        let canvas = support::build_canvas();
        let font_context = sdl2::ttf::init().unwrap();
        let texture_creator = canvas.texture_creator();
        let mut renderer = Renderer::new(Arc::clone(&config), &font_context, &texture_creator);

        let mut widget =
            TextCharacter::new('\n', 0, 0, true, Color::RGB(1, 12, 123), Arc::clone(&config));
        widget.prepare_ui(&mut renderer);
        assert_eq!(widget.dest(), &Rect::new(0, 0, 0, 0));
    }

    #[test]
    fn must_return_valid_color() {
        let config = support::build_config();
        let canvas = support::build_canvas();
        let font_context = sdl2::ttf::init().unwrap();
        let texture_creator = canvas.texture_creator();
        let mut renderer = Renderer::new(Arc::clone(&config), &font_context, &texture_creator);

        let mut widget =
            TextCharacter::new('\n', 0, 0, true, Color::RGB(1, 12, 123), Arc::clone(&config));
        widget.prepare_ui(&mut renderer);
        assert_eq!(widget.color(), &Color::RGB(1, 12, 123));
    }

    #[test]
    fn must_update_position_of_new_line() {
        let config = support::build_config();

        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();


        let window = video_subsystem
            .window("Test", 1, 1)
            .borderless()
            .opengl()
            .build()
            .unwrap();

        let canvas = window.into_canvas().accelerated().build().unwrap();

        let font_context = sdl2::ttf::init().unwrap();
        let texture_creator = canvas.texture_creator();
        let mut renderer = Renderer::new(Arc::clone(&config), &font_context, &texture_creator);

        let mut widget =
            TextCharacter::new('\n', 0, 0, true, Color::RGB(0, 0, 0), Arc::clone(&config));
        widget.prepare_ui(&mut renderer);
        let mut current = Rect::new(0, 0, 0, 0);
        widget.update_position(&mut current);
        assert_eq!(current, Rect::new(0, 0, 0, 0));
        assert_eq!(widget.dest(), &Rect::new(0, 0, 0, 0));
    }
}



#[cfg(test)]
fn main() {
        let config = support::build_config();

        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();


        let window = video_subsystem
            .window("Test", 1, 1)
            .borderless()
            .opengl()
            .build()
            .unwrap();

        let canvas = window.into_canvas().accelerated().build().unwrap();

}
