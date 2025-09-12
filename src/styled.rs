use std::fmt::{self, Debug, Display};

use super::Effect;

pub struct Styled<'a, T> {
    pub(crate) value: &'a T,
    pub(crate) effects: Vec<Effect>,
}

impl<T: Display> Display for Styled<'_, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for e in &self.effects {
            write!(f, "{}", e.to_ansi())?;
        }
        write!(f, "{}\x1b[m", self.value)
    }
}

impl<T: Debug> Debug for Styled<'_, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for e in &self.effects {
            write!(f, "{}", e.to_ansi())?;
        }
        write!(f, "{:?}\x1b[m", self.value)
    }
}
