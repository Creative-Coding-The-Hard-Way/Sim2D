use crate::graphics::vulkan::sync::FrameMask;

/// Represents a resource that is being used by frames in flight.
pub struct UsedByFrames<T> {
    pub frame_mask: FrameMask,
    pub resource: T,
}

impl<T> UsedByFrames<T> {
    pub fn new(resource: T) -> Self {
        Self {
            resource,
            frame_mask: FrameMask::empty(),
        }
    }

    pub fn release(self) -> T {
        self.resource
    }
}
