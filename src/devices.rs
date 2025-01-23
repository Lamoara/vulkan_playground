use crate::appdata;
use crate::queue_family_indices;
use crate::VALIDATION_LAYER;

use super::PORTABILITY_MACOS_VERSION;

use super::VALIDATION_ENABLED;

use anyhow::Ok;
use anyhow::{anyhow, Result};
use log::info;
use log::warn;
use thiserror::Error;
use vulkanalia::vk;
use vulkanalia::vk::DeviceV1_0;
use vulkanalia::vk::HasBuilder;
use vulkanalia::vk::InstanceV1_0;
use vulkanalia::Device;
use vulkanalia::Entry;
use vulkanalia::Instance;

#[derive(Debug, Error)]
#[error("Missing {0}.")]
pub struct SuitabilityError(pub &'static str);

pub(crate) unsafe fn pick_physical_device(instance: &Instance, data: &mut appdata::AppData) -> Result<()> {
    for physical_device in instance.enumerate_physical_devices()? {
        let properties = instance.get_physical_device_properties(physical_device);

        if let Err(error) = check_physical_device(instance, data, physical_device) {
            warn!("Skipping physical device (`{}`): {}", properties.device_name, error);
        } else {
            info!("Selected physical device (`{}`).", properties.device_name);
            data.physical_device = physical_device;
            return Ok(());
        }
    }

    Err(anyhow!("Failed to find suitable physical device."))
}

pub(crate) unsafe fn check_physical_device(
    instance: &Instance,
    data: &appdata::AppData,
    physical_device: vk::PhysicalDevice,
) -> Result<()> {
    queue_family_indices::QueueFamilyIndices::get(instance, data, physical_device)?;
    Ok(())
}

pub(crate) unsafe fn create_logical_device(
    entry: &Entry,
    instance: &Instance,
    data: &mut appdata::AppData,
) -> Result<Device> 
{
    let indices = queue_family_indices::QueueFamilyIndices::get(instance, data, data.physical_device)?;

    let queue_priorities = &[1.0];
    let queue_info = vk::DeviceQueueCreateInfo::builder()
        .queue_family_index(indices.graphics)
        .queue_priorities(queue_priorities);

    let layers = if VALIDATION_ENABLED {
        vec![VALIDATION_LAYER.as_ptr()]
    } else {
        vec![]
    };

    let mut extensions = vec![];

    // Required by Vulkan SDK on macOS since 1.3.216.
    if cfg!(target_os = "macos") && entry.version()? >= PORTABILITY_MACOS_VERSION {
        extensions.push(vk::KHR_PORTABILITY_SUBSET_EXTENSION.name.as_ptr());
    }

    let features = vk::PhysicalDeviceFeatures::builder();

    let queue_infos = &[queue_info];
    let info = vk::DeviceCreateInfo::builder()
        .queue_create_infos(queue_infos)
        .enabled_layer_names(&layers)
        .enabled_extension_names(&extensions)
        .enabled_features(&features);

    let device = instance.create_device(data.physical_device, &info, None)?;

    data.graphics_queue = device.get_device_queue(indices.graphics, 0);

    Ok(device)
}