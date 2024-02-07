mod allocator;
mod block;
mod shared_allocator;

use {
    self::allocator::{DeviceAllocator, StatsCollector},
    crate::graphics::vulkan::raii,
    ash::vk,
};

pub use self::{
    allocator::Allocator,
    block::Block,
    shared_allocator::{OwnedBlock, SharedAllocator},
};

pub const KB: u64 = 1024;
pub const MB: u64 = KB * 1024;
pub const GB: u64 = MB * 1024;

/// Create the system allocator which behaves reasonably well for this
/// application.
pub fn create_system_allocator(
    device: raii::DeviceArc,
    physical_device: vk::PhysicalDevice,
) -> SharedAllocator {
    let allocator = StatsCollector::new(
        "Raw system memory allocations using the Vulkan Device.",
        DeviceAllocator::new(device, physical_device),
    );
    SharedAllocator::new(allocator)
}