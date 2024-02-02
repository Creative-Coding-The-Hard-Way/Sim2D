/// A bitmask used to indicate which frame or frames are currently in flight.
///
/// This is typically paired with a resource to keep track of it's usage in a
/// frame.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Ord, PartialOrd)]
pub struct FrameMask(u32);

impl FrameMask {
    /// Create a mask containing no frames.
    pub fn empty() -> Self {
        Self(0)
    }

    /// Create a mask for the given frame index. Must not be greater than 32.
    pub fn for_frame(frame_index: u32) -> Self {
        debug_assert!(frame_index < 32);
        Self(1 << frame_index)
    }

    /// Check if the mask contains the bit for the given frame index.
    pub fn contains(&self, frame_index: u32) -> bool {
        let other = Self::for_frame(frame_index);
        self.0 & other.0 != 0
    }

    /// Add a frame index to the mask.
    pub fn add_frame(&mut self, frame_index: u32) {
        let other = Self::for_frame(frame_index);
        self.0 |= other.0
    }

    /// Remove a frame index from the mask.
    pub fn remove_frame(&mut self, frame_index: u32) {
        let other = Self::for_frame(frame_index);
        self.0 &= !other.0;
    }

    /// Returns true when the frame mask is entirely empty.
    pub fn is_empty(&self) -> bool {
        self.0 == 0
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_create_empty() {
        for i in 0..32 {
            assert!(!FrameMask::empty().contains(i));
        }
    }

    #[test]
    fn test_for_frame() {
        for i in 0..32 {
            assert!(FrameMask::for_frame(i).contains(i));
            for j in 0..32 {
                if i == j {
                    continue;
                }
                assert!(!FrameMask::for_frame(i).contains(j));
                assert!(!FrameMask::for_frame(j).contains(i));
            }
        }
    }

    #[test]
    fn test_add_frame() {
        let mut frame = FrameMask::empty();
        for j in 0..32 {
            frame.add_frame(j);
        }
        for j in 0..32 {
            assert!(frame.contains(j));
        }
    }

    #[test]
    fn test_remove_frame() {
        let mut frame = FrameMask::empty();
        for j in 0..32 {
            frame.add_frame(j);
        }
        for j in 0..32 {
            assert!(frame.contains(j));
            frame.remove_frame(j);
            assert!(!frame.contains(j));
        }
        assert!(frame.is_empty());
    }
}
