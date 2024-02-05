use crate::graphics::vulkan::sync::UsedByFrames;

/// An NBuffer is a generalization of the concept of double buffering and
/// tripple buffering.
///
/// A N-Buffer is composed of N copies of a resource. When a resource is
/// current, it can be freely read by frames. When a resource is published, it
/// becomes current and the previously-current resource is marked as "in_use".
/// When all of the frames using an in_us resource have completed, then it
/// becomes "free" and can be used as a write target again.
pub struct NBuffer<T> {
    current: UsedByFrames<T>,
    free: Vec<T>,
    in_use: Vec<UsedByFrames<T>>,
}

impl<Resource> NBuffer<Resource> {
    /// Manage a collection of resources.
    ///
    /// # Ownership
    ///
    /// The NBuffer does not logically own any resources. The caller must keep
    /// track of the original set of resources and destroy them according to
    /// their own strategy.
    ///
    /// The N-Buffer is strictly for managing the bookeeping associated with
    /// preventing resources from being written while in-flight frames still
    /// reference them.
    pub fn new(mut resources: Vec<Resource>) -> Self {
        assert!(resources.len() >= 2);

        let current = UsedByFrames::new(resources.pop().unwrap());
        let free = resources;
        let in_use = Vec::with_capacity(free.len());

        Self {
            current,
            free,
            in_use,
        }
    }

    /// Get the current readable resource for the given frame index.
    pub fn get_current(&mut self, frame_index: usize) -> &mut Resource {
        self.current.frame_mask.add_frame(frame_index as u32);

        {
            let mut in_use_update = Vec::with_capacity(self.in_use.len());
            for mut in_use in self.in_use.drain(0..) {
                in_use.frame_mask.remove_frame(frame_index as u32);
                if in_use.frame_mask.is_empty() {
                    self.free.push(in_use.release());
                } else {
                    in_use_update.push(in_use);
                }
            }
            self.in_use = in_use_update;
        }

        &mut self.current.resource
    }

    /// Make the given resource current.
    pub fn make_current(&mut self, resource: Resource) {
        let old_current = {
            let mut new_value = UsedByFrames::new(resource);
            std::mem::swap(&mut self.current, &mut new_value);
            new_value
        };
        self.in_use.push(old_current);
    }

    /// Attempt to get a free resource. If all resources are in_use by frames,
    /// then this will return None.
    pub fn try_get_free_resource(&mut self) -> Option<Resource> {
        self.free.pop()
    }

    /// Clear the tracked resources so "try_get_free_resource" never returns
    /// a resource.
    pub fn destroy(&mut self) {
        self.free.clear();
        self.in_use.clear();
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[derive(Copy, Clone, Debug, Default, Eq, PartialEq, PartialOrd, Ord)]
    struct FakeResource(u32);

    #[test]
    fn create_n_buffered_resources() {
        let resources = (0..3).map(FakeResource).collect::<Vec<_>>();
        let _n_buffered = NBuffer::new(resources);
    }

    #[test]
    fn get_current_should_update_the_current_frame_mask() {
        let resources = (0..3).map(FakeResource).collect::<Vec<_>>();
        let mut n_buffered = NBuffer::new(resources);

        let fake_resource = n_buffered.get_current(1);
        assert_eq!(*fake_resource, FakeResource(2));
        assert!(n_buffered.current.frame_mask.contains(1));
    }

    #[test]
    fn make_current_should_make_the_new_resource_current() {
        let resources = (0..3).map(FakeResource).collect::<Vec<_>>();
        let mut n_buffered = NBuffer::new(resources);

        let writable = n_buffered.try_get_free_resource().unwrap();
        let initially_current = n_buffered.get_current(1);
        assert_ne!(*initially_current, writable);

        n_buffered.make_current(writable);

        assert!(n_buffered.in_use.len() == 1);
        assert!(n_buffered.in_use[0].frame_mask.contains(1));

        n_buffered.get_current(2);
        assert!(n_buffered.current.frame_mask.contains(2));
    }

    #[test]
    fn get_free_should_return_none_when_all_resources_are_in_use() {
        let resources = vec![FakeResource(0), FakeResource(1)];
        let mut double_buffered = NBuffer::new(resources);

        // Frame 0
        {
            let frame0_update =
                double_buffered.try_get_free_resource().unwrap();
            assert!(frame0_update == FakeResource(0));

            // update frame 0 in some way
            double_buffered.make_current(frame0_update);
            // "draw"
            assert!(*double_buffered.get_current(0) == frame0_update);
            assert!(double_buffered.in_use.is_empty());
        }

        // Frame 1
        {
            let frame1_update =
                double_buffered.try_get_free_resource().unwrap();
            assert!(frame1_update == FakeResource(1));

            // update frame 1 in some way
            double_buffered.make_current(frame1_update);
            // "draw"
            assert!(*double_buffered.get_current(1) == frame1_update);
            assert!(double_buffered.in_use.len() == 1);
        }

        // Frame 0
        {
            assert!(double_buffered.try_get_free_resource().is_none());
            // "draw"
            assert!(*double_buffered.get_current(0) == FakeResource(1));
            assert!(double_buffered.in_use.is_empty());
        }

        // Frame 1
        {
            let frame1_update =
                double_buffered.try_get_free_resource().unwrap();
            assert!(frame1_update == FakeResource(0));

            // update frame 1 in some way
            double_buffered.make_current(frame1_update);
            // "draw"
            assert!(*double_buffered.get_current(1) == frame1_update);
            assert!(double_buffered.in_use.len() == 1);
        }

        // Frame 0
        {
            assert!(double_buffered.try_get_free_resource().is_none());
            // "draw"
            assert!(*double_buffered.get_current(0) == FakeResource(0));
            assert!(double_buffered.in_use.is_empty());
        }
    }
}
