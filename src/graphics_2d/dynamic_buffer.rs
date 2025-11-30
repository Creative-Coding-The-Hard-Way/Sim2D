use {
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

/// Returns the smallest power of two greater than the provided value.
fn round_to_power_of_two(value: usize) -> usize {
    (value as f32).log2().ceil().exp2().round() as usize
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn power_of_two_should_round_up() {
        assert_eq!(round_to_power_of_two(1), 1);
        assert_eq!(round_to_power_of_two(2), 2);
        assert_eq!(round_to_power_of_two(3), 4);
        assert_eq!(round_to_power_of_two(6), 8);
        assert_eq!(round_to_power_of_two(9), 16);
        assert_eq!(round_to_power_of_two(20), 32);
        assert_eq!(round_to_power_of_two(50), 64);
        assert_eq!(round_to_power_of_two(93), 128);
        assert_eq!(round_to_power_of_two(200), 256);
        assert_eq!(round_to_power_of_two(500), 512);
        assert_eq!(round_to_power_of_two(10_000), 16384);
    }
}
