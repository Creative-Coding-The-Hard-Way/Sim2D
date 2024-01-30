use {
    anyhow::{Context, Result},
    ash::vk,
    sim2d::graphics::vulkan::render_context::RenderContext,
    std::ffi::CString,
};

#[derive(Debug, Copy, Clone, Default)]
#[repr(packed)]
pub struct Vertex {
    pub rgba: [f32; 4],
    pub pos: [f32; 2],
    pub pad: [f32; 2],
}

/// Store bytes in a newtype aligned to 32 bytes.
///
/// This means we can always count on the included bytes being properly aligned.
#[repr(C, align(32))]
struct U32AlignedShaderSource<Bytes: ?Sized> {
    pub data: Bytes,
}

impl U32AlignedShaderSource<[u8]> {
    /// # Safety
    ///
    /// Unsafe because:
    /// - It's only safe to use this method for static instances.
    pub unsafe fn get_create_info(&self) -> vk::ShaderModuleCreateInfo {
        vk::ShaderModuleCreateInfo {
            code_size: self.data.len(),
            p_code: self.data.as_ptr() as *const u32,
            ..Default::default()
        }
    }
}

static FRAGMENT: &U32AlignedShaderSource<[u8]> = &U32AlignedShaderSource {
    data: *include_bytes!("shaders/triangle.frag.spv"),
};
static VERTEX: &U32AlignedShaderSource<[u8]> = &U32AlignedShaderSource {
    data: *include_bytes!("shaders/triangle.vert.spv"),
};

pub struct GraphicsPipeline {
    pub handle: vk::Pipeline,
    pub pipeline_layout: vk::PipelineLayout,
    pub descriptor_set_layout: vk::DescriptorSetLayout,
    pub descriptor_pool: vk::DescriptorPool,
    pub descriptor_set: vk::DescriptorSet,
    pub storage_buffer: vk::Buffer,
    pub buffer_memory: vk::DeviceMemory,
}

