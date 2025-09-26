use std::fmt::{self, Debug, Display};

use super::Effect;

pub struct Styled<'a, T> {
	pub(crate) value: &'a T,
	pub(crate) effects: Vec<Effect>,
	pub(crate) reset: bool,
}

impl<'a, T> Styled<'a, T> {
	pub fn new<const N: usize>(value: &'a T, effects: [Effect; N], reset: bool) -> Self {
		Self {
			value,
			effects: effects.to_vec(),
			reset,
		}
	}
}

impl<T: Display> Display for Styled<'_, T> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		for e in &self.effects {
			f.write_str(&e.to_ansi())?;
		}
		write!(f, "{}", self.value)?;
		if self.reset {
			f.write_str("\x1b[m")?;
		}
		Ok(())
	}
}

impl<T: Debug> Debug for Styled<'_, T> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		for e in &self.effects {
			f.write_str(&e.to_ansi())?;
		}
		write!(f, "{:?}", self.value)?;
		if self.reset {
			f.write_str("\x1b[m")?;
		}
		Ok(())
	}
}
