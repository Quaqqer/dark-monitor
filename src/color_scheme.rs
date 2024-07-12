use clap::ValueEnum;

#[derive(Clone, Copy, Debug, ValueEnum)]
pub enum ColorScheme {
    Default = 0,
    Dark = 1,
    Light = 2,
}

#[derive(Clone, Copy, Debug, ValueEnum)]
pub enum DarkLight {
    Dark,
    Light,
}

impl ColorScheme {
    pub fn with_default_as(&self, default_as: DarkLight) -> Self {
        match self {
            ColorScheme::Default => default_as.into(),
            p => *p,
        }
    }

    pub fn with_maybe_default_as(&self, default_as: Option<DarkLight>) -> Self {
        match self {
            ColorScheme::Default => {
                if let Some(default_as) = default_as {
                    default_as.into()
                } else {
                    ColorScheme::Default
                }
            }
            p => *p,
        }
    }

    pub fn into_dark_light(&self, default_as: DarkLight) -> DarkLight {
        match self {
            ColorScheme::Default => default_as,
            ColorScheme::Dark => DarkLight::Dark,
            ColorScheme::Light => DarkLight::Light,
        }
    }
}

impl DarkLight {
    pub fn toggle(&self) -> Self {
        match self {
            DarkLight::Dark => DarkLight::Light,
            DarkLight::Light => DarkLight::Dark,
        }
    }
}

impl std::fmt::Display for ColorScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ColorScheme::Default => write!(f, "default"),
            ColorScheme::Dark => write!(f, "dark"),
            ColorScheme::Light => write!(f, "light"),
        }
    }
}

impl std::fmt::Display for DarkLight {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DarkLight::Dark => write!(f, "dark"),
            DarkLight::Light => write!(f, "light"),
        }
    }
}

impl TryFrom<u32> for ColorScheme {
    type Error = &'static str;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(ColorScheme::Default),
            1 => Ok(ColorScheme::Dark),
            2 => Ok(ColorScheme::Light),
            _ => Err("Could not convert value to color scheme preference"),
        }
    }
}

impl From<DarkLight> for ColorScheme {
    fn from(value: DarkLight) -> Self {
        match value {
            DarkLight::Dark => ColorScheme::Dark,
            DarkLight::Light => ColorScheme::Light,
        }
    }
}
