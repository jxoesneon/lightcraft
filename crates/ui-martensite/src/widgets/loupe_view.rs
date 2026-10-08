//! Loupe view widget with subpixel pan/zoom and a mask overlay indicator.

pub struct LoupeViewWidget {
    pub zoom: f32,
    pub pan_offset: [f32; 2],
    pub image_size: [u32; 2],
    pub rulers_visible: bool,
    /// Animated phase for the mask overlay marching outline.
    pub overlay_phase: f32,
}

impl LoupeViewWidget {
    pub fn new(width: u32, height: u32) -> Self {
        Self { zoom: 1.0, pan_offset: [0.0, 0.0], image_size: [width, height], rulers_visible: true, overlay_phase: 0.0 }
    }

    pub fn zoom_at(&mut self, factor: f32, cursor: [f32; 2]) {
        let old_zoom = self.zoom;
        let new_zoom = (self.zoom * factor).clamp(0.01, 64.0);
        let ratio = new_zoom / old_zoom;

        self.pan_offset[0] = cursor[0] - (cursor[0] - self.pan_offset[0]) * ratio;
        self.pan_offset[1] = cursor[1] - (cursor[1] - self.pan_offset[1]) * ratio;
        self.zoom = new_zoom;
    }

    pub fn advance_overlay_animation(&mut self, dt: f32) {
        self.overlay_phase = (self.overlay_phase + dt * 2.0) % 1.0;
    }

    pub fn screen_to_image(&self, screen: [f32; 2]) -> [f32; 2] {
        [(screen[0] - self.pan_offset[0]) / self.zoom, (screen[1] - self.pan_offset[1]) / self.zoom]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_image_coordinates_and_zoom() {
        let mut loupe = LoupeViewWidget::new(6000, 4000);
        assert_eq!(loupe.screen_to_image([100.0, 100.0]), [100.0, 100.0]);

        loupe.zoom_at(2.0, [0.0, 0.0]);
        assert_eq!(loupe.zoom, 2.0);
        assert_eq!(loupe.screen_to_image([100.0, 100.0]), [50.0, 50.0]);

        loupe.advance_overlay_animation(0.25);
        assert!((loupe.overlay_phase - 0.5).abs() < 1e-4);
    }
}
