mod vertex_buffer;

use {
    anyhow::{Context, Result},
    ash::vk,
    sim2d::graphics::vulkan::{
        memory::DeviceAllocator, render_context::RenderContext,
    },
    std::ffi::CString,
};

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
    data: *include_bytes!("../shaders/triangle.frag.spv"),
};
static VERTEX: &U32AlignedShaderSource<[u8]> = &U32AlignedShaderSource {
    data: *include_bytes!("../shaders/triangle.vert.spv"),
};

use vertex_buffer::Vertex;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct PushConstants {
    pub vertex_buffer_addr: vk::DeviceAddress,
}

pub struct GraphicsPipeline {
    pub handle: vk::Pipeline,
    pub pipeline_layout: vk::PipelineLayout,
    pub vertex_buffer: vertex_buffer::VertexBuffer,
}

impl GraphicsPipeline {
    /// Create a new graphics pipeline.
    pub fn new(
        rc: &RenderContext,
        allocator: &DeviceAllocator,
        render_pass: &vk::RenderPass,
    ) -> Result<Self> {
        // Create the vertex buffer
        let mut vertex_buffer =
            vertex_buffer::VertexBuffer::new(rc, allocator)?;

        // Write some data into the buffer
        unsafe {
            vertex_buffer.write_vertex_data(&[
                Vertex {
                    rgba: [1.0, 0.0, 0.0, 1.0],
                    pos: [0.0, -0.5],
                    ..Default::default()
                },
                Vertex {
                    rgba: [0.0, 1.0, 0.0, 1.0],
                    pos: [0.5, 0.5],
                    ..Default::default()
                },
                Vertex {
                    rgba: [0.0, 0.0, 1.0, 1.0],
                    pos: [-0.5, 0.5],
                    ..Default::default()
                },
            ]);
        }

        // create the pipeline layout
        let pipeline_layout = {
            let push_constant_range = vk::PushConstantRange {
                stage_flags: vk::ShaderStageFlags::VERTEX,
                offset: 0,
                size: std::mem::size_of::<PushConstants>() as u32,
            };
            let create_info = vk::PipelineLayoutCreateInfo {
                push_constant_range_count: 1,
                p_push_constant_ranges: &push_constant_range,
                set_layout_count: 0,
                p_set_layouts: std::ptr::null(),
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
            vertex_buffer,
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
        self.vertex_buffer.destroy(rc);
        rc.device.destroy_pipeline(self.handle, None);
        rc.device
            .destroy_pipeline_layout(self.pipeline_layout, None)
    }
}
