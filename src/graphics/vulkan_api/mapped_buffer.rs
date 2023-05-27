use {
    crate::graphics::{
        vulkan_api::{raii, RenderDevice},
        GraphicsError,
    },
    ash::vk,
    std::{marker::PhantomData, os::raw::c_void, sync::Arc},
};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum WriteStatus {
    Complete,
    CompleteWithReallocation,
}

/// A typed host-accessible Vulkan buffer.
pub struct MappedBuffer<T: Copy> {
    buffer: raii::Buffer,
    host_ptr: *mut c_void,
    element_count: usize,
    usage: vk::BufferUsageFlags,
    render_device: Arc<RenderDevice>,
    _phantom_data: PhantomData<T>,
}

impl<T: Copy> MappedBuffer<T> {
    /// Create a new CPU mappped buffer.
    ///
    /// # Safety
    ///
    /// Unsafe because:
    ///   - The application must drop this buffer before the RenderDevice.
    ///   - The application must not write to the buffer while the GPU is
    ///     reading.
    ///   - The application must not read from the buffer while the GPU is
    ///     updating it.
    pub unsafe fn new(
        render_device: Arc<RenderDevice>,
        capacity: usize,
        usage: vk::BufferUsageFlags,
    ) -> Result<Self, GraphicsError> {
        let (buffer, host_ptr) = Self::allocate_mapped_buffer(
            render_device.clone(),
            capacity,
            usage,
        )?;
        Ok(Self {
            buffer,
            host_ptr,
            element_count: 0,
            usage,
            render_device,
            _phantom_data: PhantomData,
        })
    }

    /// Write data into the buffer.
    ///
    /// # Safety
    ///
    /// Unsafe because:
    ///   - If the buffer does not have capacity for the data, it will attempt
    ///     to reallocate the underlying buffer and memory with enough size for
    ///     the data.
    ///   - Any references to the old buffer (descriptor sets, etc...) must be
    ///     updated after a write which concludes with
    ///     WriteStatus::CompleteWithReallocation.
    ///   - The application must synchronize access to the buffer so that there
    ///     are no races between the CPU and GPU.
    pub unsafe fn write(
        &mut self,
        data: &[T],
    ) -> Result<WriteStatus, GraphicsError> {
        let mut write_status = WriteStatus::Complete;
        if self.capacity_in_bytes() < std::mem::size_of_val(data) as u64 {
            let (buffer, host_ptr) = Self::allocate_mapped_buffer(
                self.render_device.clone(),
                data.len(),
                self.usage,
            )?;
            self.buffer = buffer;
            self.host_ptr = host_ptr;
            write_status = WriteStatus::CompleteWithReallocation;
        }

        // Memcpy the data into the buffer.
        //
        // NOTE: both pointers are cast to u8's to avoid any question of pointer
        // alignment. This is basically std::ptr::write_unaligned is
        // implemented, but that function doesn't support copying a full
        // slice of data. https://rust-lang.github.io/rfcs/1725-unaligned-access.html
        //
        // This only works if both the CPU and the shader agree on the size and
        // alignment of items in the buffer, otherwise padding between elements
        // will mess up the stride.
        std::ptr::copy_nonoverlapping(
            data.as_ptr() as *const u8,
            self.host_ptr as *mut u8,
            Self::size_in_bytes(data.len()) as usize,
        );

        self.element_count = data.len();

        Ok(write_status)
    }

    /// How many elements are currently saved in the buffer. The value is
    /// based on the last write.
    pub fn count(&self) -> usize {
        self.element_count
    }

    /// Get the current length of the buffer based on the element count.
    #[allow(dead_code)]
    pub fn current_size_in_bytes(&self) -> u64 {
        Self::size_in_bytes(self.element_count)
    }

    /// Get the current maximum capacity for this buffer. This can change if
    /// the buffer is reallocated in a call to write().
    pub fn capacity_in_bytes(&self) -> u64 {
        self.buffer.allocation().size_in_bytes()
    }

    /// Get the raw Vulkan buffer handle.
    pub fn raw(&self) -> vk::Buffer {
        self.buffer.raw()
    }
}

impl<T: Copy> MappedBuffer<T> {
    unsafe fn allocate_mapped_buffer(
        render_device: Arc<RenderDevice>,
        capacity: usize,
        usage: vk::BufferUsageFlags,
    ) -> Result<(raii::Buffer, *mut c_void), GraphicsError> {
        let queue_family_index = render_device.graphics_queue().family_index();
        let create_info = vk::BufferCreateInfo {
            size: Self::size_in_bytes(capacity),
            usage,
            flags: vk::BufferCreateFlags::empty(),
            sharing_mode: vk::SharingMode::EXCLUSIVE,
            p_queue_family_indices: &queue_family_index,
            queue_family_index_count: 1,
            ..Default::default()
        };
        let buffer = raii::Buffer::new(
            render_device.clone(),
            &create_info,
            vk::MemoryPropertyFlags::HOST_VISIBLE
                | vk::MemoryPropertyFlags::HOST_COHERENT,
        )?;
        let host_ptr = buffer.allocation().map(render_device.device())?;
        Ok((buffer, host_ptr))
    }

    fn size_in_bytes(count: usize) -> u64 {
        (std::mem::size_of::<T>() * count) as u64
    }
}
