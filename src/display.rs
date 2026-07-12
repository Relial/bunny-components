use bunny_plugin::bunny_ui::align::Align2;

pub trait MenuDisplay {
    fn menu_display(&self) -> &str;
}

impl MenuDisplay for Align2 {
    fn menu_display(&self) -> &str {
        match *self {
            Self::LEFT_BOTTOM => "Left Bottom",
            Self::LEFT_CENTER => "Left Center",
            Self::LEFT_TOP => "Left Top",

            Self::CENTER_BOTTOM => "Center Bottom",
            Self::CENTER_CENTER => "Center Center",
            Self::CENTER_TOP => "Center Top",

            Self::RIGHT_BOTTOM => "Right Bottom",
            Self::RIGHT_CENTER => "Right Center",
            Self::RIGHT_TOP => "Right Top",
        }
    }
}
