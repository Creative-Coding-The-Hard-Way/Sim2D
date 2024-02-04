use {
    crate::{
        graphics::vulkan::memory::{Allocator, Block},
        trace,
    },
    anyhow::{Context, Result},
    ash::vk,
    std::sync::{Arc, Mutex},
};

/// A block of memory which automatically frees itself when dropped.
pub struct OwnedBlock {
    block: Block,
    allocator: SharedAllocator,
}

/// An allocator with an immutable public interface that can be cloned and
/// safely shared between threads.
#[derive(Clone)]
pub struct SharedAllocator {
    allocator: Arc<Mutex<dyn Allocator + Send>>,
    name: String,
}

impl SharedAllocator {
    pub fn new<A>(allocator: A) -> Self
    where
        A: Allocator + Send + 'static,
    {
        Self {
            allocator: Arc::new(Mutex::new(allocator)),
            name: std::any::type_name::<A>().to_string(),
        }
    }

    /// Allocate a block of memory.
    pub fn allocate(
        &self,
        memory_requirements: vk::MemoryRequirements,
        property_flags: vk::MemoryPropertyFlags,
        memory_allocate_flags: vk::MemoryAllocateFlags,
    ) -> Result<OwnedBlock> {
        let block = self.allocator
            .lock()
            .expect("Allocator mutex should not be poisoned while allocating memory!")
            .allocate(
                memory_requirements,
                property_flags,
                memory_allocate_flags,
            ).with_context(trace!("Unable to allocate memory!"))?;
        Ok(OwnedBlock {
            block,
            allocator: self.clone(),
        })
    }

    /// Free a block of memory.
    fn free(&self, block: &Block) {
        self.allocator
            .lock()
            .expect(
                "Allocator mutex should not be poisoned when freeing memory!",
            )
            .free(block)
    }
}

impl std::ops::Deref for OwnedBlock {
    type Target = Block;

    fn deref(&self) -> &Self::Target {
        &self.block
    }
}

impl Drop for OwnedBlock {
    fn drop(&mut self) {
        self.allocator.free(&self.block);
    }
}

impl std::fmt::Debug for SharedAllocator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SharedAllocator")
            .field("allocator", &self.name)
            .finish()
    }
}
