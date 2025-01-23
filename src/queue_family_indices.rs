use anyhow::Ok;
use anyhow::{anyhow, Result};

    
    use vulkanalia::vk;
    use vulkanalia::vk::InstanceV1_0;
    use vulkanalia::Instance;

    use crate::{appdata, devices::SuitabilityError};

    #[derive(Copy, Clone, Debug)]
    pub(crate) struct QueueFamilyIndices {
        pub(crate) graphics: u32,
    }

    impl QueueFamilyIndices {
        pub(crate) unsafe fn get(
            instance: &Instance,
            data: &appdata::AppData,
            physical_device: vk::PhysicalDevice,
        ) -> Result<Self> {
            let properties = instance
                .get_physical_device_queue_family_properties(physical_device);

            let graphics = properties
                .iter()
                .position(|p| p.queue_flags.contains(vk::QueueFlags::GRAPHICS))
                .map(|i| i as u32);

            if let Some(graphics) = graphics {
                Ok(Self { graphics })
            } else {
                Err(anyhow!(SuitabilityError("Missing required queue families.")))
            }
        }
    }