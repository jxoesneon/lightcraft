//! Color-neutral dark studio palette for photo evaluation.

pub struct Color(pub u8, pub u8, pub u8);

pub struct Theme {
    pub surface_loupe: Color,
    pub surface_sidebar: Color,
    pub accent_curve: Color,
}

impl Theme {
    pub fn dark_studio() -> Self {
        Self {
            surface_loupe: Color(28, 28, 28),
            surface_sidebar: Color(38, 38, 38),
            accent_curve: Color(0, 210, 255),
        }
    }
}