impl GraphicsPipeline {
    /// Create a new graphics pipeline.
    pub fn new(
        rc: &RenderContext,
        render_pass: &vk::RenderPass,
    ) -> Result<Self> {
        // Create the descriptor set layout
        let descriptor_set_layout = {
            let binding = vk::DescriptorSetLayoutBinding {
                binding: 0,
                descriptor_type: vk::DescriptorType::STORAGE_BUFFER,
                descriptor_count: 1,
                stage_flags: vk::ShaderStageFlags::VERTEX,
                p_immutable_samplers: std::ptr::null(),
            };
            let create_info = vk::DescriptorSetLayoutCreateInfo {
                binding_count: 1,
                p_bindings: &binding,
                ..Default::default()
            };
            unsafe {
                rc.device.create_descriptor_set_layout(&create_info, None)?
            }
        };

        let descriptor_pool = {
            let pool_sizes = [vk::DescriptorPoolSize {
                ty: vk::DescriptorType::STORAGE_BUFFER,
                descriptor_count: 1,
            }];
            let create_info = vk::DescriptorPoolCreateInfo {
                max_sets: 1,
                pool_size_count: pool_sizes.len() as u32,
                p_pool_sizes: pool_sizes.as_ptr(),
                ..Default::default()
            };
            unsafe { rc.device.create_descriptor_pool(&create_info, None)? }
        };

        let descriptor_set = {
            let allocate_info = vk::DescriptorSetAllocateInfo {
                descriptor_pool,
                descriptor_set_count: 1,
                p_set_layouts: &descriptor_set_layout,
                ..Default::default()
            };
            unsafe { rc.device.allocate_descriptor_sets(&allocate_info)?[0] }
        };

        let memory_size = (std::mem::size_of::<Vertex>() * 3) as u64;

        // create the storage buffer
        let storage_buffer = {
            let create_info = vk::BufferCreateInfo {
                size: memory_size,
                usage: vk::BufferUsageFlags::STORAGE_BUFFER,
                sharing_mode: vk::SharingMode::EXCLUSIVE,
                queue_family_index_count: 1,
                p_queue_family_indices: &rc.graphics_queue_index,
                ..Default::default()
            };
            unsafe { rc.device.create_buffer(&create_info, None)? }
        };

        // Allocate memory for the buffer
        let buffer_memory = {
            let memory_requirements = unsafe {
                rc.device.get_buffer_memory_requirements(storage_buffer)
            };
            let memory_properties = unsafe {
                rc.instance
                    .ash
                    .get_physical_device_memory_properties(rc.physical_device)
            };
            let memory_type_index = memory_properties
                .memory_types
                .iter()
                .enumerate()
                .find(|(index, &memory_type)| {
                    let type_bits = 1 << *index;
                    let is_required_type =
                        type_bits & memory_requirements.memory_type_bits != 0;
                    let has_required_properties =
                        memory_type.property_flags.contains(
                            vk::MemoryPropertyFlags::HOST_COHERENT
                                | vk::MemoryPropertyFlags::HOST_VISIBLE,
                        );
                    is_required_type && has_required_properties
                })
                .map(|(index, _memory_type)| index)
                .context("Unable to get suitable memory type")?;
            let allocate_info = vk::MemoryAllocateInfo {
                allocation_size: memory_size,
                memory_type_index: memory_type_index as u32,
                ..Default::default()
            };
            unsafe { rc.device.allocate_memory(&allocate_info, None)? }
        };

        // Bind the memory to the buffer
        let mapped_buffer_ptr = {
            unsafe {
                rc.device.bind_buffer_memory(
                    storage_buffer,
                    buffer_memory,
                    0,
                )?;
            }
            unsafe {
                rc.device.map_memory(
                    buffer_memory,
                    0,
                    memory_size,
                    vk::MemoryMapFlags::empty(),
                )?
            }
        };

        // Write some data into the buffer
        unsafe {
            let slice = std::slice::from_raw_parts_mut(
                mapped_buffer_ptr as *mut Vertex,
                memory_size as usize / std::mem::size_of::<Vertex>(),
            );
            slice[0].rgba = [1.0, 0.0, 0.0, 1.0];
            slice[0].pos = [0.0, -0.5];

            slice[1].rgba = [0.0, 1.0, 0.0, 1.0];
            slice[1].pos = [0.5, 0.5];

            slice[2].rgba = [0.0, 0.0, 1.0, 1.0];
            slice[2].pos = [-0.5, 0.5];
        }

        // Update the descriptor set to refer to the buffer
        {
            let buffer_info = vk::DescriptorBufferInfo {
                buffer: storage_buffer,
                offset: 0,
                range: vk::WHOLE_SIZE,
            };
            let descriptor_writes = [vk::WriteDescriptorSet {
                dst_set: descriptor_set,
                dst_binding: 0,
                descriptor_count: 1,
                descriptor_type: vk::DescriptorType::STORAGE_BUFFER,
                p_buffer_info: &buffer_info,
                ..Default::default()
            }];
            unsafe {
                rc.device.update_descriptor_sets(&descriptor_writes, &[]);
            }
        }

        // create the pipeline layout
        let pipeline_layout = {
            let create_info = vk::PipelineLayoutCreateInfo {
                push_constant_range_count: 0,
                set_layout_count: 1,
                p_set_layouts: &descriptor_set_layout,
                ..Default::default()
            };
            unsafe { rc.device.create_pipeline_layout(&create_info, None)? }
        };

        // Create the shader modules
        let vertex_shader = {
            unsafe {
                rc.device
                    .create_shader_module(&VERTEX.get_create_info(), None)?
            }
        };
        let fragment_shader = {
            unsafe {
                rc.device
                    .create_shader_module(&FRAGMENT.get_create_info(), None)?
            }
        };

        // Assign shader modules to appropriate stages
        let entrypoint = CString::new("main").unwrap();
        let shader_stages = [
            vk::PipelineShaderStageCreateInfo {
                stage: vk::ShaderStageFlags::VERTEX,
                module: vertex_shader,
                p_name: entrypoint.as_ptr(),
                ..Default::default()
            },
            vk::PipelineShaderStageCreateInfo {
                stage: vk::ShaderStageFlags::FRAGMENT,
                module: fragment_shader,
                p_name: entrypoint.as_ptr(),
                ..Default::default()
            },
        ];

        // Setup dynamic states
        let dynamic_states =
            [vk::DynamicState::VIEWPORT, vk::DynamicState::SCISSOR];
        let dynamic_state_create_info = vk::PipelineDynamicStateCreateInfo {
            dynamic_state_count: dynamic_states.len() as u32,
            p_dynamic_states: dynamic_states.as_ptr(),
            ..Default::default()
        };

        // Vertex Input
        let vertex_input_create_info = vk::PipelineVertexInputStateCreateInfo {
            vertex_binding_description_count: 0,
            vertex_attribute_description_count: 0,
            ..Default::default()
        };

        // Input Assembly
        let input_assembly_create_info =
            vk::PipelineInputAssemblyStateCreateInfo {
                topology: vk::PrimitiveTopology::TRIANGLE_LIST,
                primitive_restart_enable: vk::FALSE,
                ..Default::default()
            };

        // Viewport setup
        let viewport_state_create_info = vk::PipelineViewportStateCreateInfo {
            viewport_count: 1,
            scissor_count: 1,
            ..Default::default()
        };

        // Rasterizer Setup
        let rasterizer_create_info = vk::PipelineRasterizationStateCreateInfo {
            depth_clamp_enable: vk::FALSE,
            rasterizer_discard_enable: vk::FALSE,
            polygon_mode: vk::PolygonMode::FILL,
            cull_mode: vk::CullModeFlags::NONE,
            front_face: vk::FrontFace::CLOCKWISE,
            depth_bias_enable: vk::FALSE,
            line_width: 1.0,
            ..Default::default()
        };

        // Multisampling Setup
        let multisample_create_info = vk::PipelineMultisampleStateCreateInfo {
            rasterization_samples: vk::SampleCountFlags::TYPE_1,
            sample_shading_enable: vk::FALSE,
            ..Default::default()
        };

        // Bendling setup
        let color_blend_attachment_state =
            vk::PipelineColorBlendAttachmentState {
                blend_enable: vk::TRUE,
                src_color_blend_factor: vk::BlendFactor::SRC_ALPHA,
                dst_color_blend_factor: vk::BlendFactor::ONE_MINUS_SRC_ALPHA,
                color_blend_op: vk::BlendOp::ADD,
                src_alpha_blend_factor: vk::BlendFactor::ONE,
                dst_alpha_blend_factor: vk::BlendFactor::ZERO,
                alpha_blend_op: vk::BlendOp::ADD,
                color_write_mask: vk::ColorComponentFlags::RGBA,
            };
        let blend_state_create_info = vk::PipelineColorBlendStateCreateInfo {
            logic_op_enable: vk::FALSE,
            attachment_count: 1,
            p_attachments: &color_blend_attachment_state,
            ..Default::default()
        };

        // Graphics Pipeline create info
        let create_info = vk::GraphicsPipelineCreateInfo {
            stage_count: shader_stages.len() as u32,
            p_stages: shader_stages.as_ptr(),
            p_vertex_input_state: &vertex_input_create_info,
            p_input_assembly_state: &input_assembly_create_info,
            p_viewport_state: &viewport_state_create_info,
            p_rasterization_state: &rasterizer_create_info,
            p_multisample_state: &multisample_create_info,
            p_color_blend_state: &blend_state_create_info,
            p_dynamic_state: &dynamic_state_create_info,
            layout: pipeline_layout,
            render_pass: *render_pass,
            subpass: 0,
            ..Default::default()
        };

        let handle = unsafe {
            match rc.device.create_graphics_pipelines(
                vk::PipelineCache::null(),
                &[create_info],
                None,
            ) {
                Ok(pipelines) => pipelines[0],
                Err((pipelines, err)) => {
                    err.result().with_context(|| {
                        "Unable to create graphics pipeline!"
                    })?;
                    pipelines[0]
                }
            }
        };

        // Cleanup
        unsafe {
            rc.device.destroy_shader_module(vertex_shader, None);
            rc.device.destroy_shader_module(fragment_shader, None);
        }
        Ok(Self {
            handle,
            pipeline_layout,
            descriptor_set_layout,
            descriptor_pool,
            descriptor_set,
            storage_buffer,
            buffer_memory,
        })
    }

    /// Destroy the graphics pipeline.
    ///
    /// # Safety
    ///
    /// Unsafe because:
    /// - The graphics pipeline must not be in-use by the GPU when it is
    ///   destroyed.
    /// - The graphics pipeline must not be used after being destroyed.
    /// - destroy() should only be called once, even if there are many clones of
    ///   the pipeline.
    pub unsafe fn destroy(&mut self, rc: &RenderContext) {
        rc.device.destroy_buffer(self.storage_buffer, None);
        rc.device.free_memory(self.buffer_memory, None);
        rc.device.destroy_pipeline(self.handle, None);
        rc.device
            .destroy_descriptor_pool(self.descriptor_pool, None);
        rc.device
            .destroy_descriptor_set_layout(self.descriptor_set_layout, None);
        rc.device
            .destroy_pipeline_layout(self.pipeline_layout, None)
    }
}
