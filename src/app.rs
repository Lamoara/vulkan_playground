
use crate::appdata;
use crate::appdata::AppData;
use crate::devices;
use crate::instance::create_instance;
use crate::pipeline::create_pipeline;
use crate::pipeline::create_render_pass;
use crate::swapchain::create_swapchain;
use crate::swapchain::create_swapchain_image_views;

use super::VALIDATION_ENABLED;

use anyhow::Ok;
use vulkanalia::loader::LibloadingLoader;
use vulkanalia::vk::DeviceV1_0;
use vulkanalia::vk::ExtDebugUtilsExtension;
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
        create_swapchain_image_views(&device, &mut data)?;
        create_render_pass(&instance, &device, &mut data)?;
        create_pipeline(&device, &mut data)?;
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

        self.data.swapchain_image_views
            .iter()
            .for_each(|v| self.device.destroy_image_view(*v, None));
        self.device.destroy_swapchain_khr(self.data.swapchain, None);
        self.instance.destroy_surface_khr(self.data.surface, None);
        self.device.destroy_render_pass(self.data.render_pass, None);
        self.device.destroy_pipeline_layout(self.data.pipeline_layout, None);
        self.device.destroy_pipeline(self.data.pipeline, None);
        self.device.destroy_device(None);
        self.instance.destroy_instance(None);
    }
}

