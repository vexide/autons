pub use vexide::devices::rgb::Rgb;

/// Default dark theme for [`SimpleSelect`].
/// 
/// [`SimpleSelect`]: super::SimpleSelect
pub const THEME_DARK: SimpleSelectTheme = SimpleSelectTheme {
    background_default: Rgb::new(25, 25, 25),
    background_active: Rgb::new(102, 102, 102),
    background_selected: Rgb::new(67, 189, 224),
    background_selected_active: Rgb::new(123, 209, 233),

    text_default: Rgb::new(187, 187, 187),
    text_selected: Rgb::new(255, 255, 255),
    text_active: Rgb::new(187, 187, 187),
    text_selected_active: Rgb::new(255, 255, 255),

    border: Rgb::new(153, 153, 153),
};

/// Color theme for the [`SimpleSelect`] autonomous selector.
/// 
/// [`SimpleSelect`]: super::SimpleSelect
#[derive(Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct SimpleSelectTheme {
    pub background_default: Rgb<u8>,
    pub background_active: Rgb<u8>,
    pub background_selected: Rgb<u8>,
    pub background_selected_active: Rgb<u8>,

    pub text_default: Rgb<u8>,
    pub text_active: Rgb<u8>,
    pub text_selected: Rgb<u8>,
    pub text_selected_active: Rgb<u8>,

    pub border: Rgb<u8>,
}

impl Default for SimpleSelectTheme {
    fn default() -> Self {
        THEME_DARK
    }
}
