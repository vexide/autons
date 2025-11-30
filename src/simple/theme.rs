use vexide::color::Color;

/// Default dark theme for [`SimpleSelect`].
///
/// [`SimpleSelect`]: super::SimpleSelect
pub const THEME_DARK: SimpleSelectTheme = SimpleSelectTheme {
    background_default: Color::new(25, 25, 25),
    background_active: Color::new(102, 102, 102),
    background_selected: Color::new(67, 189, 224),
    background_selected_active: Color::new(123, 209, 233),

    text_default: Color::new(187, 187, 187),
    text_selected: Color::new(255, 255, 255),
    text_active: Color::new(187, 187, 187),
    text_selected_active: Color::new(255, 255, 255),

    border: Color::new(153, 153, 153),
};

/// Color theme for the [`SimpleSelect`] autonomous selector.
///
/// [`SimpleSelect`]: super::SimpleSelect
#[derive(Debug, Eq, PartialEq)]
pub struct SimpleSelectTheme {
    pub background_default: Color,
    pub background_active: Color,
    pub background_selected: Color,
    pub background_selected_active: Color,

    pub text_default: Color,
    pub text_active: Color,
    pub text_selected: Color,
    pub text_selected_active: Color,

    pub border: Color,
}

impl Default for SimpleSelectTheme {
    fn default() -> Self {
        THEME_DARK
    }
}
