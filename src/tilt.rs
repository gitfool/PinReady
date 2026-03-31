/// Tilt/nudge sensitivity configuration state.
#[derive(Debug, Clone)]
pub struct TiltConfig {
    /// Nudge sensitivity (0.0 = min, 1.0 = max) — controls NudgeStrength
    pub nudge_sensitivity: f32,
    /// Tilt sensitivity (0.0 = min, 1.0 = max) — controls PlumbThreshold + PlumbInertia
    pub tilt_sensitivity: f32,
    /// Show advanced controls
    pub advanced_mode: bool,
    /// Individual parameters (advanced mode)
    pub nudge_strength: f32,
    pub plumb_inertia: f32,
    pub plumb_threshold_angle: f32,
    pub nudge_filter_0: bool,
    pub nudge_filter_1: bool,
}

impl Default for TiltConfig {
    fn default() -> Self {
        Self {
            nudge_sensitivity: 0.5,
            tilt_sensitivity: 0.5,
            advanced_mode: false,
            nudge_strength: 0.02,
            plumb_inertia: 0.35,
            plumb_threshold_angle: 35.0,
            nudge_filter_0: false,
            nudge_filter_1: false,
        }
    }
}

impl TiltConfig {
    pub fn load_from_config(&mut self, config: &crate::config::VpxConfig) {
        if let Some(v) = config.get_f32("Player", "NudgeStrength") { self.nudge_strength = v; }
        if let Some(v) = config.get_f32("Player", "PlumbInertia") { self.plumb_inertia = v; }
        if let Some(v) = config.get_f32("Player", "PlumbThresholdAngle") { self.plumb_threshold_angle = v; }
        if let Some(v) = config.get_i32("Player", "NudgeFilter0") { self.nudge_filter_0 = v != 0; }
        if let Some(v) = config.get_i32("Player", "NudgeFilter1") { self.nudge_filter_1 = v != 0; }
        self.nudge_sensitivity = self.compute_nudge_sensitivity();
        self.tilt_sensitivity = self.compute_tilt_sensitivity();
    }

    /// Apply nudge sensitivity slider to NudgeStrength.
    pub fn apply_nudge_sensitivity(&mut self) {
        let t = self.nudge_sensitivity;
        self.nudge_strength = 0.005 + t * 0.245;
    }

    /// Apply tilt sensitivity slider to PlumbInertia + PlumbThresholdAngle.
    pub fn apply_tilt_sensitivity(&mut self) {
        let t = self.tilt_sensitivity;
        self.plumb_inertia = 0.8 - t * 0.75;
        self.plumb_threshold_angle = 55.0 - t * 47.0;
    }

    pub fn save_to_config(&self, config: &mut crate::config::VpxConfig) {
        config.set_nudge_strength(self.nudge_strength);
        config.set_plumb_inertia(self.plumb_inertia);
        config.set_plumb_threshold_angle(self.plumb_threshold_angle);
        config.set_nudge_filter(0, self.nudge_filter_0);
        config.set_nudge_filter(1, self.nudge_filter_1);
    }

    fn compute_nudge_sensitivity(&self) -> f32 {
        ((self.nudge_strength - 0.005) / 0.245).clamp(0.0, 1.0)
    }

    fn compute_tilt_sensitivity(&self) -> f32 {
        ((55.0 - self.plumb_threshold_angle) / 47.0).clamp(0.0, 1.0)
    }
}
