use {
    crate::{
        graphics::vulkan::{
            raii, render_context::RenderContext, U32AlignedShaderSource,
        },
        trace,
    },
    anyhow::{Context, Result},
    ash::vk,
    std::ffi::CString,
};

static FRAGMENT: &U32AlignedShaderSource<[u8]> = &U32AlignedShaderSource {
    data: *include_bytes!("shaders/passthrough.frag.spv"),
};
static VERTEX: &U32AlignedShaderSource<[u8]> = &U32AlignedShaderSource {
    data: *include_bytes!("shaders/interpolated_primitive.vert.spv"),
};

#[repr(C)]
#[derive(Copy, Clone)]
pub struct PushConstants {
    pub dt: f32,
    pub vertex_buffer_addr: vk::DeviceAddress,
}

pub struct GraphicsPipeline {
    pub pipeline: raii::PipelineArc,
    pub pipeline_layout: raii::PipelineLayoutArc,
    pub descriptor_set_layout: raii::DescriptorSetLayoutArc,
    pub topology: vk::PrimitiveTopology,
}

impl GraphicsPipeline {
    /// Create a new graphics pipeline.
    pub fn new(
        rc: &RenderContext,
        render_pass: &vk::RenderPass,
        topology: vk::PrimitiveTopology,
    ) -> Result<Self> {
        let descriptor_set_layout = {
            let binding = vk::DescriptorSetLayoutBinding {
                binding: 0,
                descriptor_type: vk::DescriptorType::UNIFORM_BUFFER,
                descriptor_count: 1,
                stage_flags: vk::ShaderStageFlags::VERTEX,
                p_immutable_samplers: std::ptr::null(),
            };
            let create_info = vk::DescriptorSetLayoutCreateInfo {
                binding_count: 1,
                p_bindings: &binding,
                ..Default::default()
            };
            raii::DescriptorSetLayout::new(rc.device.clone(), &create_info)?
        };
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
                set_layout_count: 1,
                p_set_layouts: &descriptor_set_layout.raw,
                ..Default::default()
            };
            raii::PipelineLayout::new(rc.device.clone(), &create_info)
                .with_context(trace!("Unable to create pipeline layout!"))?
        };

        // Create the shader modules
        let vertex_shader = {
            let create_info = vk::ShaderModuleCreateInfo {
                code_size: VERTEX.data.len(),
                p_code: VERTEX.data.as_ptr() as *const u32,
                ..Default::default()
            };
            raii::ShaderModule::new_single_owner(
                rc.device.clone(),
                &create_info,
            )
            .with_context(trace!("Unable to create vertex shader module!"))?
        };
        let fragment_shader = {
            let create_info = vk::ShaderModuleCreateInfo {
                code_size: FRAGMENT.data.len(),
                p_code: FRAGMENT.data.as_ptr() as *const u32,
                ..Default::default()
            };
            raii::ShaderModule::new_single_owner(
                rc.device.clone(),
                &create_info,
            )
            .with_context(trace!("Unable to create fragment shader module!"))?
        };

        // Assign shader modules to appropriate stages
        let entrypoint = CString::new("main").unwrap();
        let shader_stages = [
            vk::PipelineShaderStageCreateInfo {
                stage: vk::ShaderStageFlags::VERTEX,
                module: vertex_shader.raw,
                p_name: entrypoint.as_ptr(),
                ..Default::default()
            },
            vk::PipelineShaderStageCreateInfo {
                stage: vk::ShaderStageFlags::FRAGMENT,
                module: fragment_shader.raw,
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
                topology,
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
            layout: pipeline_layout.raw,
            render_pass: *render_pass,
            subpass: 0,
            ..Default::default()
        };

        let pipeline = raii::Pipeline::create_graphics_pipelines(
            rc.device.clone(),
            &[create_info],
        )
        .with_context(trace!("Unable to create the triangles pipeline!"))?
        .pop()
        .with_context(trace!("Expected exactly one graphics pipeline!"))?;

        Ok(Self {
            pipeline,
            pipeline_layout,
            descriptor_set_layout,
            topology,
        })
    }
}
