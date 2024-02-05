use crate::graphics::vulkan::raii;

/// A block of DeviceMemory.
///
/// A block can refer to a subsection of device memory, or to the entire
/// original memory allocation.
pub struct Block {
    /// The mapped_ptr exists IFF the memory is HOST_VISIBLE. The pointer
    /// always points to the beginning of the block.
    pub mapped_ptr: *mut std::ffi::c_void,
    pub memory: raii::DeviceMemoryArc,
    pub size_in_bytes: u64,
}

unsafe impl Send for Block {}
unsafe impl Sync for Block {}
