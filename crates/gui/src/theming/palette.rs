use iced::Color;
use serde::{Deserialize, Serialize};
use std::num::ParseIntError;

#[derive(Debug, Clone, thiserror::Error, PartialEq)]
pub enum Error {
	#[error("invalid colour hex format")]
	HexInvalidFormat,
	#[error("hex colour string was too long. Got {0}, expected 6.")]
	HexStringTooLong(usize),
	#[error("hex colour string was too short. Got {0}, expected 6")]
	HexStringTooShort(usize),
	#[error("invalid hex character: {0}.")]
	HexInvalidCharacter(#[from] ParseIntError),
	#[error(
		"invalid HSV: expected h (0..360.0), s (0..1.0), v (0..1.0), got h = {0}, s = {1}, v = {2}"
	)]
	HsvInvalid(f32, f32, f32),
}

#[derive(Debug, Serialize, Deserialize, Clone, Hash, PartialEq, Eq)]
#[serde(untagged)]
pub enum Colour {
	Rgb { r: u8, g: u8, b: u8 },
	Hex(String),
	// HSV { h: f32, s: f32, v: f32 },
}

type Result<T> = std::result::Result<T, Error>;

impl TryInto<Color> for Colour {
	type Error = Error;

	fn try_into(self) -> Result<Color> {
		match self {
			Self::Rgb { r, g, b } => Ok(Color {
				r: f32::from(r) / 255.0,
				g: f32::from(g) / 255.0,
				b: f32::from(b) / 255.0,
				a: 1.0,
			}),
			Self::Hex(mut hex_code) => {
				if !hex_code.starts_with('#') {
					return Err(Error::HexInvalidFormat);
				}

				hex_code.remove(0);

				if hex_code.len() > 6 {
					return Err(Error::HexStringTooLong(hex_code.len()));
				}

				if hex_code.len() < 6 {
					return Err(Error::HexStringTooShort(hex_code.len()));
				}

				let chars = hex_code.chars().collect::<Vec<char>>();
				let chunks = chars.chunks_exact(2).collect::<Vec<_>>();

				let mut rgb = [0u8; 3];

				for (index, chars) in chunks.iter().enumerate() {
					let string = chars.iter().collect::<String>();

					if let Some(v) = rgb.get_mut(index) {
						*v = u8::from_str_radix(&string, 16).map_err(Error::HexInvalidCharacter)?;
					}
				}

				Ok(Color {
					r: f32::from(rgb[0]) / 255.0,
					g: f32::from(rgb[1]) / 255.0,
					b: f32::from(rgb[2]) / 255.0,
					a: 1.0,
				})
			},
			// Self::HSV { h, s, v } if h < 360.0 && s <= 1.0 && v <= 1.0 => {
			// 	let i = h * 6.0;
			// 	let f = h * 6.0 - i;
			// 	let p = v * (1.0 - s);
			// 	let q = v * (1.0 - f * s);
			// 	let t = v * (1.0 - (1.0 - f) * s);
			//
			// 	let rgb = match i as u32 % 6 {
			// 		0 => (v, t, p),
			// 		1 => (q, v, p),
			// 		2 => (p, v, t),
			// 		3 => (p, q, v),
			// 		4 => (t, p, v),
			// 		5 => (v, p, q),
			// 		_ => unreachable!("i % 6 should not reach any number above 5"),
			// 	};
			//
			// 	let (r, g, b) = rgb;
			//
			// 	Ok(Color { r, g, b, a: 1.0 })
			// },
			// Self::HSV { h, s, v } => Err(Error::HsvInvalid(h, s, v)),
		}
	}
}

#[derive(Debug, Serialize, Deserialize, Clone, Hash, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Palette {
	pub(crate) foreground: Colour,
	pub(crate) background: Colour,
	pub(crate) primary: Colour,
	pub(crate) success: Colour,
	pub(crate) danger: Colour,
}

impl Palette {
	pub fn frappe_sky() -> Self {
		Self {
			foreground: Colour::Hex(String::from("#c6d0f5")),
			background: Colour::Hex(String::from("#303446")),
			primary: Colour::Hex(String::from("#99d1db")),
			success: Colour::Hex(String::from("#a6d189")),
			danger: Colour::Hex(String::from("#e78284")),
		}
	}
}
