//! Sovereign retained-mode RAW development UI for LightCraft built on Martensite.

pub mod command_reg;
pub mod develop_sliders;
pub mod menus;
pub mod theme;

pub struct LightcraftApp {
    pub develop: develop_sliders::DevelopParams,
    pub loupe_zoom: f32,
    pub before_after_split: bool,
}

impl LightcraftApp {
    pub fn new() -> Self {
        Self {
            develop: develop_sliders::DevelopParams::default(),
            loupe_zoom: 1.0,
            before_after_split: false,
        }
    }

    pub fn reset_adjustments(&mut self) {
        self.develop = develop_sliders::DevelopParams::default();
    }

    pub fn toggle_before_after(&mut self) -> bool {
        self.before_after_split = !self.before_after_split;
        self.before_after_split
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_develop_state() {
        let mut app = LightcraftApp::new();
        assert_eq!(app.develop.exposure, 0.0);
        assert_eq!(app.develop.temperature, 5500);

        app.develop.exposure = 1.25;
        assert_eq!(app.develop.exposure, 1.25);

        assert!(app.toggle_before_after());
        assert!(app.before_after_split);

        app.reset_adjustments();
        assert_eq!(app.develop.exposure, 0.0);
    }
}
