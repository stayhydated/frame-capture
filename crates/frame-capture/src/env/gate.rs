use super::*;

impl CaptureFrameGate {
    pub fn frame(&self) -> u32 {
        self.frame
    }

    pub fn requested(&self) -> bool {
        self.requested
    }

    pub fn advance(&mut self) {
        if !self.requested {
            self.frame = self.frame.saturating_add(1);
        }
    }

    pub fn ready(&self, target_frame: CaptureFrame) -> bool {
        !self.requested && self.frame >= target_frame.get()
    }

    pub fn mark_requested(&mut self) {
        self.requested = true;
    }
}
