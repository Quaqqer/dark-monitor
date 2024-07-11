#[derive(Debug)]
pub enum ColorSchemePreference {
    Default = 0,
    PreferDark = 1,
    PreferLight = 2,
}

impl TryFrom<u32> for ColorSchemePreference {
    type Error = &'static str;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(ColorSchemePreference::Default),
            1 => Ok(ColorSchemePreference::PreferDark),
            2 => Ok(ColorSchemePreference::PreferLight),
            _ => Err("Could not convert value to color scheme preference"),
        }
    }
}
