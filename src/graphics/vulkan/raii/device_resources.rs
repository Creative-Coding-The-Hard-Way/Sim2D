use {
    crate::{graphics::vulkan::raii, trace},
    anyhow::{Context, Result},
    ash::vk,
    std::sync::Arc,
};

macro_rules! device_resource_struct {
    (
        $name: ident,
        $arc_name: ident,
        $raw_type: ty,
        $destroy: ident
    ) => {
        /// RAII wrapper
        pub struct $name {
            pub raw: $raw_type,
            pub device: Arc<raii::Device>,
        }

        pub type $arc_name = Arc<$name>;

        impl std::fmt::Debug for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.debug_struct(stringify!($name))
                    .field("ext", &"<extension loader>")
                    .field("raw", &self.raw)
                    .field("device", &self.device)
                    .finish()
            }
        }

        impl std::ops::Deref for $name {
            type Target = $raw_type;

            fn deref(&self) -> &Self::Target {
                &self.raw
            }
        }

        impl Drop for $name {
            fn drop(&mut self) {
                unsafe { self.device.$destroy(self.raw, None) }
            }
        }
    };
}

macro_rules! device_resource {
    (
        $name: ident,
        $arc_name: ident,
        $raw_type: ty,
        $create_info_type: ty,
        $create: ident,
        $destroy: ident
    ) => {
        device_resource_struct!($name, $arc_name, $raw_type, $destroy);

        impl $name {
            /// Create an instance wrapped in an Arc<>
            pub fn new(
                device: Arc<raii::Device>,
                create_info: &$create_info_type,
            ) -> Result<Arc<Self>> {
                Ok(Arc::new(Self::new_single_owner(device, create_info)?))
            }

            /// Create a new instance
            pub fn new_single_owner(
                device: Arc<raii::Device>,
                create_info: &$create_info_type,
            ) -> Result<Self> {
                let raw = unsafe { device.$create(create_info, None)? };
                Ok(Self { raw, device })
            }
        }
    };
}

device_resource!(
    Buffer,
    BufferArc,
    vk::Buffer,
    vk::BufferCreateInfo,
    create_buffer,
    destroy_buffer
);

device_resource!(
    ImageView,
    ImageViewArc,
    vk::ImageView,
    vk::ImageViewCreateInfo,
    create_image_view,
    destroy_image_view
);

device_resource!(
    Semaphore,
    SemaphoreArc,
    vk::Semaphore,
    vk::SemaphoreCreateInfo,
    create_semaphore,
    destroy_semaphore
);

device_resource!(
    Fence,
    FenceArc,
    vk::Fence,
    vk::FenceCreateInfo,
    create_fence,
    destroy_fence
);

device_resource!(
    CommandPool,
    CommandPoolArc,
    vk::CommandPool,
    vk::CommandPoolCreateInfo,
    create_command_pool,
    destroy_command_pool
);

device_resource!(
    RenderPass,
    RenderPassArc,
    vk::RenderPass,
    vk::RenderPassCreateInfo,
    create_render_pass,
    destroy_render_pass
);

device_resource!(
    Framebuffer,
    FramebufferArc,
    vk::Framebuffer,
    vk::FramebufferCreateInfo,
    create_framebuffer,
    destroy_framebuffer
);

device_resource!(
    PipelineLayout,
    PipelineLayoutArc,
    vk::PipelineLayout,
    vk::PipelineLayoutCreateInfo,
    create_pipeline_layout,
    destroy_pipeline_layout
);

device_resource!(
    ShaderModule,
    ShaderModuleArc,
    vk::ShaderModule,
    vk::ShaderModuleCreateInfo,
    create_shader_module,
    destroy_shader_module
);

device_resource_struct!(Pipeline, PipelineArc, vk::Pipeline, destroy_pipeline);

impl Pipeline {
    pub fn new(
        device: raii::DeviceArc,
        raw_pipeline: vk::Pipeline,
    ) -> PipelineArc {
        Arc::new(Self::new_single_owner(device, raw_pipeline))
    }

    pub fn new_single_owner(
        device: raii::DeviceArc,
        raw_pipeline: vk::Pipeline,
    ) -> Pipeline {
        Self {
            raw: raw_pipeline,
            device,
        }
    }

    /// Create multiple graphics pipelines with a single call.
    ///
    /// Errors are handled safely and all pipelines are always cleaned up
    /// properly when an error is returned.
    pub fn create_graphics_pipelines(
        device: raii::DeviceArc,
        create_infos: &[vk::GraphicsPipelineCreateInfo],
    ) -> Result<Vec<PipelineArc>> {
        let result = unsafe {
            device.create_graphics_pipelines(
                vk::PipelineCache::null(),
                create_infos,
                None,
            )
        };
        let pipelines: Vec<PipelineArc> = {
            let raw_pipelines = match &result {
                Ok(raw_pipelines) => raw_pipelines,
                Err((raw_pipelines, _)) => raw_pipelines,
            };
            raw_pipelines
                .iter()
                .map(|pipeline| Self::new(device.clone(), *pipeline))
                .collect()
        };
        if let Err((_, error)) = result {
            error.result().with_context(trace!(
                "Error while creating graphics pipelines!"
            ))?;
        }
        Ok(pipelines)
    }
}

device_resource_struct!(
    DeviceMemory,
    DeviceMemoryArc,
    vk::DeviceMemory,
    free_memory
);

impl DeviceMemory {
    pub fn new(device: raii::DeviceArc, raw: vk::DeviceMemory) -> Arc<Self> {
        Arc::new(Self::new_single_owner(device, raw))
    }

    pub fn new_single_owner(
        device: raii::DeviceArc,
        raw: vk::DeviceMemory,
    ) -> Self {
        Self { raw, device }
    }
}
