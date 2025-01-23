
use crate::appdata;
use crate::devices;
use crate::instance::create_instance;

use super::VALIDATION_ENABLED;

use anyhow::Ok;
use vulkanalia::loader::LibloadingLoader;
use vulkanalia::vk::DeviceV1_0;
use vulkanalia::vk::ExtDebugUtilsExtension;
use vulkanalia::vk::InstanceV1_0;
use vulkanalia::Device;
use vulkanalia::Entry;
use vulkanalia::Instance;
use anyhow::{anyhow, Result};


use vulkanalia::loader::LIBRARY;
use winit::window::Window;

/// Our Vulkan app.
#[derive(Clone, Debug)]
pub(crate) struct App {
    pub(crate) entry: Entry,
    pub(crate) instance: Instance,
    pub(crate) data: appdata::AppData,
    pub(crate) device: Device,
}

impl App 
{
    /// Creates our Vulkan app.
    pub(crate) unsafe fn create(window: &Window) -> Result<Self> 
    {
        let loader = LibloadingLoader::new(LIBRARY)?;
        let entry = Entry::new(loader).map_err(|b| anyhow!("{}", b))?;
        let mut data = appdata::AppData::default();
        let instance = create_instance(window, &entry, &mut data)?;
        devices::pick_physical_device(&instance, &mut data)?;
        let device = devices::create_logical_device(&entry, &instance, &mut data)?;
        Ok(Self { entry, instance, data, device })
    }


    /// Renders a frame for our Vulkan app.
    pub(crate) unsafe fn render(&mut self, window: &Window) -> Result<()> {
        Ok(())
    }

    /// Destroys our Vulkan app.
    pub(crate) unsafe fn destroy(&mut self) {
        self.device.destroy_device(None);
        if VALIDATION_ENABLED 
        {
            self.instance.destroy_debug_utils_messenger_ext(self.data.messenger, None);
        }
    
            self.instance.destroy_instance(None);
    }
}