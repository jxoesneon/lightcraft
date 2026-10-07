//! Photometric RAW develop parameters.

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DevelopParams {
    pub temperature: u32,  // 2000K .. 50000K
    pub tint: i32,         // -150 .. +150
    pub exposure: f32,     // -5.0 .. +5.0 EV
    pub contrast: f32,     // -100 .. +100
    pub highlights: f32,   // -100 .. +100
    pub shadows: f32,      // -100 .. +100
    pub whites: f32,       // -100 .. +100
    pub blacks: f32,       // -100 .. +100
    pub vibrance: f32,     // -100 .. +100
    pub saturation: f32,   // -100 .. +100
}

impl Default for DevelopParams {
    fn default() -> Self {
        Self {
            temperature: 5500,
            tint: 0,
            exposure: 0.0,
            contrast: 0.0,
            highlights: 0.0,
            shadows: 0.0,
            whites: 0.0,
            blacks: 0.0,
            vibrance: 0.0,
            saturation: 0.0,
        }
    }
}

impl DevelopParams {
    pub fn clamp_all(&mut self) {
        self.temperature = self.temperature.clamp(2000, 50000);
        self.tint = self.tint.clamp(-150, 150);
        self.exposure = self.exposure.clamp(-5.0, 5.0);
        self.contrast = self.contrast.clamp(-100.0, 100.0);
        self.highlights = self.highlights.clamp(-100.0, 100.0);
        self.shadows = self.shadows.clamp(-100.0, 100.0);
        self.whites = self.whites.clamp(-100.0, 100.0);
        self.blacks = self.blacks.clamp(-100.0, 100.0);
        self.vibrance = self.vibrance.clamp(-100.0, 100.0);
        self.saturation = self.saturation.clamp(-100.0, 100.0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_develop_clamping() {
        let mut d = DevelopParams::default();
        d.exposure = 10.0;
        d.temperature = 1000;
        d.clamp_all();
        assert_eq!(d.exposure, 5.0);
        assert_eq!(d.temperature, 2000);
    }
}
