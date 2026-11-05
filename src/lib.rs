#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Easing {
    Linear,
    EaseInQuad,
    EaseOutQuad,
    EaseInOutQuad,
    CubicBezier(f32, f32, f32, f32),
}

#[derive(Debug, Clone)]
pub struct AnimationTrack {
    pub start_val: f32,
    pub end_val: f32,
    pub duration_ms: u64,
    pub easing: Easing,
}

impl AnimationTrack {
    pub fn new(start: f32, end: f32, duration_ms: u64, easing: Easing) -> Self {
        Self { start_val: start, end_val: end, duration_ms, easing }
    }

    pub fn sample(&self, elapsed_ms: u64) -> f32 {
        if self.duration_ms == 0 {
            return self.end_val;
        }
        let t = (elapsed_ms as f32 / self.duration_ms as f32).clamp(0.0, 1.0);
        let factor = match self.easing {
            Easing::Linear => t,
            Easing::EaseInQuad => t * t,
            Easing::EaseOutQuad => t * (2.0 - t),
            Easing::EaseInOutQuad => {
                if t < 0.5 { 2.0 * t * t } else { -1.0 + (4.0 - 2.0 * t) * t }
            }
            Easing::CubicBezier(_x1, _y1, _x2, _y2) => {
                t * t * (3.0 - 2.0 * t)
            }
        };
        self.start_val + (self.end_val - self.start_val) * factor
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_animation_interpolation() {
        // Arrange
        let track = AnimationTrack::new(0.0, 100.0, 1000, Easing::Linear);
        // Act
        let mid = track.sample(500);
        let end = track.sample(1000);
        // Assert
        assert!((mid - 50.0).abs() < 1e-4);
        assert!((end - 100.0).abs() < 1e-4);
    }
}
