//! Sovereign retained-mode interface for LightCraft built on the Martensite GUI engine.

#![forbid(unsafe_code)]
#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::unimplemented, clippy::todo, clippy::unreachable)]

pub mod command_reg;
pub mod develop_sliders;
pub mod menus;
pub mod shortcuts;
pub mod theme;
pub mod widgets;

use std::sync::{Arc, Mutex};

use lightcraft_engine::Engine;

/// Application state container managing the Martensite GUI pipeline.
pub struct LightcraftApp {
    pub engine: Arc<Mutex<Engine>>,
    pub theme: theme::CraftTheme,
    pub keyboard: shortcuts::KeyboardEngine,
    pub active_tool: lightcraft_engine::Tool,
    /// Photometric develop parameters bound to the Loupe view's slider panels.
    pub develop: develop_sliders::DevelopParams,
    pub zoom_level: f32,
    pub pan_offset: [f32; 2],
    pub rulers_visible: bool,
    /// Before/After split view (`Y`): the loupe shows original and developed side by side.
    pub before_after_split: bool,
    pub is_dirty: bool,
}

impl LightcraftApp {
    pub fn new(engine: Engine) -> Self {
        Self {
            engine: Arc::new(Mutex::new(engine)),
            theme: theme::CraftTheme::dark_studio(),
            keyboard: shortcuts::KeyboardEngine::new(),
            active_tool: lightcraft_engine::Tool::Loupe,
            develop: develop_sliders::DevelopParams::default(),
            zoom_level: 1.0,
            pan_offset: [0.0, 0.0],
            rulers_visible: true,
            before_after_split: false,
            is_dirty: false,
        }
    }

    pub fn set_tool(&mut self, tool: lightcraft_engine::Tool) {
        self.active_tool = tool;
    }

    pub fn set_zoom(&mut self, zoom: f32) {
        self.zoom_level = zoom.clamp(0.01, 64.0);
    }

    pub fn pan_by(&mut self, dx: f32, dy: f32) {
        self.pan_offset[0] += dx;
        self.pan_offset[1] += dy;
    }

    pub fn reset_view(&mut self) {
        self.zoom_level = 1.0;
        self.pan_offset = [0.0, 0.0];
    }

    pub fn toggle_rulers(&mut self) -> bool {
        self.rulers_visible = !self.rulers_visible;
        self.rulers_visible
    }

    pub fn toggle_before_after(&mut self) -> bool {
        self.before_after_split = !self.before_after_split;
        self.before_after_split
    }

    /// Reset every develop parameter to its neutral value (Develop ▸ Reset).
    pub fn reset_adjustments(&mut self) {
        self.develop = develop_sliders::DevelopParams::default();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_initialization() {
        let engine = Engine::new();
        let app = LightcraftApp::new(engine);
        assert_eq!(app.active_tool, lightcraft_engine::Tool::Loupe);
        assert_eq!(app.zoom_level, 1.0);
        assert_eq!(app.pan_offset, [0.0, 0.0]);
        assert!(app.rulers_visible);
        assert!(!app.before_after_split);
        assert!(!app.is_dirty);
        assert_eq!(app.develop.exposure, 0.0);
        assert_eq!(app.develop.temperature, 5500);
    }

    #[test]
    fn test_zoom_clamping() {
        let engine = Engine::new();
        let mut app = LightcraftApp::new(engine);

        app.set_zoom(2.5);
        assert_eq!(app.zoom_level, 2.5);

        app.set_zoom(0.0001);
        assert_eq!(app.zoom_level, 0.01);

        app.set_zoom(1000.0);
        assert_eq!(app.zoom_level, 64.0);
    }

    #[test]
    fn test_pan_and_reset() {
        let engine = Engine::new();
        let mut app = LightcraftApp::new(engine);

        app.pan_by(120.0, -45.0);
        assert_eq!(app.pan_offset, [120.0, -45.0]);

        app.set_zoom(3.0);
        app.reset_view();
        assert_eq!(app.zoom_level, 1.0);
        assert_eq!(app.pan_offset, [0.0, 0.0]);
    }

    #[test]
    fn test_toggles_and_develop_reset() {
        let engine = Engine::new();
        let mut app = LightcraftApp::new(engine);

        assert!(app.rulers_visible);
        assert!(!app.toggle_rulers());
        assert!(!app.rulers_visible);
        assert!(app.toggle_rulers());

        assert!(!app.before_after_split);
        assert!(app.toggle_before_after());
        assert!(app.before_after_split);
        assert!(!app.toggle_before_after());

        app.develop.exposure = 1.25;
        app.reset_adjustments();
        assert_eq!(app.develop.exposure, 0.0);
    }
}
