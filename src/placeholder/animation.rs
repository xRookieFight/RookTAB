use crate::config::AnimationConfig;

use super::{Placeholder, PlaceholderContext};

pub struct AnimationPlaceholder {
    interval_ticks: u64,
    frames: Vec<String>,
}

impl AnimationPlaceholder {
    pub fn new(config: &AnimationConfig) -> Self {
        Self {
            interval_ticks: config.interval_ticks(),
            frames: config.frames().to_vec(),
        }
    }

    fn frame_at(&self, tick: u64) -> Option<&String> {
        if self.frames.is_empty() {
            return None;
        }

        let index = (tick / self.interval_ticks) % self.frames.len() as u64;
        self.frames.get(index as usize)
    }
}

impl Placeholder for AnimationPlaceholder {
    fn resolve(&self, context: &PlaceholderContext<'_>) -> Option<String> {
        self.frame_at(context.tick()).cloned()
    }
}
