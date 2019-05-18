#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CaretPosition {
    text_position: usize,
    line_number: usize,
    line_position: usize,
}

impl CaretPosition {
    pub fn new(text_position: usize, line_number: usize, line_position: usize) -> Self {
        Self {
            text_position,
            line_number,
            line_position,
        }
    }

    #[inline]
    pub fn text_position(&self) -> usize {
        self.text_position
    }

    #[inline]
    pub fn line_number(&self) -> usize {
        self.line_number
    }

    #[inline]
    pub fn line_position(&self) -> usize {
        self.line_position
    }

    #[inline]
    pub fn reset(&mut self) {
        self.text_position = 0;
        self.line_number = 0;
        self.line_position = 0;
    }

    #[inline]
    pub fn set_text_position(&mut self, n: usize) {
        self.text_position = n;
    }

    #[inline]
    pub fn set_line_number(&mut self, n: usize) {
        self.line_number = n;
    }

    #[inline]
    pub fn set_line_position(&mut self, n: usize) {
        self.line_position = n;
    }

    pub fn moved(&self, text_position: i32, line_number: i32, line_position: i32) -> Self {
        Self {
            text_position: (self.text_position as i32 + text_position) as usize,
            line_number: (self.line_number as i32 + line_number) as usize,
            line_position: (self.line_position as i32 + line_position) as usize,
        }
    }

    #[inline]
    pub fn is_first(&self) -> bool {
        self.line_number == 0 && self.text_position == 0
    }
}

#[cfg(test)]
mod test_getters {
    use super::*;

    #[test]
    fn assert_text_position() {
        let target = CaretPosition::new(1, 2, 3);
        let result = target.text_position();
        let expected = 1;
        assert_eq!(result, expected);
    }

    #[test]
    fn assert_line_number() {
        let target = CaretPosition::new(1, 2, 3);
        let result = target.line_number();
        let expected = 2;
        assert_eq!(result, expected);
    }

    #[test]
    fn assert_line_position() {
        let target = CaretPosition::new(1, 2, 3);
        let result = target.line_position();
        let expected = 3;
        assert_eq!(result, expected);
    }

    #[test]
    fn assert_moved() {
        let target = CaretPosition::new(1, 2, 3);
        let result = target.moved(1, 2, 3);
        let expected = CaretPosition::new(2, 4, 6);
        assert_eq!(result, expected);
    }
}

#[cfg(test)]
mod test_setters {
    use super::*;

    #[test]
    fn assert_text_position() {
        let mut target = CaretPosition::new(1, 2, 3);
        target.set_text_position(100);
        let result = target.text_position();
        let expected = 100;
        assert_eq!(result, expected);
    }

    #[test]
    fn assert_line_number() {
        let mut target = CaretPosition::new(1, 2, 3);
        target.set_line_number(200);
        let result = target.line_number();
        let expected = 200;
        assert_eq!(result, expected);
    }

    #[test]
    fn assert_line_position() {
        let mut target = CaretPosition::new(1, 2, 3);
        target.set_line_position(300);
        let result = target.line_position();
        let expected = 300;
        assert_eq!(result, expected);
    }
}
