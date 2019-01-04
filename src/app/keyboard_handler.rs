use sdl2::keyboard::{Keycode, Scancode};
use sdl2::event::EventPollIterator;
use sdl2::EventPump;

use crate::app::UpdateResult;

pub fn resolve_action(key_code: Keycode, event_pump: &mut EventPump) -> UpdateResult {
    match key_code {
        Keycode::Backspace => return UpdateResult::DeleteFront,
        Keycode::Delete => return UpdateResult::DeleteBack,

        Keycode::Escape => return UpdateResult::NoOp,

        Keycode::Space => {
            let character: char = ' ';
            return UpdateResult::Input(character);
        }
        Keycode::Exclaim => {
            let character: char = '!';
            return UpdateResult::Input(character);
        }
        Keycode::Quotedbl => {
            let character: char = '"';
            return UpdateResult::Input(character);
        }
        Keycode::Hash => {
            let character: char = '#';
            return UpdateResult::Input(character);
        }
        Keycode::Dollar => {
            let character: char = '$';
            return UpdateResult::Input(character);
        }
        Keycode::Percent => {
            let character: char = '%';
            return UpdateResult::Input(character);
        }
        Keycode::Ampersand => {
            let character: char = '&';
            return UpdateResult::Input(character);
        }
        Keycode::Quote => {
            let character: char = '\'';
            return UpdateResult::Input(character);
        }
        Keycode::LeftParen => {
            let character: char = '{';
            return UpdateResult::Input(character);
        }
        Keycode::RightParen => {
            let character: char = '}';
            return UpdateResult::Input(character);
        }
        Keycode::Asterisk => {
            let character: char = '*';
            return UpdateResult::Input(character);
        }
        Keycode::Plus => {
            let character: char = '+';
            return UpdateResult::Input(character);
        }
        Keycode::Comma => {
            let character: char = ',';
            return UpdateResult::Input(character);
        }
        Keycode::Minus => {
            let character: char = '-';
            return UpdateResult::Input(character);
        }
        Keycode::Period => {
            let character: char = '.';
            return UpdateResult::Input(character);
        }
        Keycode::Slash => {
            let character: char = '/';
            return UpdateResult::Input(character);
        }
        Keycode::Num0 => {
            let character: char = '0';
            return UpdateResult::Input(character);
        }
        Keycode::Num1 => {
            let character: char = '1';
            return UpdateResult::Input(character);
        }
        Keycode::Num2 => {
            let character: char = '2';
            return UpdateResult::Input(character);
        }
        Keycode::Num3 => {
            let character: char = '3';
            return UpdateResult::Input(character);
        }
        Keycode::Num4 => {
            let character: char = '4';
            return UpdateResult::Input(character);
        }
        Keycode::Num5 => {
            let character: char = '5';
            return UpdateResult::Input(character);
        }
        Keycode::Num6 => {
            let character: char = '6';
            return UpdateResult::Input(character);
        }
        Keycode::Num7 => {
            let character: char = '7';
            return UpdateResult::Input(character);
        }
        Keycode::Num8 => {
            let character: char = '8';
            return UpdateResult::Input(character);
        }
        Keycode::Num9 => {
            let character: char = '9';
            return UpdateResult::Input(character);
        }
        Keycode::Colon => {
            let character: char = ':';
            return UpdateResult::Input(character);
        }
        Keycode::Semicolon => {
            let character: char = ';';
            return UpdateResult::Input(character);
        }
        Keycode::Less => {
            let character: char = '<';
            return UpdateResult::Input(character);
        }
        Keycode::Equals => {
            let character: char = '=';
            return UpdateResult::Input(character);
        }
        Keycode::Greater => {
            let character: char = '>';
            return UpdateResult::Input(character);
        }
        Keycode::Question => {
            let character: char = '?';
            return UpdateResult::Input(character);
        }
        Keycode::At => {
            let character: char = '@';
            return UpdateResult::Input(character);
        }
        Keycode::LeftBracket => {
            let character: char = '[';
            return UpdateResult::Input(character);
        }
        Keycode::Backslash => {
            let character: char = '\\';
            return UpdateResult::Input(character);
        }
        Keycode::RightBracket => {
            let character: char = ']';
            return UpdateResult::Input(character);
        }
        Keycode::Caret => {
            let character: char = '^';
            return UpdateResult::Input(character);
        }
        Keycode::Underscore => {
            let character: char = '_';
            return UpdateResult::Input(character);
        }
        Keycode::Backquote => {
            let character: char = '`';
            return UpdateResult::Input(character);
        }
        Keycode::A => {
            let character: char = if with_shift(event_pump) { 'A' } else { 'a' };
            return UpdateResult::Input(character);
        }
        Keycode::B => {
            let character: char = if with_shift(event_pump) { 'B' } else { 'b' };
            return UpdateResult::Input(character);
        }
        Keycode::C => {
            let character: char = if with_shift(event_pump) { 'C' } else { 'c' };
            return UpdateResult::Input(character);
        }
        Keycode::D => {
            let character: char = if with_shift(event_pump) { 'D' } else { 'd' };
            return UpdateResult::Input(character);
        }
        Keycode::E => {
            let character: char = if with_shift(event_pump) { 'E' } else { 'e' };
            return UpdateResult::Input(character);
        }
        Keycode::F => {
            let character: char = if with_shift(event_pump) { 'F' } else { 'f' };
            return UpdateResult::Input(character);
        }
        Keycode::G => {
            let character: char = if with_shift(event_pump) { 'G' } else { 'g' };
            return UpdateResult::Input(character);
        }
        Keycode::H => {
            let character: char = if with_shift(event_pump) { 'H' } else { 'h' };
            return UpdateResult::Input(character);
        }
        Keycode::I => {
            let character: char = if with_shift(event_pump) { 'I' } else { 'i' };
            return UpdateResult::Input(character);
        }
        Keycode::J => {
            let character: char = if with_shift(event_pump) { 'J' } else { 'j' };
            return UpdateResult::Input(character);
        }
        Keycode::K => {
            let character: char = if with_shift(event_pump) { 'K' } else { 'k' };
            return UpdateResult::Input(character);
        }
        Keycode::L => {
            let character: char = if with_shift(event_pump) { 'L' } else { 'l' };
            return UpdateResult::Input(character);
        }
        Keycode::M => {
            let character: char = if with_shift(event_pump) { 'M' } else { 'm' };
            return UpdateResult::Input(character);
        }
        Keycode::N => {
            let character: char = if with_shift(event_pump) { 'N' } else { 'n' };
            return UpdateResult::Input(character);
        }
        Keycode::O => {
            let character: char = if with_shift(event_pump) { 'O' } else { 'o' };
            return UpdateResult::Input(character);
        }
        Keycode::P => {
            let character: char = if with_shift(event_pump) { 'P' } else { 'p' };
            return UpdateResult::Input(character);
        }
        Keycode::Q => {
            let character: char = if with_shift(event_pump) { 'Q' } else { 'q' };
            return UpdateResult::Input(character);
        }
        Keycode::R => {
            let character: char = if with_shift(event_pump) { 'R' } else { 'r' };
            return UpdateResult::Input(character);
        }
        Keycode::S => {
            let character: char = if with_shift(event_pump) { 'S' } else { 's' };
            return UpdateResult::Input(character);
        }
        Keycode::T => {
            let character: char = if with_shift(event_pump) { 'T' } else { 't' };
            return UpdateResult::Input(character);
        }
        Keycode::U => {
            let character: char = if with_shift(event_pump) { 'U' } else { 'u' };
            return UpdateResult::Input(character);
        }
        Keycode::V => {
            let character: char = if with_shift(event_pump) { 'V' } else { 'v' };
            return UpdateResult::Input(character);
        }
        Keycode::W => {
            let character: char = if with_shift(event_pump) { 'W' } else { 'w' };
            return UpdateResult::Input(character);
        }
        Keycode::X => {
            let character: char = if with_shift(event_pump) { 'X' } else { 'x' };
            return UpdateResult::Input(character);
        }
        Keycode::Y => {
            let character: char = if with_shift(event_pump) { 'Y' } else { 'y' };
            return UpdateResult::Input(character);
        }
        Keycode::Z => {
            let character: char = if with_shift(event_pump) { 'Z' } else { 'z' };
            return UpdateResult::Input(character);
        }
        _ => UpdateResult::NoOp
    }
}

fn with_shift(event_pump: &mut EventPump) -> bool {
    event_pump.keyboard_state().is_scancode_pressed(Scancode::LShift) ||
    event_pump.keyboard_state().is_scancode_pressed(Scancode::RShift)
}
