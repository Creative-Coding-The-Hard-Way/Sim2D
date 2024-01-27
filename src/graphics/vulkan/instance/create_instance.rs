use {
    anyhow::{Context, Result},
    ash::vk,
};

/// Create an Ash Instance.
pub(super) unsafe fn create_instance<S>(
    entry: &ash::Entry,
    app_name: S,
    extensions: &[String],
) -> Result<ash::Instance>
where
    S: AsRef<str>,
{
    let app_name_ffi = std::ffi::CString::new(app_name.as_ref())?;
    let engine_name = std::ffi::CString::new("Sim2d")?;
    let app_info = vk::ApplicationInfo {
        p_application_name: app_name_ffi.as_ptr(),
        application_version: vk::make_api_version(0, 1, 0, 0),
        p_engine_name: engine_name.as_ptr(),
        engine_version: vk::make_api_version(0, 1, 0, 0),
        api_version: vk::make_api_version(0, 1, 3, 0),
        ..Default::default()
    };
    let (_extension_cstrs, extension_str_ptrs) =
        crate::graphics::vulkan::ffi::to_os_ptrs(&extensions);
    let create_info = vk::InstanceCreateInfo {
        p_application_info: &app_info,
        enabled_layer_count: 0,
        pp_enabled_layer_names: std::ptr::null(),
        enabled_extension_count: extension_str_ptrs.len() as u32,
        pp_enabled_extension_names: extension_str_ptrs.as_ptr(),
        ..Default::default()
    };
    entry
        .create_instance(&create_info, None)
        .context("Unable to create the Vulkan instance.")
}
