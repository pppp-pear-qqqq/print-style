use derive::AnsiStylize;
pub use styled::Styled;

mod styled;

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
#[derive(Clone, Copy, AnsiStylize)]
#[repr(u8)]
#[allow(non_camel_case_types)]
pub enum Ansi {
	reset = 0,
	bold = 1,
	faint = 2,
	italic = 3,
	underline = 4,
	blink = 5,
	rapid_blink = 6,
	reverse = 7,
	conceal = 8,
	strike = 9,
	black = 30,
	red = 31,
	green = 32,
	yellow = 33,
	blue = 34,
	magenta = 35,
	cyan = 36,
	white = 37,
	bg_black = 40,
	bg_red = 41,
	bg_green = 42,
	bg_yellow = 43,
	bg_blue = 44,
	bg_magenta = 45,
	bg_cyan = 46,
	bg_white = 47,
	bright_black = 90,
	bright_red = 91,
	bright_green = 92,
	bright_yellow = 93,
	bright_blue = 94,
	bright_magent = 95,
	bright_cyan = 96,
	bright_white = 97,
	bg_bright_black = 100,
	bg_bright_red = 101,
	bg_bright_green = 102,
	bg_bright_yellow = 103,
	bg_bright_blue = 104,
	bg_bright_magent = 105,
	bg_bright_cyan = 106,
	bg_bright_white = 107,
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

pub trait Stylize: Sized {
	fn with(&self, effects: Vec<Effect>, reset: bool) -> Styled<'_, Self> {
		Styled { value: self, effects, reset }
	}

	fn color(&self, code: u8) -> Styled<'_, Self> {
		self.with(vec![Effect::Color(code)], true)
	}
	fn truecolor(&self, r: u8, g: u8, b: u8) -> Styled<'_, Self> {
		self.with(vec![Effect::TrueColor(r, g, b)], true)
	}
	fn bg_color(&self, code: u8) -> Styled<'_, Self> {
		self.with(vec![Effect::BgColor(code)], true)
	}
	fn bg_truecolor(&self, r: u8, g: u8, b: u8) -> Styled<'_, Self> {
		self.with(vec![Effect::BgTrueColor(r, g, b)], true)
	}
	fn override_line(&self) -> Styled<'_, Self> {
		self.with(vec![Effect::ClearLine, Effect::MoveCursorToStart], false)
	}
}
impl<T> Stylize for T {}

#[cfg(test)]
mod tests {
	use crate::{AnsiStylize as _, Stylize as _};

	#[test]
	fn it_works() {
		assert_eq!(format!("{}", "red".red().bold()), "\x1b[1m\x1b[31mred\x1b[m\x1b[m");
		let array = vec![1, 2, 3];
		assert_eq!(format!("{:?}", array.override_line()), "\x1b[2K\x1b[1G[1, 2, 3]\x1b[m");
		let string = String::from("truecolor");
		assert_eq!(format!("{}", string.bg_truecolor(255, 255, 0)), "\x1b[48;2;255;255;0mtruecolor\x1b[m");
	}
}
