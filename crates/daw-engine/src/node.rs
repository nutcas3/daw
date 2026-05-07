//! Audio processing nodes for effects and synthesis

/// Base trait for all audio processors
pub trait AudioNode {
    /// Process audio in-place
    fn process(&mut self, buffer: &mut [f32]);
    
    /// Reset internal state
    fn reset(&mut self);
}

/// Gain node - adjusts volume
pub struct GainNode {
    gain: f32,
}

impl GainNode {
    pub fn new(gain: f32) -> Self {
        Self { gain }
    }
    
    pub fn set_gain(&mut self, gain: f32) {
        self.gain = gain;
    }
}

impl AudioNode for GainNode {
    fn process(&mut self, buffer: &mut [f32]) {
        for sample in buffer.iter_mut() {
            *sample *= self.gain;
        }
    }
    
    fn reset(&mut self) {
        // No state to reset
    }
}

/// Pan node - stereo panning (constant power)
pub struct PanNode {
    pan: f32,  // -1.0 (left) to 1.0 (right)
}

impl PanNode {
    pub fn new(pan: f32) -> Self {
        Self { pan: pan.clamp(-1.0, 1.0) }
    }
    
    pub fn set_pan(&mut self, pan: f32) {
        self.pan = pan.clamp(-1.0, 1.0);
    }
    
    /// Apply panning to stereo buffer (interleaved L/R)
    pub fn process_stereo(&mut self, buffer: &mut [f32]) {
        assert!(buffer.len() % 2 == 0, "Buffer must be stereo (even length)");
        
        let angle = (self.pan + 1.0) * 0.25 * std::f32::consts::PI;
        let left_gain = angle.cos();
        let right_gain = angle.sin();
        
        for chunk in buffer.chunks_exact_mut(2) {
            let mono = (chunk[0] + chunk[1]) * 0.5;
            chunk[0] = mono * left_gain;
            chunk[1] = mono * right_gain;
        }
    }
}

impl AudioNode for PanNode {
    fn process(&mut self, buffer: &mut [f32]) {
        // For mono buffer, this is a no-op
        // Use process_stereo for actual panning
    }
    
    fn reset(&mut self) {
        // No state to reset
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gain_node() {
        let mut node = GainNode::new(0.5);
        let mut buffer = vec![1.0, 1.0, 1.0, 1.0];
        
        node.process(&mut buffer);
        
        for &sample in &buffer {
            assert!((sample - 0.5).abs() < 0.0001);
        }
    }

    #[test]
    fn test_gain_adjustment() {
        let mut node = GainNode::new(1.0);
        let mut buffer = vec![1.0; 4];
        
        node.set_gain(0.25);
        node.process(&mut buffer);
        
        for &sample in &buffer {
            assert!((sample - 0.25).abs() < 0.0001);
        }
    }

    #[test]
    fn test_pan_node_center() {
        let mut node = PanNode::new(0.0);  // Center
        let mut buffer = vec![1.0, 1.0, 1.0, 1.0];  // Stereo
        
        node.process_stereo(&mut buffer);
        
        // Should be roughly equal for center pan
        assert!((buffer[0] - buffer[1]).abs() < 0.1);
    }
}
