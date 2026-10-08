//! Luminance range mask widget: dual split-slider thresholding over tonal range
//! (the range endpoints can feather independently, like a split "Blend If").

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SplitSliderRange {
    pub min_start: u8,
    pub min_end: u8,
    pub max_start: u8,
    pub max_end: u8,
}

impl SplitSliderRange {
    pub const fn default_full() -> Self {
        Self { min_start: 0, min_end: 0, max_start: 255, max_end: 255 }
    }

    pub fn is_split_min(&self) -> bool {
        self.min_start != self.min_end
    }

    pub fn is_split_max(&self) -> bool {
        self.max_start != self.max_end
    }

    pub fn set_min_split(&mut self, start: u8, end: u8) {
        let s = start.min(end);
        let e = start.max(end);
        self.min_start = s;
        self.min_end = e.min(self.max_start);
    }

    pub fn set_max_split(&mut self, start: u8, end: u8) {
        let s = start.min(end);
        let e = start.max(end);
        self.max_start = s.max(self.min_end);
        self.max_end = e;
    }
}

impl Default for SplitSliderRange {
    fn default() -> Self {
        Self::default_full()
    }
}

/// Luminance Range mask: shadows/highlights bounds plus an overall smoothness.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RangeMaskWidget {
    pub luminance: SplitSliderRange,
    /// Edge smoothness of the range falloff (0–100).
    pub smoothness: f32,
    pub invert: bool,
}

impl RangeMaskWidget {
    pub fn new() -> Self {
        Self { luminance: SplitSliderRange::default_full(), smoothness: 50.0, invert: false }
    }

    pub fn reset(&mut self) {
        self.luminance = SplitSliderRange::default_full();
        self.smoothness = 50.0;
        self.invert = false;
    }

    pub fn toggle_invert(&mut self) -> bool {
        self.invert = !self.invert;
        self.invert
    }
}

impl Default for RangeMaskWidget {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_range_mask_split_thresholds() {
        let mut widget = RangeMaskWidget::new();
        assert_eq!(widget.luminance.min_start, 0);
        assert_eq!(widget.luminance.max_end, 255);
        assert!(!widget.luminance.is_split_min());

        // Split the shadow bound for a feathered range edge.
        widget.luminance.set_min_split(20, 50);
        assert_eq!(widget.luminance.min_start, 20);
        assert_eq!(widget.luminance.min_end, 50);
        assert!(widget.luminance.is_split_min());

        // Split the highlight bound.
        widget.luminance.set_max_split(200, 230);
        assert_eq!(widget.luminance.max_start, 200);
        assert_eq!(widget.luminance.max_end, 230);
        assert!(widget.luminance.is_split_max());

        assert!(widget.toggle_invert());
        assert!(widget.invert);

        widget.reset();
        assert_eq!(widget.luminance, SplitSliderRange::default_full());
        assert!(!widget.invert);
    }
}
