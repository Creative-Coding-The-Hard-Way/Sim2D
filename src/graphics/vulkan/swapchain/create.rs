use {
    crate::{
        graphics::vulkan::{raii, render_context::RenderContext},
        trace,
    },
    anyhow::{Context, Result},
    ash::vk,
};

/// Create a new swapchain.
pub(super) fn create_swapchain(
    rc: &RenderContext,
    framebuffer_size: (u32, u32),
    previous_swapchain: Option<vk::SwapchainKHR>,
) -> Result<(raii::SwapchainArc, vk::Extent2D, vk::SurfaceFormatKHR)> {
    let capabilities = unsafe {
        rc.surface.ext.get_physical_device_surface_capabilities(
            rc.physical_device,
            rc.surface.raw,
        )?
    };
    let extent = select_extent(&capabilities, framebuffer_size);
    let surface_format = select_surface_format(rc)?;
    let handle = {
        let image_count = select_image_count(&capabilities);
        let present_mode = select_present_mode(rc)?;
        let mut create_info = vk::SwapchainCreateInfoKHR {
            surface: rc.surface.raw,
            min_image_count: image_count,
            image_format: surface_format.format,
            image_color_space: surface_format.color_space,
            image_extent: extent,
            image_array_layers: 1,
            image_usage: vk::ImageUsageFlags::COLOR_ATTACHMENT,
            pre_transform: capabilities.current_transform,
            composite_alpha: vk::CompositeAlphaFlagsKHR::OPAQUE,
            present_mode,
            clipped: vk::TRUE,
            old_swapchain: previous_swapchain
                .unwrap_or(vk::SwapchainKHR::null()),
            ..Default::default()
        };
        let queue_family_indices =
            [rc.graphics_queue_index, rc.present_queue_index];
        if rc.graphics_queue_index == rc.present_queue_index {
            create_info.image_sharing_mode = vk::SharingMode::EXCLUSIVE;
            create_info.p_queue_family_indices = std::ptr::null();
            create_info.queue_family_index_count = 0;
        } else {
            create_info.image_sharing_mode = vk::SharingMode::CONCURRENT;
            create_info.p_queue_family_indices = queue_family_indices.as_ptr();
            create_info.queue_family_index_count =
                queue_family_indices.len() as u32;
        };
        raii::Swapchain::new(rc.device.clone(), &create_info)
            .with_context(trace!("Unable to create the swapchain!"))?
    };
    Ok((handle, extent, surface_format))
}

fn select_image_count(capabilities: &vk::SurfaceCapabilitiesKHR) -> u32 {
    let count = capabilities.min_image_count + 2;
    if capabilities.max_image_count > 0 {
        count.clamp(capabilities.min_image_count, capabilities.max_image_count)
    } else {
        count
    }
}

fn select_extent(
    capabilities: &vk::SurfaceCapabilitiesKHR,
    framebuffer_size: (u32, u32),
) -> vk::Extent2D {
    if capabilities.current_extent.width != std::u32::MAX {
        // Indicates that the current extent should be used
        return capabilities.current_extent;
    }

    let (desired_width, desired_height) = framebuffer_size;
    vk::Extent2D {
        width: desired_width.clamp(
            capabilities.max_image_extent.width,
            capabilities.min_image_extent.width,
        ),
        height: desired_height.clamp(
            capabilities.max_image_extent.height,
            capabilities.min_image_extent.height,
        ),
    }
}

fn select_present_mode(rc: &RenderContext) -> Result<vk::PresentModeKHR> {
    let present_modes = unsafe {
        rc.surface.ext.get_physical_device_surface_present_modes(
            rc.physical_device,
            rc.surface.raw,
        )?
    };

    if let Some(preferred_mode) = present_modes
        .iter()
        .find(|&&mode| mode == vk::PresentModeKHR::MAILBOX)
    {
        return Ok(*preferred_mode);
    }

    // FIFO is guaranteed to be available
    Ok(vk::PresentModeKHR::FIFO)
}

fn select_surface_format(rc: &RenderContext) -> Result<vk::SurfaceFormatKHR> {
    let surface_formats = unsafe {
        rc.surface.ext.get_physical_device_surface_formats(
            rc.physical_device,
            rc.surface.raw,
        )?
    };

    let preferred = surface_formats.iter().find(|surface_format| {
        let has_color_space =
            surface_format.color_space == vk::ColorSpaceKHR::SRGB_NONLINEAR;
        let has_format = surface_format.format == vk::Format::B8G8R8A8_SRGB;
        has_color_space && has_format
    });

    let format = preferred.or(surface_formats.first()).context(
        "Unable to find a suitable surface format for the swapchain!",
    )?;

    Ok(*format)
}
