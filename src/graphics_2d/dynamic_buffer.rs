use {
    crate::graphics_2d::utility::round_to_power_of_two,
    anyhow::{Context, Result},
    ash::vk,
    demo_vk::graphics::vulkan::{CPUBuffer, VulkanContext},
};

/// An automatically resizable CPU buffer that reallocates the underlying buffer
/// if needed.
pub struct DynamicBuffer<DataT: Copy> {
    usage: vk::BufferUsageFlags,
    cpu_buffer: CPUBuffer<DataT>,
    buffer_device_address: vk::DeviceAddress,
}

impl<DataT: Copy> DynamicBuffer<DataT> {
    pub fn new(
        ctx: &VulkanContext,
        initial_capacity: usize,
        usage: vk::BufferUsageFlags,
    ) -> Result<Self> {
        let cpu_buffer = CPUBuffer::allocate(
            ctx,
            round_to_power_of_two(initial_capacity),
            usage,
        )?;
        let buffer_device_address = unsafe {
            ctx.get_buffer_device_address(&vk::BufferDeviceAddressInfo {
                buffer: cpu_buffer.buffer(),
                ..Default::default()
            })
        };
        Ok(Self {
            usage,
            cpu_buffer,
            buffer_device_address,
        })
    }

    /// Returns the raw buffer handle.
    ///
    /// # Safety
    ///
    /// Note that the returned buffer handle can be invalidated by calls to
    /// write_data.
    pub fn raw(&self) -> vk::Buffer {
        self.cpu_buffer.buffer()
    }

    /// Returns the current buffer device address.
    pub fn buffer_device_address(&self) -> vk::DeviceAddress {
        self.buffer_device_address
    }

    /// Writes the provided data to the underlying buffer.
    ///
    /// # Safety
    ///
    /// The caller is required to synchronize access to the underlying buffer.
    /// Importantly: if [data] is longer than the underlying buffer, then the
    /// buffer will be reallocated with more memory.
    pub unsafe fn write_data(
        &mut self,
        ctx: &VulkanContext,
        data: &[&[DataT]],
    ) -> Result<bool> {
        let total_size = data.iter().map(|chunk| chunk.len()).sum();
        let reallocated = if self.cpu_buffer.capacity() < total_size {
            let new_size = round_to_power_of_two(total_size);
            log::info!(
                "reallocate buffer. Current size: {}, required size: {}, new size: {}",
                self.cpu_buffer.capacity(),
                total_size,
                new_size
            );
            self.cpu_buffer = CPUBuffer::allocate(ctx, new_size, self.usage)
                .context("Unable to reallocate new buffer!")?;
            self.buffer_device_address = unsafe {
                ctx.get_buffer_device_address(&vk::BufferDeviceAddressInfo {
                    buffer: self.cpu_buffer.buffer(),
                    ..Default::default()
                })
            };

            true // cpu buffer was reallocated
        } else {
            false // cpu buffer was not reallocated
        };

        let mut offset = 0;
        for chunk in data {
            unsafe {
                self.cpu_buffer.write_data(offset, chunk)?;
            }
            offset += chunk.len();
        }

        Ok(reallocated)
    }
}
