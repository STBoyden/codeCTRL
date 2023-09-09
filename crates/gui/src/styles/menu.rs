use iced_aw::menu;

pub struct MenuStyle;

impl menu::StyleSheet for MenuStyle {
	type Style = iced::Theme;

	fn appearance(&self, _style: &Self::Style) -> menu::Appearance { todo!() }
}

#[allow(dead_code)]
pub struct MenuButtonStyle;
