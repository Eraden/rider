use crate::caret_color::CaretColor;
use crate::CodeHighlightingColor;
use crate::DiffColor;
use crate::SerdeColor;
use crate::Theme;
use crate::ThemeConfig;
use crate::ThemeImages;

pub fn build_theme() -> Theme {
    Theme::new(
        "railscasts".to_string(),
        SerdeColor::new(18, 18, 18, 0),
        SerdeColor::new(200, 200, 200, 0),
        CaretColor::new(
            ThemeConfig::new(SerdeColor::new(121, 121, 121, 0), false, false),
            ThemeConfig::new(SerdeColor::new(21, 21, 21, 0), false, false),
        ),
        CodeHighlightingColor {
            comment: ThemeConfig::new(SerdeColor::new(175, 135, 95, 0), false, false),
            constant: ThemeConfig::new(SerdeColor::new(109, 156, 190, 0), false, false),
            error: ThemeConfig::new(SerdeColor::new(255, 255, 255, 0), false, false),
            warning: ThemeConfig::new(SerdeColor::new(128, 0, 0, 0), false, false),
            identifier: ThemeConfig::new(SerdeColor::new(175, 95, 95, 0), false, false),
            keyword: ThemeConfig::new(SerdeColor::new(175, 95, 0, 0), false, false),
            literal: ThemeConfig::new(SerdeColor::new(228, 228, 228, 0), false, false),
            number: ThemeConfig::new(SerdeColor::new(135, 175, 95, 0), false, false),
            operator: ThemeConfig::new(SerdeColor::new(228, 228, 228, 0), false, false),
            separator: ThemeConfig::new(SerdeColor::new(228, 228, 228, 0), false, false),
            statement: ThemeConfig::new(SerdeColor::new(175, 95, 0, 0), false, false),
            string: ThemeConfig::new(SerdeColor::new(135, 175, 95, 0), false, false),
            title: ThemeConfig::new(SerdeColor::new(255, 255, 255, 0), false, false),
            type_: ThemeConfig::new(SerdeColor::new(223, 95, 95, 0), false, false),
            todo: ThemeConfig::new(SerdeColor::new(223, 95, 95, 0), false, false),
            pre_proc: ThemeConfig::new(SerdeColor::new(255, 135, 0, 0), false, false),
            special: ThemeConfig::new(SerdeColor::new(0, 95, 0, 0), false, false),
            whitespace: ThemeConfig::new(SerdeColor::new(220, 220, 220, 90), false, false),
        },
        DiffColor::new(
            ThemeConfig::new(SerdeColor::new(228, 228, 228, 0), false, false),
            ThemeConfig::new(SerdeColor::new(102, 0, 0, 0), false, false),
            ThemeConfig::new(SerdeColor::new(135, 0, 135, 0), false, false),
            ThemeConfig::new(SerdeColor::new(18, 18, 18, 0), false, false),
        ),
        ThemeImages::new(
            "railscasts/images/directory-64x64.png".to_owned(),
            "railscasts/images/file-64x64.png".to_owned(),
            "railscasts/images/save-64x64.png".to_owned(),
        ),
    )
}
