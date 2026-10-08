//! Tool strip widget: view/develop tool slots and the mask overlay toggle.

use lightcraft_engine::Tool;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ToolSlot {
    pub primary: Tool,
    pub alternatives: &'static [Tool],
}

pub const TOOL_SLOTS: &[ToolSlot] = &[
    ToolSlot { primary: Tool::Grid, alternatives: &[] },
    ToolSlot { primary: Tool::Loupe, alternatives: &[] },
    ToolSlot { primary: Tool::Compare, alternatives: &[] },
    ToolSlot { primary: Tool::Survey, alternatives: &[] },
    ToolSlot { primary: Tool::Crop, alternatives: &[] },
    ToolSlot { primary: Tool::SpotRemove, alternatives: &[] },
    ToolSlot { primary: Tool::RedEye, alternatives: &[] },
    ToolSlot { primary: Tool::MaskingBrush, alternatives: &[] },
    ToolSlot { primary: Tool::LinearGradient, alternatives: &[Tool::RadialGradient] },
    ToolSlot { primary: Tool::RadialGradient, alternatives: &[] },
    ToolSlot { primary: Tool::Hand, alternatives: &[] },
    ToolSlot { primary: Tool::Zoom, alternatives: &[] },
];

pub struct ToolStripWidget {
    pub active_tool: Tool,
    pub double_column: bool,
    /// Red-tinted mask overlay shown over the loupe (`O`).
    pub mask_overlay: bool,
}

impl ToolStripWidget {
    pub fn new() -> Self {
        Self { active_tool: Tool::Loupe, double_column: false, mask_overlay: false }
    }

    pub fn toggle_column_mode(&mut self) -> bool {
        self.double_column = !self.double_column;
        self.double_column
    }

    pub fn toggle_mask_overlay(&mut self) -> bool {
        self.mask_overlay = !self.mask_overlay;
        self.mask_overlay
    }
}

impl Default for ToolStripWidget {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tool_strip_state() {
        let mut strip = ToolStripWidget::new();
        assert_eq!(strip.active_tool, Tool::Loupe);
        assert!(!strip.double_column);
        assert!(!strip.mask_overlay);

        assert!(strip.toggle_column_mode());
        assert!(strip.double_column);
        assert!(!strip.toggle_column_mode());

        assert!(strip.toggle_mask_overlay());
        assert!(strip.mask_overlay);
        assert!(!strip.toggle_mask_overlay());
    }

    #[test]
    fn test_tool_slots_cover_all_tools() {
        assert_eq!(TOOL_SLOTS.len(), 12);
        for slot in TOOL_SLOTS {
            for alt in slot.alternatives {
                assert_ne!(*alt, slot.primary);
            }
        }
    }
}
