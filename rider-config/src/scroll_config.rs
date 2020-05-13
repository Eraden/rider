#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScrollConfig {
    width: u32,
    margin_right: i32,
    speed: i32,
}

impl ScrollConfig {
    pub fn new() -> Self {
        Self {
            width: 4,
            margin_right: 5,
            speed: 30,
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn set_width(&mut self, width: u32) {
        self.width = width;
    }

    pub fn margin_right(&self) -> i32 {
        self.margin_right
    }

    pub fn set_margin_right(&mut self, margin_right: i32) {
        self.margin_right = margin_right;
    }

    pub fn speed(&self) -> i32 {
        self.speed
    }

    pub fn set_speed(&mut self, speed: i32) {
        self.speed = speed
    }
}

impl Default for ScrollConfig {
    fn default() -> Self {
        Self {
            width: 4,
            margin_right: 5,
            speed: 10,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn assert_default() {
        let config = ScrollConfig::default();
        assert_eq!(config.width, 4);
        assert_eq!(config.margin_right, 5);
        assert_eq!(config.speed, 10);
    }

    #[test]
    fn assert_width() {
        let config = ScrollConfig::new();
        let result = config.width();
        let expected = 4;
        assert_eq!(result, expected);
    }

    #[test]
    fn assert_set_width() {
        let mut config = ScrollConfig::new();
        config.set_width(60);
        let result = config.width();
        let expected = 60;
        assert_eq!(result, expected);
    }

    #[test]
    fn assert_margin_right() {
        let config = ScrollConfig::new();
        let result = config.margin_right();
        let expected = 5;
        assert_eq!(result, expected);
    }

    #[test]
    fn assert_set_margin_right() {
        let mut config = ScrollConfig::new();
        config.set_margin_right(98);
        let result = config.margin_right();
        let expected = 98;
        assert_eq!(result, expected);
    }

    #[test]
    fn assert_speed() {
        let config = ScrollConfig::new();
        let result = config.speed();
        let expected = 30;
        assert_eq!(result, expected);
    }

    #[test]
    fn assert_set_speed() {
        let mut config = ScrollConfig::new();
        config.set_speed(98);
        let result = config.speed();
        let expected = 98;
        assert_eq!(result, expected);
    }
}
