use ash::vk;

/// Returns true when request is true but available is false.
fn is_requested_and_not_available(requested: u32, available: u32) -> bool {
    requested == vk::TRUE && available == vk::FALSE
}

#[rustfmt::skip]
pub(super) fn are_descriptor_indexing_features(
    requested: &vk::PhysicalDeviceDescriptorIndexingFeatures,
    available: &vk::PhysicalDeviceDescriptorIndexingFeatures,
) -> bool {
    macro_rules! check_support {
        ($property:ident) => {
            if is_requested_and_not_available(
                requested.$property,
                available.$property,
            ) {
                log::warn!(
                    "PhysicalDeviceDescriptorIndexingFeatures.{} is not supported by this device",
                    stringify!($property)
                );
                return false;
            }
        };
    }
    check_support!(shader_input_attachment_array_dynamic_indexing);
    check_support!(shader_uniform_texel_buffer_array_dynamic_indexing);
    check_support!(shader_storage_texel_buffer_array_dynamic_indexing);
    check_support!(shader_uniform_buffer_array_non_uniform_indexing);
    check_support!(shader_sampled_image_array_non_uniform_indexing);
    check_support!(shader_storage_buffer_array_non_uniform_indexing);
    check_support!(shader_storage_image_array_non_uniform_indexing);
    check_support!(shader_input_attachment_array_non_uniform_indexing);
    check_support!(shader_uniform_texel_buffer_array_non_uniform_indexing);
    check_support!(shader_storage_texel_buffer_array_non_uniform_indexing);
    check_support!(descriptor_binding_uniform_buffer_update_after_bind);
    check_support!(descriptor_binding_sampled_image_update_after_bind);
    check_support!(descriptor_binding_storage_image_update_after_bind);
    check_support!(descriptor_binding_storage_buffer_update_after_bind);
    check_support!(descriptor_binding_uniform_texel_buffer_update_after_bind);
    check_support!(descriptor_binding_storage_texel_buffer_update_after_bind);
    check_support!(descriptor_binding_update_unused_while_pending);
    check_support!(descriptor_binding_partially_bound);
    check_support!(descriptor_binding_variable_descriptor_count);
    check_support!(runtime_descriptor_array);
    true
}

#[rustfmt::skip]
pub(super) fn are_physical_device_vulkan_13_features_supported(
    requested: &vk::PhysicalDeviceVulkan13Features,
    available: &vk::PhysicalDeviceVulkan13Features,
) -> bool {
    macro_rules! check_support {
        ($property:ident) => {
            if is_requested_and_not_available(
                requested.$property,
                available.$property,
            ) {
                log::warn!(
                    "PhysicalDeviceVulkan13Features.{} is not supported by this device",
                    stringify!($property)
                );
                return false;
            }
        };
    }
    check_support!(robust_image_access);
    check_support!(inline_uniform_block);
    check_support!(descriptor_binding_inline_uniform_block_update_after_bind);
    check_support!(pipeline_creation_cache_control);
    check_support!(private_data);
    check_support!(shader_demote_to_helper_invocation);
    check_support!(shader_terminate_invocation);
    check_support!(subgroup_size_control);
    check_support!(compute_full_subgroups);
    check_support!(synchronization2);
    check_support!(texture_compression_astc_hdr);
    check_support!(shader_zero_initialize_workgroup_memory);
    check_support!(dynamic_rendering);
    check_support!(shader_integer_dot_product);
    check_support!(maintenance4);
    true
}

#[rustfmt::skip]
pub(super) fn are_physical_device_features_supported(
    requested: &vk::PhysicalDeviceFeatures,
    available: &vk::PhysicalDeviceFeatures,
) -> bool {
    macro_rules! check_support {
        ($property:ident) => {
            if is_requested_and_not_available(
                requested.$property,
                available.$property,
            ) {
                log::warn!(
                    "PhysicalDeviceFeatures.{} is not supported by this device",
                    stringify!($property)
                );
                return false;
            }
        };
    }
    check_support!(robust_buffer_access);
    check_support!(robust_buffer_access);
    check_support!(full_draw_index_uint32);
    check_support!(image_cube_array);
    check_support!(independent_blend);
    check_support!(geometry_shader);
    check_support!(tessellation_shader);
    check_support!(sample_rate_shading);
    check_support!(dual_src_blend);
    check_support!(logic_op);
    check_support!(multi_draw_indirect);
    check_support!(draw_indirect_first_instance);
    check_support!(depth_clamp);
    check_support!(depth_bias_clamp);
    check_support!(fill_mode_non_solid);
    check_support!(depth_bounds);
    check_support!(wide_lines);
    check_support!(large_points);
    check_support!(alpha_to_one);
    check_support!(multi_viewport);
    check_support!(sampler_anisotropy);
    check_support!(texture_compression_etc2);
    check_support!(texture_compression_astc_ldr);
    check_support!(texture_compression_bc);
    check_support!(occlusion_query_precise);
    check_support!(pipeline_statistics_query);
    check_support!(vertex_pipeline_stores_and_atomics);
    check_support!(fragment_stores_and_atomics);
    check_support!(shader_tessellation_and_geometry_point_size);
    check_support!(shader_image_gather_extended);
    check_support!(shader_storage_image_extended_formats);
    check_support!(shader_storage_image_multisample);
    check_support!(shader_storage_image_read_without_format);
    check_support!(shader_storage_image_write_without_format);
    check_support!(shader_uniform_buffer_array_dynamic_indexing);
    check_support!(shader_sampled_image_array_dynamic_indexing);
    check_support!(shader_storage_buffer_array_dynamic_indexing);
    check_support!(shader_storage_image_array_dynamic_indexing);
    check_support!(shader_clip_distance);
    check_support!(shader_cull_distance);
    check_support!(shader_float64);
    check_support!(shader_int64);
    check_support!(shader_int16);
    check_support!(shader_resource_residency);
    check_support!(shader_resource_min_lod);
    check_support!(sparse_binding);
    check_support!(sparse_residency_buffer);
    check_support!(sparse_residency_image2_d);
    check_support!(sparse_residency_image3_d);
    check_support!(sparse_residency2_samples);
    check_support!(sparse_residency4_samples);
    check_support!(sparse_residency8_samples);
    check_support!(sparse_residency16_samples);
    check_support!(sparse_residency_aliased);
    check_support!(variable_multisample_rate);
    check_support!(inherited_queries);
    true
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn verify_is_requested_and_not_supported() {
        assert!(is_requested_and_not_available(1, 0));
        assert!(is_requested_and_not_available(1, 1) == false);
        assert!(is_requested_and_not_available(0, 1) == false);
        assert!(is_requested_and_not_available(0, 0) == false);
    }
}
