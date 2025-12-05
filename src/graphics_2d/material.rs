use {
    crate::Gfx,
    anyhow::{Context, Result},
    ash::vk,
    demo_vk::graphics::vulkan::raii,
    std::ffi::CStr,
};

/// The shader entrypoint name, always defaults to 'main'.
const SHADER_ENTRYPOINT: &CStr = c"main";

/// Materials are used to style mesh properties.
///
/// Materials are immutable and can be shared by meshes.
#[derive(Debug)]
pub struct Material {
    pipeline: raii::Pipeline,
}

impl Material {
    /// Builds a pipeline layout for use with Material pipelines.
    pub(super) fn create_pipeline_layout(
        gfx: &Gfx,
        texture_atlas_descriptor_set_layout: &raii::DescriptorSetLayout,
        frame_constants_descriptor_set_layout: &raii::DescriptorSetLayout,
    ) -> Result<raii::PipelineLayout> {
        let raw_descriptor_set_layouts = [
            texture_atlas_descriptor_set_layout.raw,
            frame_constants_descriptor_set_layout.raw,
        ];
        let push_constant_ranges = [vk::PushConstantRange {
            stage_flags: vk::ShaderStageFlags::VERTEX,
            offset: 0,
            size: 8 + 8 + 4,
        }];
        raii::PipelineLayout::new(
            "FirstTriangle",
            gfx.vulkan.device.clone(),
            &vk::PipelineLayoutCreateInfo {
                set_layout_count: raw_descriptor_set_layouts.len() as u32,
                p_set_layouts: raw_descriptor_set_layouts.as_ptr(),
                push_constant_range_count: push_constant_ranges.len() as u32,
                p_push_constant_ranges: push_constant_ranges.as_ptr(),
                ..Default::default()
            },
        )
    }

    /// Creates a new material for use when rendering meshes.
    pub(super) fn new(
        gfx: &Gfx,
        pipeline_layout: &raii::PipelineLayout,
        vertex_shader_module: &raii::ShaderModule,
        fragment_shader_module: &raii::ShaderModule,
    ) -> Result<Self> {
        let stages = [
            vk::PipelineShaderStageCreateInfo {
                stage: vk::ShaderStageFlags::VERTEX,
                module: vertex_shader_module.raw,
                p_name: SHADER_ENTRYPOINT.as_ptr(),
                ..Default::default()
            },
            vk::PipelineShaderStageCreateInfo {
                stage: vk::ShaderStageFlags::FRAGMENT,
                module: fragment_shader_module.raw,
                p_name: SHADER_ENTRYPOINT.as_ptr(),
                ..Default::default()
            },
        ];

        let vertex_input_state = vk::PipelineVertexInputStateCreateInfo {
            vertex_binding_description_count: 0,
            vertex_attribute_description_count: 0,
            ..Default::default()
        };
        let input_assembly_state = vk::PipelineInputAssemblyStateCreateInfo {
            topology: vk::PrimitiveTopology::TRIANGLE_LIST,
            primitive_restart_enable: vk::FALSE,
            ..Default::default()
        };
        let tesselation_state = vk::PipelineTessellationStateCreateInfo {
            patch_control_points: 0,
            ..Default::default()
        };
        let rasterization_state = vk::PipelineRasterizationStateCreateInfo {
            depth_clamp_enable: vk::FALSE,
            rasterizer_discard_enable: vk::FALSE,
            polygon_mode: vk::PolygonMode::FILL,
            cull_mode: vk::CullModeFlags::NONE,
            front_face: vk::FrontFace::COUNTER_CLOCKWISE,
            depth_bias_enable: vk::FALSE,
            depth_bias_constant_factor: 0.0,
            depth_bias_clamp: 0.0,
            depth_bias_slope_factor: 0.0,
            line_width: 1.0,
            ..Default::default()
        };
        let multisample_state = vk::PipelineMultisampleStateCreateInfo {
            rasterization_samples: vk::SampleCountFlags::TYPE_1,
            sample_shading_enable: vk::FALSE,
            min_sample_shading: 1.0,
            p_sample_mask: std::ptr::null(),
            alpha_to_coverage_enable: vk::FALSE,
            alpha_to_one_enable: vk::FALSE,
            ..Default::default()
        };
        let depth_stencil_state = vk::PipelineDepthStencilStateCreateInfo {
            depth_test_enable: vk::FALSE,
            depth_write_enable: vk::FALSE,
            depth_compare_op: vk::CompareOp::LESS,
            depth_bounds_test_enable: vk::FALSE,
            stencil_test_enable: vk::FALSE,
            min_depth_bounds: 0.0,
            max_depth_bounds: 1.0,
            ..Default::default()
        };
        let color_blend_statetachment_state =
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
        let color_blend_state = vk::PipelineColorBlendStateCreateInfo {
            logic_op_enable: vk::FALSE,
            logic_op: vk::LogicOp::COPY,
            attachment_count: 1,
            p_attachments: &color_blend_statetachment_state,
            blend_constants: [0.0, 0.0, 0.0, 0.0],
            ..Default::default()
        };

        let dynamic_states =
            [vk::DynamicState::VIEWPORT, vk::DynamicState::SCISSOR];
        let dynamic_state = vk::PipelineDynamicStateCreateInfo {
            dynamic_state_count: dynamic_states.len() as u32,
            p_dynamic_states: dynamic_states.as_ptr(),
            ..Default::default()
        };

        let viewport_state = vk::PipelineViewportStateCreateInfo {
            viewport_count: 1,
            scissor_count: 1,
            ..Default::default()
        };

        let color_attachment_formats = [gfx.swapchain.format()];
        let mut rendering_info = vk::PipelineRenderingCreateInfo {
            view_mask: 0,
            color_attachment_count: 1,
            p_color_attachment_formats: color_attachment_formats.as_ptr(),
            ..Default::default()
        };

        let create_info = vk::GraphicsPipelineCreateInfo {
            stage_count: stages.len() as u32,
            p_stages: stages.as_ptr(),
            p_vertex_input_state: &vertex_input_state,
            p_input_assembly_state: &input_assembly_state,
            p_tessellation_state: &tesselation_state,
            p_viewport_state: &viewport_state,
            p_rasterization_state: &rasterization_state,
            p_multisample_state: &multisample_state,
            p_depth_stencil_state: &depth_stencil_state,
            p_color_blend_state: &color_blend_state,
            p_dynamic_state: &dynamic_state,
            layout: pipeline_layout.raw,
            render_pass: vk::RenderPass::null(),
            subpass: 0,
            base_pipeline_handle: vk::Pipeline::null(),
            base_pipeline_index: 0,
            ..Default::default()
        }
        .push_next(&mut rendering_info);

        let pipeline = raii::Pipeline::new_graphics_pipeline(
            gfx.vulkan.device.clone(),
            &create_info,
        )
        .context("Unable to create pipeline!")?;

        Ok(Self { pipeline })
    }

    /// Returns the pipeline used by this material.
    pub fn pipeline(&self) -> &raii::Pipeline {
        &self.pipeline
    }
}
