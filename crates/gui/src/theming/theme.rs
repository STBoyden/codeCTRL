use super::Palette;
use iced::theme::Palette as IcedPalette;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Hash, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct VariantTheme {
	theme_name: String,
	dark_theme: bool,
	file_path: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Hash, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Theme {
	theme_name: String,
	dark_theme: bool,
	variant_theme: Option<VariantTheme>,
	palette: Palette,
}

impl Theme {
	pub fn get_name(&self) -> &str { &self.theme_name }
	pub fn is_dark_theme(&self) -> bool { self.dark_theme }
	pub fn get_variant_theme(&self) -> Option<&VariantTheme> { self.variant_theme.as_ref() }
	pub fn get_palette(&self) -> &Palette { &self.palette }

	pub fn catppuccin_frappe_sky() -> Self {
		Self {
			theme_name: String::from("Catppuccin Frappe - Sky"),
			dark_theme: true,
			variant_theme: None,
			palette: Palette::frappe_sky(),
		}
	}
}

impl TryInto<IcedPalette> for Theme {
	type Error = super::palette::Error;

	fn try_into(self) -> Result<IcedPalette, Self::Error> {
		Ok(IcedPalette {
			background: self.palette.background.try_into()?,
			text: self.palette.foreground.try_into()?,
			primary: self.palette.primary.try_into()?,
			success: self.palette.success.try_into()?,
			danger: self.palette.danger.try_into()?,
		})
	}
}
