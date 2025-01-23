
use crate::appdata;
use crate::appdata::AppData;
use crate::devices;
use crate::instance::create_instance;
use crate::queue_family_indices::QueueFamilyIndices;
use crate::swapchain_support::get_swapchain_extent;
use crate::swapchain_support::get_swapchain_present_mode;
use crate::swapchain_support::get_swapchain_surface_format;
use crate::swapchain_support::SwapchainSupport;

use super::VALIDATION_ENABLED;

use anyhow::Ok;
use vulkanalia::loader::LibloadingLoader;
use vulkanalia::vk;
use vulkanalia::vk::DeviceV1_0;
use vulkanalia::vk::ExtDebugUtilsExtension;
use vulkanalia::vk::Handle;
use vulkanalia::vk::HasBuilder;
use vulkanalia::vk::InstanceV1_0;
use vulkanalia::vk::KhrSurfaceExtension;
use vulkanalia::vk::KhrSwapchainExtension;
use vulkanalia::window as vk_window;
use vulkanalia::Device;
use vulkanalia::Entry;
use vulkanalia::Instance;
use anyhow::{anyhow, Result};


use vulkanalia::loader::LIBRARY;
use winit::window::Window;

/// Our Vulkan app.
#[derive(Clone, Debug)]
pub struct App {
    pub entry: Entry,
    pub instance: Instance,
    pub data: AppData,
    pub device: Device,
}

impl App 
{
    /// Creates our Vulkan app.
    pub unsafe fn create(window: &Window) -> Result<Self> 
    {
        let loader = LibloadingLoader::new(LIBRARY)?;
        let entry = Entry::new(loader).map_err(|b| anyhow!("{}", b))?;
        let mut data = appdata::AppData::default();
        let instance = create_instance(window, &entry, &mut data)?;
        data.surface = vk_window::create_surface(&instance, &window, &window)?;
        devices::pick_physical_device(&instance, &mut data)?;
        let device = devices::create_logical_device(&entry, &instance, &mut data)?;
        create_swapchain(window, &instance, &device, &mut data)?;
        Ok(Self { entry, instance, data, device })
    }


    /// Renders a frame for our Vulkan app.
    pub unsafe fn render(&mut self, window: &Window) -> Result<()> {
        Ok(())
    }

    /// Destroys our Vulkan app.
    pub unsafe fn destroy(&mut self) {
        if VALIDATION_ENABLED 
        {
            self.instance.destroy_debug_utils_messenger_ext(self.data.messenger, None);
        }
        
        self.device.destroy_swapchain_khr(self.data.swapchain, None);
        self.instance.destroy_surface_khr(self.data.surface, None);
        self.device.destroy_device(None);
        self.instance.destroy_instance(None);
    }
}

unsafe fn create_swapchain(
    window: &Window,
    instance: &Instance,
    device: &Device,
    data: &mut AppData,
) -> Result<()> {
    let indices = QueueFamilyIndices::get(instance, data, data.physical_device)?;
    let support = SwapchainSupport::get(instance, data, data.physical_device)?;

    let surface_format = get_swapchain_surface_format(&support.formats);
    let present_mode = get_swapchain_present_mode(&support.present_modes);
    let extent = get_swapchain_extent(window, support.capabilities);

    let mut image_count = support.capabilities.min_image_count + 1;
    if support.capabilities.max_image_count != 0
        && image_count > support.capabilities.max_image_count
    {
        image_count = support.capabilities.max_image_count;
    }

    let mut queue_family_indices = vec![];
    let image_sharing_mode = if indices.graphics != indices.present {
        queue_family_indices.push(indices.graphics);
        queue_family_indices.push(indices.present);
        vk::SharingMode::CONCURRENT
    } else {
        vk::SharingMode::EXCLUSIVE
    };

    let info = vk::SwapchainCreateInfoKHR::builder()
        .surface(data.surface)
        .min_image_count(image_count)
        .image_format(surface_format.format)
        .image_color_space(surface_format.color_space)
        .image_extent(extent)
        .image_array_layers(1)
        .image_usage(vk::ImageUsageFlags::COLOR_ATTACHMENT)
        .image_sharing_mode(image_sharing_mode)
        .queue_family_indices(&queue_family_indices)
        .pre_transform(support.capabilities.current_transform)
        .composite_alpha(vk::CompositeAlphaFlagsKHR::OPAQUE)
        .present_mode(present_mode)
        .clipped(true)
        .old_swapchain(vk::SwapchainKHR::null());

        data.swapchain = device.create_swapchain_khr(&info, None)?;
        data.swapchain_images = device.get_swapchain_images_khr(data.swapchain)?;
        data.swapchain_format = surface_format.format;
        data.swapchain_extent = extent;
        
    Ok(())
}