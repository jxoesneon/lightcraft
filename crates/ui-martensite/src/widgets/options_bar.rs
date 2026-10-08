//! Options bar widget adapting dynamically to the active tool
//! (masking brush size/feather/flow/density, crop aspect, spot radius).

use crate::widgets::scrubby_input::ScrubbyInputWidget;
use lightcraft_engine::Tool;

pub struct OptionsBarWidget {
    pub active_tool: Tool,
    pub brush_size: ScrubbyInputWidget,
    pub brush_feather: ScrubbyInputWidget,
    pub flow: ScrubbyInputWidget,
    pub density: ScrubbyInputWidget,
    /// Overall mask strength (0–200 %), applied on top of every component.
    pub mask_amount: ScrubbyInputWidget,
    pub auto_mask: bool,
    pub invert_mask: bool,
}

impl OptionsBarWidget {
    pub fn new() -> Self {
        Self {
            active_tool: Tool::MaskingBrush,
            brush_size: ScrubbyInputWidget::new("Size", 30.0, 1.0, 500.0, "px"),
            brush_feather: ScrubbyInputWidget::new("Feather", 50.0, 0.0, 100.0, "%"),
            flow: ScrubbyInputWidget::new("Flow", 100.0, 1.0, 100.0, "%"),
            density: ScrubbyInputWidget::new("Density", 100.0, 1.0, 100.0, "%"),
            mask_amount: ScrubbyInputWidget::new("Amount", 100.0, 0.0, 200.0, "%"),
            auto_mask: false,
            invert_mask: false,
        }
    }

    pub fn set_tool(&mut self, tool: Tool) {
        self.active_tool = tool;
    }

    pub fn toggle_auto_mask(&mut self) -> bool {
        self.auto_mask = !self.auto_mask;
        self.auto_mask
    }

    pub fn toggle_invert_mask(&mut self) -> bool {
        self.invert_mask = !self.invert_mask;
        self.invert_mask
    }
}

impl Default for OptionsBarWidget {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_options_bar_defaults() {
        let mut bar = OptionsBarWidget::new();
        assert_eq!(bar.active_tool, Tool::MaskingBrush);
        assert_eq!(bar.brush_size.value, 30.0);
        assert_eq!(bar.density.value, 100.0);
        assert!(!bar.auto_mask);

        bar.set_tool(Tool::SpotRemove);
        assert_eq!(bar.active_tool, Tool::SpotRemove);

        assert!(bar.toggle_auto_mask());
        assert!(bar.auto_mask);
        assert!(!bar.toggle_auto_mask());

        assert!(bar.toggle_invert_mask());
        assert!(bar.invert_mask);
    }
}
