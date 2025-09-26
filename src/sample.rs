// とりあえず仮でパフォーマンス改良版を作ろうとしているが、いろいろ難しい
// Styledおよび使える型に完全に自力で実装しようと考えたが、&strに実装しようとするとライフタイムがごちゃごちゃになって苦しい
// 各種装飾メソッドトレイトをひとつに纏めて

use std::fmt::{self, Debug, Display};

#[derive(Clone)]
pub enum Effect {
	Style(Ansi),
	Color(u8),
	TrueColor(u8, u8, u8),
	BgColor(u8),
	BgTrueColor(u8, u8, u8),
	ClearLine,
	MoveCursorToStart,
}
#[derive(Clone, Copy)]
#[repr(u8)]
#[allow(non_camel_case_types)]
pub enum Ansi {
	bold = 1,
	faint = 2,
	italic = 3,
	underline = 4,
	blink = 5,
	rapid_blink = 6,
	reverse = 7,
	conceal = 8,
	strike = 9,
}
impl Effect {
	fn to_ansi(&self) -> String {
		match self {
			Effect::Style(s) => format!("\x1b[{}m", *s as u8),
			Effect::Color(code) => format!("\x1b[38;5;{code}m"),
			Effect::TrueColor(r, g, b) => format!("\x1b[38;2;{r};{g};{b}m"),
			Effect::BgColor(code) => format!("\x1b[48;5;{code}m"),
			Effect::BgTrueColor(r, g, b) => format!("\x1b[48;2;{r};{g};{b}m"),
			Effect::ClearLine => "\x1b[2K".into(),
			Effect::MoveCursorToStart => "\x1b[1G".into(),
		}
	}
}

pub struct Styled<'a, T> {
	value: &'a T,
	effects: Vec<Effect>,
	reset: bool,
}
impl<'a, T> Styled<'a, T> {
	pub fn new(value: &'a T, effects: Vec<Effect>, reset: bool) -> Self {
		Self { value, effects, reset }
	}
	fn and(&mut self, mut effects: Vec<Effect>, reset: bool) -> &mut Self {
		self.effects.append(&mut effects);
		self.reset |= reset;
		self
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

impl<T> Styled<'_, T> {}

pub trait Stylize<T>: Sized {
	fn bold(&self) -> Styled<'_, T>;
	fn override_line(&self) -> Styled<'_, T>;
}
impl<T> Stylize<T> for Styled<'_, T> {
	fn with(value: &T, effects: Vec<Effect>, reset: bool) -> Styled<'_, T> {
		todo!()
	}
}
impl Stylize<String> for String {
	fn with(value: &String, effects: Vec<Effect>, reset: bool) -> Styled<'_, String> {
		todo!()
	}
}

#[cfg(test)]
mod tests {
	use super::Stylize as _;

	#[test]
	fn it_works() {
		// assert_eq!(format!("{}", "red".bold().color(4)), "\x1b[1m\x1b[31mred\x1b[m\x1b[m");
		// let array = vec![1, 2, 3];
		// assert_eq!(format!("{:?}", array.style().override_line()), "\x1b[2K\x1b[1G[1, 2, 3]\x1b[m");
		let string = String::from("truecolor");
		assert_eq!(format!("{}", string.bold().override_line()), "\x1b[48;2;255;255;0mtruecolor\x1b[m");
	}
}
