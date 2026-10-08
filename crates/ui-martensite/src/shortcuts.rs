//! Keystroke state machine providing Lightroom keyboard ergonomics.

use lightcraft_engine::Tool;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KeyModifiers {
    pub shift: bool,
    pub ctrl: bool,
    pub alt: bool,
    pub cmd: bool,
}

impl KeyModifiers {
    pub const fn empty() -> Self {
        Self { shift: false, ctrl: false, alt: false, cmd: false }
    }
}

pub struct KeyboardEngine {
    pub prior_tool: Option<Tool>,
    pub space_held: bool,
    pub z_held: bool,
    pub alt_held: bool,
}

impl KeyboardEngine {
    pub fn new() -> Self {
        Self { prior_tool: None, space_held: false, z_held: false, alt_held: false }
    }

    pub fn on_key_down(&mut self, key: &str, current: Tool) -> Option<Tool> {
        match key {
            "Space" if !self.space_held => {
                self.space_held = true;
                self.prior_tool = Some(current);
                Some(Tool::Zoom)
            }
            "z" | "Z" if !self.z_held => {
                self.z_held = true;
                self.prior_tool = Some(current);
                Some(Tool::Zoom)
            }
            "Alt" => {
                self.alt_held = true;
                None
            }
            // Single-key view and tool shortcuts
            "g" | "G" => Some(Tool::Grid),
            "e" | "E" => Some(Tool::Loupe),
            "c" | "C" => Some(Tool::Compare),
            "n" | "N" => Some(Tool::Survey),
            "r" | "R" => Some(Tool::Crop),
            "q" | "Q" => Some(Tool::SpotRemove),
            "k" | "K" => Some(Tool::MaskingBrush),
            "m" | "M" => Some(Tool::LinearGradient),
            "h" | "H" => Some(Tool::Hand),
            _ => None,
        }
    }

    pub fn on_key_up(&mut self, key: &str) -> Option<Tool> {
        match key {
            "Space" if self.space_held => {
                self.space_held = false;
                self.prior_tool.take()
            }
            "z" | "Z" if self.z_held => {
                self.z_held = false;
                self.prior_tool.take()
            }
            "Alt" => {
                self.alt_held = false;
                None
            }
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_key_tool_switching() {
        let mut k = KeyboardEngine::new();
        assert_eq!(k.on_key_down("g", Tool::Loupe), Some(Tool::Grid));
        assert_eq!(k.on_key_down("E", Tool::Grid), Some(Tool::Loupe));
        assert_eq!(k.on_key_down("c", Tool::Loupe), Some(Tool::Compare));
        assert_eq!(k.on_key_down("n", Tool::Compare), Some(Tool::Survey));
        assert_eq!(k.on_key_down("r", Tool::Survey), Some(Tool::Crop));
        assert_eq!(k.on_key_down("q", Tool::Crop), Some(Tool::SpotRemove));
        assert_eq!(k.on_key_down("k", Tool::SpotRemove), Some(Tool::MaskingBrush));
        assert_eq!(k.on_key_down("m", Tool::MaskingBrush), Some(Tool::LinearGradient));
        assert_eq!(k.on_key_down("h", Tool::LinearGradient), Some(Tool::Hand));
    }

    #[test]
    fn test_spring_loaded_zoom_on_space() {
        let mut k = KeyboardEngine::new();
        let initial = Tool::MaskingBrush;

        // Press Space: temporary Zoom
        assert_eq!(k.on_key_down("Space", initial), Some(Tool::Zoom));
        assert!(k.space_held);

        // Multiple down events shouldn't overwrite prior tool
        assert_eq!(k.on_key_down("Space", Tool::Zoom), None);

        // Release Space: restores initial tool
        assert_eq!(k.on_key_up("Space"), Some(initial));
        assert!(!k.space_held);
    }

    #[test]
    fn test_spring_loaded_zoom_on_z() {
        let mut k = KeyboardEngine::new();
        let initial = Tool::Loupe;

        assert_eq!(k.on_key_down("z", initial), Some(Tool::Zoom));
        assert!(k.z_held);

        assert_eq!(k.on_key_up("z"), Some(initial));
        assert!(!k.z_held);
    }
}
