use crate::themes::SerdeColor;
use crate::themes::ThemeConfig;

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct CodeHighlightingColor {
    pub comment: ThemeConfig,
    pub constant: ThemeConfig,
    pub error: ThemeConfig,
    pub warning: ThemeConfig,
    pub identifier: ThemeConfig,
    pub keyword: ThemeConfig,
    pub literal: ThemeConfig,
    pub number: ThemeConfig,
    pub operator: ThemeConfig,
    pub separator: ThemeConfig,
    pub statement: ThemeConfig,
    pub string: ThemeConfig,
    pub title: ThemeConfig,
    pub type_: ThemeConfig,
    pub todo: ThemeConfig,
    pub pre_proc: ThemeConfig,
    pub special: ThemeConfig,
    pub whitespace: ThemeConfig,
}

impl Default for CodeHighlightingColor {
    fn default() -> Self {
        Self {
            comment: ThemeConfig::new(SerdeColor::new(0, 0, 0, 0), false, false),
            constant: ThemeConfig::new(SerdeColor::new(0, 0, 0, 0), false, false),
            error: ThemeConfig::new(SerdeColor::new(0, 0, 0, 0), false, false),
            warning: ThemeConfig::new(SerdeColor::new(0, 0, 0, 0), false, false),
            identifier: ThemeConfig::new(SerdeColor::new(0, 0, 0, 0), false, false),
            keyword: ThemeConfig::new(SerdeColor::new(0, 0, 0, 0), false, false),
            literal: ThemeConfig::new(SerdeColor::new(0, 0, 0, 0), false, false),
            number: ThemeConfig::new(SerdeColor::new(0, 0, 0, 0), false, false),
            operator: ThemeConfig::new(SerdeColor::new(0, 0, 0, 0), false, false),
            separator: ThemeConfig::new(SerdeColor::new(0, 0, 0, 0), false, false),
            statement: ThemeConfig::new(SerdeColor::new(0, 0, 0, 0), false, false),
            string: ThemeConfig::new(SerdeColor::new(0, 0, 0, 0), false, false),
            title: ThemeConfig::new(SerdeColor::new(0, 0, 0, 0), false, false),
            type_: ThemeConfig::new(SerdeColor::new(0, 0, 0, 0), false, false),
            todo: ThemeConfig::new(SerdeColor::new(0, 0, 0, 0), false, false),
            pre_proc: ThemeConfig::new(SerdeColor::new(0, 0, 0, 0), false, false),
            special: ThemeConfig::new(SerdeColor::new(0, 0, 0, 0), false, false),
            whitespace: ThemeConfig::new(SerdeColor::new(0, 0, 0, 0), false, false),
        }
    }
}

impl CodeHighlightingColor {
    pub fn comment(&self) -> &ThemeConfig {
        &self.comment
    }

    pub fn constant(&self) -> &ThemeConfig {
        &self.constant
    }

    pub fn error(&self) -> &ThemeConfig {
        &self.error
    }

    pub fn warning(&self) -> &ThemeConfig {
        &self.warning
    }

    pub fn identifier(&self) -> &ThemeConfig {
        &self.identifier
    }

    pub fn keyword(&self) -> &ThemeConfig {
        &self.keyword
    }

    pub fn literal(&self) -> &ThemeConfig {
        &self.literal
    }

    pub fn number(&self) -> &ThemeConfig {
        &self.number
    }

    pub fn operator(&self) -> &ThemeConfig {
        &self.operator
    }

    pub fn separator(&self) -> &ThemeConfig {
        &self.separator
    }

    pub fn statement(&self) -> &ThemeConfig {
        &self.statement
    }

    pub fn string(&self) -> &ThemeConfig {
        &self.string
    }

    pub fn title(&self) -> &ThemeConfig {
        &self.title
    }

    pub fn type_(&self) -> &ThemeConfig {
        &self.type_
    }

    pub fn todo(&self) -> &ThemeConfig {
        &self.todo
    }

    pub fn pre_proc(&self) -> &ThemeConfig {
        &self.pre_proc
    }

    pub fn special(&self) -> &ThemeConfig {
        &self.special
    }

    pub fn whitespace(&self) -> &ThemeConfig {
        &self.whitespace
    }
}
