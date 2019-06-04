pub mod open_file;
pub mod settings;

pub use self::open_file::*;
pub use self::settings::*;

pub enum ModalType {
    OpenFile(OpenFile),
    Settings(Settings),
}

impl PartialEq for ModalType {
    fn eq(&self, other: &ModalType) -> bool {
        match (self, other) {
            (ModalType::OpenFile { .. }, ModalType::OpenFile { .. }) => true,
            (ModalType::Settings { .. }, ModalType::Settings { .. }) => true,
            _ => false,
        }
    }
}

impl std::fmt::Debug for ModalType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        let name = match self {
            ModalType::OpenFile(_) => "OpenFile",
            ModalType::Settings(_) => "Settings",
        };
        write!(f, "<Modal::{:?} {{}}", name)
    }
}
