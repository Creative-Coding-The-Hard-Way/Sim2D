mod device_allocator;
mod stats_collector;

use {crate::graphics::vulkan::memory::Block, anyhow::Result, ash::vk};

pub use self::{
    device_allocator::DeviceAllocator, stats_collector::StatsCollector,
};

/// Types which implement this trait can allocate and free blocks of memory.
pub trait Allocator {
    /// Allocate a block of memory.
    fn allocate(
        &mut self,
        memory_requirements: vk::MemoryRequirements,
        property_flags: vk::MemoryPropertyFlags,
        memory_allocate_flags: vk::MemoryAllocateFlags,
    ) -> Result<Block>;

    /// Free a block of memory.
    fn free(&mut self, block: &Block);
}
