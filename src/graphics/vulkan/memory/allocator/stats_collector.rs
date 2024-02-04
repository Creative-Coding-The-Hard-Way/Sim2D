use {
    crate::graphics::vulkan::memory::{Allocator, Block},
    ash::vk,
};

pub struct StatsCollector<A: Allocator> {
    description: String,
    total_allocations: u64,
    max_concurrent_allocations: u64,
    concurrent_allocations: u64,
    decorated: A,
}

impl<A: Allocator> StatsCollector<A> {
    pub fn new(description: impl Into<String>, allocator: A) -> Self {
        Self {
            description: description.into(),
            total_allocations: 0,
            max_concurrent_allocations: 0,
            concurrent_allocations: 0,
            decorated: allocator,
        }
    }
}

impl<A: Allocator> Allocator for StatsCollector<A> {
    fn allocate(
        &mut self,
        memory_requirements: vk::MemoryRequirements,
        property_flags: vk::MemoryPropertyFlags,
        memory_allocate_flags: vk::MemoryAllocateFlags,
    ) -> anyhow::Result<Block> {
        self.total_allocations += 1;
        self.concurrent_allocations += 1;
        self.max_concurrent_allocations = self
            .max_concurrent_allocations
            .max(self.concurrent_allocations);

        self.decorated.allocate(
            memory_requirements,
            property_flags,
            memory_allocate_flags,
        )
    }

    fn free(&mut self, block: &Block) {
        self.concurrent_allocations -= 1;
        self.decorated.free(block)
    }
}

impl<A: Allocator> Drop for StatsCollector<A> {
    fn drop(&mut self) {
        log::info!(
            indoc::indoc! {"
                Allocator StatsCollector Report

                {}

                # Stats

                total_allocations: {}
                current_allocations: {}
                max_concurrent_allocations: {}
            "},
            self.description,
            self.total_allocations,
            self.concurrent_allocations,
            self.max_concurrent_allocations
        );
    }
}
