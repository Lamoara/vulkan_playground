use anyhow::Ok;
use anyhow::{anyhow, Result};
use log::info;
use vulkanalia::vk;
use vulkanalia::vk::EntryV1_0;
use vulkanalia::vk::HasBuilder;
use vulkanalia::Entry;
use vulkanalia::Instance;
use vulkanalia::window as vk_window;
use crate::appdata;

use super::debug_callback;

use super::VALIDATION_LAYER;

use std::collections::HashSet;

use super::PORTABILITY_MACOS_VERSION;

use super::VALIDATION_ENABLED;


use winit::window::Window;

pub unsafe fn create_instance(
    window: &Window,
    entry: &Entry,
    data: &mut appdata::AppData
) -> Result<Instance>
{
    let api_version = vk::ApplicationInfo::builder()
        .application_name(b"Vulkan Tutorial\0")
        .application_version(vk::make_version(1, 0, 0))
        .engine_name(b"No Engine\0")
        .engine_version(vk::make_version(1, 0, 0))
        .api_version(vk::make_version(1, 0, 0));
    let application_info = api_version;

    let mut extensions = vk_window::get_required_instance_extensions(window)
        .iter()
        .map(|e| e.as_ptr())
        .collect::<Vec<_>>();

    if VALIDATION_ENABLED 
    {
        extensions.push(vk::EXT_DEBUG_UTILS_EXTENSION.name.as_ptr());
    }


    // Required by Vulkan SDK on macOS since 1.3.216.
    let flags = if 
        cfg!(target_os = "macos") && 
        entry.version()? >= PORTABILITY_MACOS_VERSION
    {
        info!("Enabling extensions for macOS portability.");
        extensions.push(vk::KHR_GET_PHYSICAL_DEVICE_PROPERTIES2_EXTENSION.name.as_ptr());
        extensions.push(vk::KHR_PORTABILITY_ENUMERATION_EXTENSION.name.as_ptr());
        vk::InstanceCreateFlags::ENUMERATE_PORTABILITY_KHR
    } else {
        vk::InstanceCreateFlags::empty()
    };

    let available_layers = entry
        .enumerate_instance_layer_properties()?
        .iter()
        .map(|l| l.layer_name)
        .collect::<HashSet<_>>();

    if VALIDATION_ENABLED && !available_layers.contains(&VALIDATION_LAYER) 
    {
        return Err(anyhow!("Validation layer requested but not supported."));
    }

    let layers = if VALIDATION_ENABLED 
    {
        vec![VALIDATION_LAYER.as_ptr()]
    } else {
        Vec::new()
    };


    let mut info = vk::InstanceCreateInfo::builder()
        .application_info(&application_info)
        .enabled_layer_names(&layers)
        .enabled_extension_names(&extensions)
        .flags(flags);


    let mut debug_info = vk::DebugUtilsMessengerCreateInfoEXT::builder()
        .message_severity(vk::DebugUtilsMessageSeverityFlagsEXT::all())
        .message_type(
            vk::DebugUtilsMessageTypeFlagsEXT::GENERAL
                | vk::DebugUtilsMessageTypeFlagsEXT::VALIDATION
                | vk::DebugUtilsMessageTypeFlagsEXT::PERFORMANCE,
        )
        .user_callback(Some(debug_callback));

    if VALIDATION_ENABLED 
    {
        info = info.push_next(&mut debug_info);
    }


    let instance = entry.create_instance(&info, None)?;

    Ok(instance)
}