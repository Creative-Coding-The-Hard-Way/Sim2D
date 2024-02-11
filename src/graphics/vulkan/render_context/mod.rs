mod instance;
mod logical_device;
mod physical_device;
mod queue_families;

use {
    crate::{
        graphics::vulkan::{
            memory::{create_system_allocator, SharedAllocator},
            raii,
        },
        trace,
    },
    anyhow::Result,
    ash::vk,
};
pub use {
    anyhow::Context, instance::Instance,
    physical_device::PhysicalDeviceMetadata,
};

/// The Vulkan rendering context.
///
/// The context contains all of the core resources required by most Vulkan
/// graphics applications. This includes the logical device, the physical
/// device, the surface, and the relevant queue handles.
#[derive(Clone, Debug)]
pub struct RenderContext {
    pub surface: raii::SurfaceArc,
    pub physical_device: vk::PhysicalDevice,
    pub physical_device_metadata: PhysicalDeviceMetadata,
    pub graphics_queue: vk::Queue,
    pub graphics_queue_index: u32,
    pub present_queue: vk::Queue,
    pub present_queue_index: u32,
    pub device: raii::DeviceArc,
    pub instance: Instance,
    pub allocator: SharedAllocator,
}

impl RenderContext {
    /// Create a Vulkan instance and RenderContext for the given window.
    pub fn frow_glfw_window(window: &glfw::Window) -> Result<Self> {
        let instance = Instance::new(
            "Sim2D",
            &window
                .glfw
                .get_required_instance_extensions()
                .unwrap_or_default(),
        )
        .with_context(trace!(
            "Error while creating Vulkan instance from a GLFW window!"
        ))?;
        let surface =
            raii::Surface::from_glfw_window(instance.ash.clone(), window)
                .with_context(trace!(
                    "Error creating surface for glfw window!"
                ))?;
        Self::new(instance, surface)
    }

    /// Create a new RenderContext for this application.
    pub fn new(instance: Instance, surface: raii::SurfaceArc) -> Result<Self> {
        let (physical_device, physical_device_metadata) =
            physical_device::find_suitable_device(&instance, &surface)
                .with_context(trace!(
                    "Unable to find a suitable Physical Device"
                ))?;
        log::info!(
            "Chosen physical device: {}",
            physical_device_metadata.device_name()
        );
        let queue_families = queue_families::QueueFamilies::new(
            physical_device,
            &physical_device_metadata,
            &surface,
        )
        .with_context(trace!(
            "Unable to get suitable queue families for the Render Context!"
        ))?;
        let device = logical_device::create_logical_device(
            instance.ash.clone(),
            physical_device,
            &queue_families,
        )
        .with_context(trace!("Unable to create the logical device!"))?;
        let (graphics_queue, present_queue) =
            queue_families.get_queues_from_device(&device);
        let allocator =
            create_system_allocator(device.clone(), physical_device);
        Ok(Self {
            surface,
            instance,
            physical_device,
            physical_device_metadata,
            device,
            graphics_queue,
            graphics_queue_index: queue_families.graphics_family_index,
            present_queue,
            present_queue_index: queue_families.present_family_index,
            allocator,
        })
    }
}
