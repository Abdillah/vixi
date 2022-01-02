mod termion;

pub use self::termion::TermionKeyboard;

pub trait Keyboard {
    fn get_next_keystroke(&mut self) -> Option<KeyStroke>;
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum KeyStroke {
    Char(char),
    KeyF(u8),
    Alt(char),
    KeyUp,
    KeyDown,
    KeyLeft,
    KeyRight,
    KeyPreviousPage,
    KeyNextPage,
    KeyEscape,
    KeyBackSpace,
    KeyDelete,
    KeySpace,
}

impl KeyStroke {
    pub fn from_description(description: &str) -> Option<Self> {
        if description.len() == 1 {
            return Some(KeyStroke::Char(description.chars().nth(0).unwrap()));
        }

        match description.to_lowercase().as_str() {
            "<f1>" => Some(KeyStroke::KeyF(1)),
            "<up>" => Some(KeyStroke::KeyUp),
            "<down>" => Some(KeyStroke::KeyDown),
            "<left>" => Some(KeyStroke::KeyLeft),
            "<right>" => Some(KeyStroke::KeyRight),
            "<pageup>" => Some(KeyStroke::KeyPreviousPage),
            "<pagedown>" => Some(KeyStroke::KeyNextPage),
            "<backspace>" => Some(KeyStroke::KeyBackSpace),
            "<del>" => Some(KeyStroke::KeyDelete),
            "<space>" => Some(KeyStroke::KeySpace),
            "<esc>" => Some(KeyStroke::KeyEscape),
            _ => None,
        }
    }
}
